mod builder_methods {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_new_should_initialize_as_empty_string() {
    let query = sql::Transaction::new().as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_as_string_should_convert_the_current_state_into_string() {
    let query = sql::Transaction::new().as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_debug_should_print_at_console_in_a_human_readable_format() {
    #[cfg(not(feature = "sqlite"))]
    {
      let query = sql::Transaction::new().start_transaction("").debug().as_string();
      let expected_query = "START TRANSACTION;";

      assert_eq!(query, expected_query);
    }

    #[cfg(feature = "sqlite")]
    {
      let query = sql::Transaction::new().begin("").debug().as_string();
      let expected_query = "BEGIN;";

      assert_eq!(query, expected_query);
    }
  }

  #[test]
  fn method_print_should_print_in_one_line_the_current_state_of_builder() {
    #[cfg(not(feature = "sqlite"))]
    {
      let query = sql::Transaction::new()
        .start_transaction("isolation level serializable")
        .print()
        .as_string();
      let expected_query = "START TRANSACTION isolation level serializable;";

      assert_eq!(query, expected_query);
    }

    #[cfg(feature = "sqlite")]
    {
      let query = sql::Transaction::new().begin("EXCLUSIVE").print().as_string();
      let expected_query = "BEGIN EXCLUSIVE;";

      assert_eq!(query, expected_query);
    }
  }

  #[test]
  fn method_raw_should_add_raw_sql_on_top_of_the_command() {
    #[cfg(not(feature = "sqlite"))]
    {
      let query = sql::Transaction::new()
        .raw("/* the transaction command */")
        .start_transaction("")
        .as_string();
      let expected_query = "/* the transaction command */ START TRANSACTION;";

      assert_eq!(query, expected_query);
    }

    #[cfg(feature = "sqlite")]
    {
      let query = sql::Transaction::new()
        .raw("/* the transaction command */")
        .begin("IMMEDIATE")
        .as_string();
      let expected_query = "/* the transaction command */ BEGIN IMMEDIATE;";

      assert_eq!(query, expected_query);
    }
  }

  #[test]
  fn method_raw_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Transaction::new()
      .raw("/* raw one */")
      .raw("/* raw two */")
      .as_string();
    let expected_query = "/* raw one */ /* raw two */";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_be_the_first_to_be_concatenated() {
    #[cfg(not(feature = "sqlite"))]
    {
      let query = sql::Transaction::new()
        .start_transaction("")
        .raw("/* the first */")
        .as_string();
      let expected_query = "/* the first */ START TRANSACTION;";

      assert_eq!(query, expected_query);
    }
    #[cfg(feature = "sqlite")]
    {
      let query = sql::Transaction::new().begin("").raw("/* the first */").as_string();
      let expected_query = "/* the first */ BEGIN;";

      assert_eq!(query, expected_query);
    }
  }

  #[test]
  fn method_raw_should_trim_space_of_the_argument() {
    let query = sql::Transaction::new().raw("  /* raw one */  ").as_string();
    let expected_query = "/* raw one */";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Transaction::new()
      .raw("/* should not be repeat */")
      .raw("/* should not be repeat */")
      .as_string();
    let expected_query = "/* should not be repeat */";

    assert_eq!(query, expected_query);
  }
}

mod multi_commands {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn should_build_multi_commands() {
    #[cfg(not(feature = "sqlite"))]
    {
      let insert_foo = sql::Insert::new()
        .insert_into("users (login, name)")
        .values("('foo', 'Foo')");

      let insert_bar = sql::Insert::new()
        .insert_into("users (login, name)")
        .values("('bar', 'Bar')");

      let update_foo = sql::Update::new()
        .update("users")
        .set("name = 'Foooo'")
        .where_clause("login = 'foo'");

      let query = sql::Transaction::new()
        .start_transaction("")
        .set_transaction("READ ONLY")
        .insert(insert_foo)
        .savepoint("saved_foo")
        .insert(insert_bar)
        .update(update_foo)
        .savepoint("saved_bar_updated_foo")
        .release_savepoint("saved_foo")
        .commit("")
        .as_string();

      let expected_query = "\
        START TRANSACTION; \
        SET TRANSACTION READ ONLY; \
        INSERT INTO users (login, name) VALUES ('foo', 'Foo'); \
        SAVEPOINT saved_foo; \
        INSERT INTO users (login, name) VALUES ('bar', 'Bar'); \
        UPDATE users SET name = 'Foooo' WHERE login = 'foo'; \
        SAVEPOINT saved_bar_updated_foo; \
        RELEASE SAVEPOINT saved_foo; \
        COMMIT;\
      ";

      assert_eq!(query, expected_query);
    }

