mod builder_features {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn transaction_builder_should_be_displayable() {
    #[cfg(not(feature = "sqlite"))]
    {
      let tr = sql::Transaction::new().start_transaction("").commit("");

      println!("{}", tr);

      let query = tr.as_string();
      let expected_query = "START TRANSACTION; COMMIT;";

      assert_eq!(query, expected_query);
    }

    #[cfg(feature = "sqlite")]
    {
      let tr = sql::Transaction::new().begin("").commit("");

      println!("{}", tr);

      let query = tr.as_string();
      let expected_query = "BEGIN; COMMIT;";

      assert_eq!(query, expected_query);
    }
  }

  #[test]
  fn transaction_builder_should_be_debuggable() {
    #[cfg(not(feature = "sqlite"))]
    {
      let tr = sql::Transaction::new().start_transaction("").commit("TRANSACTION");

      println!("{:?}", tr);

      let expected_query = "START TRANSACTION; COMMIT TRANSACTION;";
      let query = tr.as_string();

      assert_eq!(query, expected_query);
    }

    #[cfg(feature = "sqlite")]
    {
      let tr = sql::Transaction::new().begin("").commit("TRANSACTION");

      println!("{:?}", tr);

      let expected_query = "BEGIN; COMMIT TRANSACTION;";
      let query = tr.as_string();

      assert_eq!(query, expected_query);
    }
  }

  #[test]
  fn transaction_builder_should_be_able_to_conditionally_add_clauses() {
    #[cfg(not(feature = "sqlite"))]
    {
      let mut tr = sql::Transaction::new().start_transaction("");

      if true {
        tr = tr.commit("WORK");
      }

      let query = tr.as_string();
      let expected_query = "START TRANSACTION; COMMIT WORK;";

      assert_eq!(query, expected_query);
    }

    #[cfg(feature = "sqlite")]
    {
      let mut tr = sql::Transaction::new().begin("");

      if true {
        tr = tr.commit("");
      }

      let query = tr.as_string();
      let expected_query = "BEGIN; COMMIT;";

      assert_eq!(query, expected_query);
    }
  }

  #[test]
  fn transaction_builder_should_be_composable() {
    #[cfg(not(feature = "sqlite"))]
    {
      fn start_transaction(tr: sql::Transaction) -> sql::Transaction {
        tr.start_transaction("")
          .set_transaction("isolation level read committed")
      }

      fn commit(tr: sql::Transaction) -> sql::Transaction {
        tr.commit("")
      }

      fn as_string(tr: sql::Transaction) -> String {
        tr.as_string()
      }

      let query = Some(sql::Transaction::new())
        .map(start_transaction)
        .map(commit)
        .map(as_string)
        .unwrap();

      let expected_query = "\
        START TRANSACTION; \
        SET TRANSACTION isolation level read committed; \
        COMMIT;\
      ";

      assert_eq!(query, expected_query);
    }

    #[cfg(feature = "sqlite")]
    {
      fn begin(tr: sql::Transaction) -> sql::Transaction {
        tr.begin("")
      }

      fn commit(tr: sql::Transaction) -> sql::Transaction {
        tr.commit("")
      }

      fn as_string(tr: sql::Transaction) -> String {
        tr.as_string()
      }

      let query = Some(sql::Transaction::new())
        .map(begin)
        .map(commit)
        .map(as_string)
        .unwrap();

      let expected_query = "\
        BEGIN; \
        COMMIT;\
      ";

      assert_eq!(query, expected_query);
    }
  }
}

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
      let query = sql::Transaction::new()
        .start_transaction("")
        .set_transaction("read only")
        .savepoint("foo")
        .release_savepoint("foo")
        .rollback("transaction")
        .commit("transaction")
        .debug()
        .as_string();

      let expected_query = "\
        START TRANSACTION; \
        SET TRANSACTION read only; \
        SAVEPOINT foo; \
        RELEASE SAVEPOINT foo; \
        ROLLBACK transaction; \
        COMMIT transaction;\
      ";

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

mod alter_table_method {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_alter_table_should_add_a_alter_table_command() {
    let query = sql::Transaction::new()
      .alter_table(sql::AlterTable::new().alter_table("users"))
      .as_string();

    let expected_query = "ALTER TABLE users;";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_alter_table_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Transaction::new()
      .alter_table(sql::AlterTable::new().alter_table("users"))
      .alter_table(sql::AlterTable::new().alter_table("orders"))
      .as_string();

    let expected_query = "ALTER TABLE users; ALTER TABLE orders;";

    assert_eq!(expected_query, query);
  }
}

mod create_table_method {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_create_table_should_add_a_create_table_command() {
    let query = sql::Transaction::new()
      .create_table(sql::CreateTable::new().create_table("users"))
      .as_string();

    let expected_query = "CREATE TABLE users;";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_create_table_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Transaction::new()
      .create_table(sql::CreateTable::new().create_table("users"))
      .create_table(sql::CreateTable::new().create_table("orders"))
      .as_string();

    let expected_query = "CREATE TABLE users; CREATE TABLE orders;";

    assert_eq!(expected_query, query);
  }
}

mod delete_method {
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

mod insert_method {
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

mod select_method {
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

mod update_method {
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