    #[cfg(feature = "sqlite")]
    {
      let insert_foo = sql::Insert::new()
        .insert_into("users (login, name)")
        .values("('foo', 'Foo')");

      let insert_bar = sql::Insert::new()
        .insert_into("users (login, name)")
        .values("('bar', 'Bar')");

      let update_foo = sql::Update::new()
        .update("users")
        .set("name = 'Foooo'")
        .where_clause("login = 'foo'");

      let query = sql::Transaction::new()
        .begin("")
        .insert(insert_foo)
        .savepoint("saved_foo")
        .insert(insert_bar)
        .update(update_foo)
        .savepoint("saved_bar_updated_foo")
        .release_savepoint("saved_foo")
        .end("")
        .as_string();

      let expected_query = "\
        BEGIN; \
        INSERT INTO users (login, name) VALUES ('foo', 'Foo'); \
        SAVEPOINT saved_foo; \
        INSERT INTO users (login, name) VALUES ('bar', 'Bar'); \
        UPDATE users SET name = 'Foooo' WHERE login = 'foo'; \
        SAVEPOINT saved_bar_updated_foo; \
        RELEASE SAVEPOINT saved_foo; \
        END;\
      ";

      assert_eq!(query, expected_query);
    }
  }
}

#[cfg(not(feature = "sqlite"))]
mod order_of_commands {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn command_set_transaction_should_be_add_after_start_transaction() {
    let query = sql::Transaction::new()
      .set_transaction("READ ONLY")
      .start_transaction("")
      .as_string();
    let expected_query = "\
      START TRANSACTION; \
      SET TRANSACTION READ ONLY;\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn command_commit_should_be_add_after_start_transaction_when_specified() {
    let query = sql::Transaction::new()
      .commit("")
      .start_transaction("REPEATABLE READ")
      .as_string();
    let expected_query = "\
      START TRANSACTION REPEATABLE READ; \
      COMMIT;\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn command_commit_should_be_add_after_set_transaction_when_specified() {
    let query = sql::Transaction::new()
      .commit("")
      .set_transaction("READ ONLY")
      .as_string();
    let expected_query = "\
      SET TRANSACTION READ ONLY; \
      COMMIT;\
    ";

    assert_eq!(query, expected_query);
  }
}

#[cfg(feature = "sqlite")]
mod order_of_commands {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn command_commit_should_be_add_after_begin_when_specified() {
    let query = sql::Transaction::new().commit("").begin("DEFERRED").as_string();
    let expected_query = "\
      BEGIN DEFERRED; \
      COMMIT;\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn command_end_should_be_add_after_begin_when_specified() {
    let query = sql::Transaction::new().end("").begin("IMMEDIATE").as_string();
    let expected_query = "\
      BEGIN IMMEDIATE; \
      END;\
    ";

    assert_eq!(query, expected_query);
  }
}

mod commit_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_commit_should_add_a_commit_command() {
    let query = sql::Transaction::new().commit("").as_string();
    let expected_query = "COMMIT;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_commit_should_add_the_transaction_mode_argument() {
    let query = sql::Transaction::new().commit("TRANSACTION").as_string();
    let expected_query = "COMMIT TRANSACTION;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_commit_should_trim_space_of_the_argument() {
    let query = sql::Transaction::new().commit("  TRANSACTION  ").as_string();
    let expected_query = "COMMIT TRANSACTION;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_commit_should_override_the_previews_value_on_consecutive_calls() {
    let query = sql::Transaction::new().commit("TRANSACTION").commit("WORK").as_string();
    let expected_query = "COMMIT WORK;";

    assert_eq!(query, expected_query);
  }
}

mod delete_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_delete_should_add_a_delete_command() {
    let query = sql::Transaction::new()
      .delete(sql::Delete::new().delete_from("users"))
      .as_string();
    let expected_query = "DELETE FROM users;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_delete_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Transaction::new()
      .delete(sql::Delete::new().delete_from("users"))
      .delete(sql::Delete::new().delete_from("users"))
      .as_string();
    let expected_query = "DELETE FROM users; DELETE FROM users;";

    assert_eq!(query, expected_query);
  }
}

mod insert_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_insert_should_add_a_insert_command() {
    let query = sql::Transaction::new()
      .insert(sql::Insert::new().insert_into("users (login, name)"))
      .as_string();
    let expected_query = "INSERT INTO users (login, name);";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_insert_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Transaction::new()
      .insert(sql::Insert::new().insert_into("users (login, name)"))
      .insert(sql::Insert::new().insert_into("users (login, name)"))
      .as_string();
    let expected_query = "INSERT INTO users (login, name); INSERT INTO users (login, name);";

    assert_eq!(query, expected_query);
  }
}

mod release_savepoint_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_release_savepoint_should_add_a_release_savepoint_command() {
    let query = sql::Transaction::new().release_savepoint("foo").as_string();
    let expected_query = "RELEASE SAVEPOINT foo;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_release_savepoint_should_trim_space_of_the_argument() {
    let query = sql::Transaction::new().release_savepoint("  bar  ").as_string();
    let expected_query = "RELEASE SAVEPOINT bar;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_release_savepoint_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Transaction::new()
      .release_savepoint("foo")
      .release_savepoint("bar")
      .as_string();
    let expected_query = "RELEASE SAVEPOINT foo; RELEASE SAVEPOINT bar;";

    assert_eq!(query, expected_query);
  }
}

mod rollback_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_rollback_should_add_a_rollback_command() {
    let query = sql::Transaction::new().rollback("TRANSACTION").as_string();
    let expected_query = "ROLLBACK TRANSACTION;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_rollback_should_trim_space_of_the_argument() {
    let query = sql::Transaction::new().rollback("  WORK  ").as_string();
    let expected_query = "ROLLBACK WORK;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_rollback_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Transaction::new()
      .rollback("TRANSACTION")
      .rollback("TO SAVEPOINT foo")
      .as_string();
    let expected_query = "ROLLBACK TRANSACTION; ROLLBACK TO SAVEPOINT foo;";

    assert_eq!(query, expected_query);
  }
}

mod savepoint_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_savepoint_should_add_a_savepoint_command() {
    let query = sql::Transaction::new().savepoint("foo").as_string();
    let expected_query = "SAVEPOINT foo;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_savepoint_should_trim_space_of_the_argument() {
    let query = sql::Transaction::new().savepoint("  bar  ").as_string();
    let expected_query = "SAVEPOINT bar;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_savepoint_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Transaction::new().savepoint("foo").savepoint("bar").as_string();
    let expected_query = "SAVEPOINT foo; SAVEPOINT bar;";

    assert_eq!(query, expected_query);
  }
}

mod select_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_select_should_add_a_select_command() {
    let query = sql::Transaction::new()
      .select(sql::Select::new().select("login, name"))
      .as_string();
    let expected_query = "SELECT login, name;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_select_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Transaction::new()
      .select(sql::Select::new().select("login, name"))
      .select(sql::Select::new().select("login, name"))
      .as_string();
    let expected_query = "SELECT login, name; SELECT login, name;";

    assert_eq!(query, expected_query);
  }
}

#[cfg(not(feature = "sqlite"))]
mod set_transaction_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_set_transaction_should_add_a_set_transaction_command() {
    let query = sql::Transaction::new().set_transaction("").as_string();
    let expected_query = "SET TRANSACTION;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_set_transaction_should_add_the_transaction_mode_argument() {
    let query = sql::Transaction::new().set_transaction("READ WRITE").as_string();
    let expected_query = "SET TRANSACTION READ WRITE;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_set_transaction_should_trim_space_of_the_argument() {
    let query = sql::Transaction::new().set_transaction("  READ WRITE  ").as_string();
    let expected_query = "SET TRANSACTION READ WRITE;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_set_transaction_should_override_the_previews_value_on_consecutive_calls() {
    let query = sql::Transaction::new()
      .set_transaction("ISOLATION LEVEL SERIALIZABLE")
      .set_transaction("ISOLATION LEVEL REPEATABLE READ")
      .as_string();
    let expected_query = "SET TRANSACTION ISOLATION LEVEL REPEATABLE READ;";

    assert_eq!(query, expected_query);
  }
}

#[cfg(not(feature = "sqlite"))]
mod start_transaction_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_start_transaction_should_add_a_start_transaction_command() {
    let query = sql::Transaction::new().start_transaction("").as_string();
    let expected_query = "START TRANSACTION;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_start_transaction_should_add_the_transaction_mode_argument() {
    let query = sql::Transaction::new().start_transaction("READ WRITE").as_string();
    let expected_query = "START TRANSACTION READ WRITE;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_start_transaction_should_trim_space_of_the_argument() {
    let query = sql::Transaction::new().start_transaction("  READ WRITE  ").as_string();
    let expected_query = "START TRANSACTION READ WRITE;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_start_transaction_should_override_the_previews_value_on_consecutive_calls() {
    let query = sql::Transaction::new()
      .start_transaction("ISOLATION LEVEL SERIALIZABLE")
      .start_transaction("ISOLATION LEVEL REPEATABLE READ")
      .as_string();
    let expected_query = "START TRANSACTION ISOLATION LEVEL REPEATABLE READ;";

    assert_eq!(query, expected_query);
  }
}

mod update_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_update_should_add_a_update_command() {
    let query = sql::Transaction::new()
      .update(sql::Update::new().update("users"))
      .as_string();
    let expected_query = "UPDATE users;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_update_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Transaction::new()
      .update(sql::Update::new().update("users"))
      .update(sql::Update::new().update("users"))
      .as_string();
    let expected_query = "UPDATE users; UPDATE users;";

    assert_eq!(query, expected_query);
  }
}
