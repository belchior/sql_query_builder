#[cfg(feature = "postgresql")]
mod from_clause {
  mod update_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_from_should_add_the_from_clause() {
      let query = sql::Update::new().from("users").as_string();
      let expected_query = "FROM users";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_from_should_accumulate_values_on_consecutive_calls() {
      let query = sql::Update::new().from("users").from("address").as_string();
      let expected_query = "FROM users, address";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_from_should_trim_space_of_the_argument() {
      let query = sql::Update::new().from("  users  ").as_string();
      let expected_query = "FROM users";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_from_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Update::new().from("address").from("address").as_string();
      let expected_query = "FROM address";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_from_should_be_after_set_clause() {
      let query = sql::Update::new().set("country = 'Bar'").from("address").as_string();
      let expected_query = "SET country = 'Bar' FROM address";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_from_clause() {
      let query = sql::Update::new()
        .raw_before(sql::UpdateClause::From, "set country = 'Bar'")
        .from("address")
        .as_string();
      let expected_query = "set country = 'Bar' FROM address";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_from_clause() {
      let query = sql::Update::new()
        .from("users")
        .raw_after(sql::UpdateClause::From, "where login = $1")
        .as_string();
      let expected_query = "FROM users where login = $1";

      assert_eq!(query, expected_query);
    }
  }
}

#[cfg(feature = "postgresql")]
mod limit_clause {
  mod select_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_limit_should_add_the_limit_clause() {
      let query = sql::Select::new().limit("3").as_string();
      let expected_query = "LIMIT 3";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_limit_should_override_the_current_value() {
      let query = sql::Select::new().limit("3").limit("4").as_string();
      let expected_query = "LIMIT 4";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_limit_should_trim_space_of_the_argument() {
      let query = sql::Select::new().limit("  50  ").as_string();
      let expected_query = "LIMIT 50";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_limit_should_be_after_order_by_clause() {
      let query = sql::Select::new().order_by("created_at desc").limit("42").as_string();
      let expected_query = "ORDER BY created_at desc LIMIT 42";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_limit_clause() {
      let query = sql::Select::new()
        .raw_before(sql::SelectClause::Limit, "group by id")
        .limit("10")
        .as_string();
      let expected_query = "group by id LIMIT 10";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_limit_clause() {
      let query = sql::Select::new()
        .limit("10")
        .raw_after(sql::SelectClause::Limit, "except select id, login")
        .as_string();
      let expected_query = "LIMIT 10 except select id, login";

      assert_eq!(query, expected_query);
    }
  }
}

#[cfg(feature = "postgresql")]
mod offset_clause {
  mod select_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_offset_should_add_the_offset_clause() {
      let query = sql::Select::new().offset("100").as_string();
      let expected_query = "OFFSET 100";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_offset_should_override_the_current_value() {
      let query = sql::Select::new().offset("100").offset("200").as_string();
      let expected_query = "OFFSET 200";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_offset_should_trim_space_of_the_argument() {
      let query = sql::Select::new().offset("  2000  ").as_string();
      let expected_query = "OFFSET 2000";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_offset_should_be_after_limit_clause() {
      let query = sql::Select::new().limit("500").offset("100").as_string();
      let expected_query = "LIMIT 500 OFFSET 100";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_offset_clause() {
      let query = sql::Select::new()
        .raw_before(sql::SelectClause::Limit, "limit 1000")
        .offset("50")
        .as_string();
      let expected_query = "limit 1000 OFFSET 50";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_offset_clause() {
      let query = sql::Select::new()
        .offset("10")
        .raw_after(sql::SelectClause::Offset, "/* the end */")
        .as_string();
      let expected_query = "OFFSET 10 /* the end */";

      assert_eq!(query, expected_query);
    }
  }
}

#[cfg(feature = "postgresql")]
mod returning_clause {
  mod delete_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_returning_should_add_the_returning_clause() {
      let query = sql::Delete::new().returning("*").as_string();
      let expected_query = "RETURNING *";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_returning_should_accumulate_values_on_consecutive_calls() {
      let query = sql::Delete::new().returning("login").returning("name").as_string();
      let expected_query = "RETURNING login, name";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_returning_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Delete::new().returning("id").returning("id").as_string();
      let expected_query = "RETURNING id";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_returning_should_trim_space_of_the_argument() {
      let query = sql::Delete::new().returning("  login  ").as_string();
      let expected_query = "RETURNING login";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_returning_should_be_after_where_clause() {
      let query = sql::Delete::new().returning("id").where_clause("name = $1").as_string();
      let expected_query = "WHERE name = $1 RETURNING id";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_returning_clause() {
      let query = sql::Delete::new()
        .raw_before(sql::DeleteClause::Returning, "delete from users")
        .returning("login")
        .as_string();
      let expected_query = "delete from users RETURNING login";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_returning_clause() {
      let query = sql::Delete::new()
        .returning("id")
        .raw_after(sql::DeleteClause::Returning, ", login, name")
        .as_string();
      let expected_query = "RETURNING id , login, name";

      assert_eq!(query, expected_query);
    }
  }

  mod insert_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_returning_should_add_the_returning_clause() {
      let query = sql::Insert::new().returning("*").as_string();
      let expected_query = "RETURNING *";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_returning_should_accumulate_values_on_consecutive_calls() {
      let query = sql::Insert::new().returning("login").returning("name").as_string();
      let expected_query = "RETURNING login, name";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_returning_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Insert::new().returning("id").returning("id").as_string();
      let expected_query = "RETURNING id";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_returning_should_trim_space_of_the_argument() {
      let query = sql::Insert::new().returning("  login  ").as_string();
      let expected_query = "RETURNING login";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_returning_should_be_after_values_clause() {
      let query = sql::Insert::new()
        .insert_into("(login, name)")
        .returning("login")
        .values("('foo', 'Foo')")
        .as_string();
      let expected_query = "INSERT INTO (login, name) VALUES ('foo', 'Foo') RETURNING login";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_returning_should_be_after_on_conflict_clause() {
      let query = sql::Insert::new()
        .insert_into("(login, name)")
        .values("('foo', 'Foo')")
        .on_conflict("do nothing")
        .returning("login")
        .as_string();
      let expected_query = "INSERT INTO (login, name) VALUES ('foo', 'Foo') ON CONFLICT do nothing RETURNING login";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_returning_clause() {
      let query = sql::Insert::new()
        .raw_before(sql::InsertClause::Returning, "values ('foo')")
        .returning("login")
        .as_string();
      let expected_query = "values ('foo') RETURNING login";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_returning_clause() {
      let query = sql::Insert::new()
        .returning("id")
        .raw_after(sql::InsertClause::Returning, ", login, name")
        .as_string();
      let expected_query = "RETURNING id , login, name";

      assert_eq!(query, expected_query);
    }
  }

  mod update_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_returning_should_add_the_returning_clause() {
      let query = sql::Update::new().returning("*").as_string();
      let expected_query = "RETURNING *";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_returning_should_accumulate_values_on_consecutive_calls() {
      let query = sql::Update::new().returning("login").returning("name").as_string();
      let expected_query = "RETURNING login, name";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_returning_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Update::new().returning("id").returning("id").as_string();
      let expected_query = "RETURNING id";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_returning_should_trim_space_of_the_argument() {
      let query = sql::Update::new().returning("  login  ").as_string();
      let expected_query = "RETURNING login";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_returning_should_be_after_where_clause() {
      let query = sql::Update::new().returning("id").where_clause("name = $1").as_string();
      let expected_query = "WHERE name = $1 RETURNING id";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_returning_clause() {
      let query = sql::Update::new()
        .raw_before(sql::UpdateClause::Returning, "where login = $1")
        .returning("login")
        .as_string();
      let expected_query = "where login = $1 RETURNING login";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_returning_clause() {
      let query = sql::Update::new()
        .returning("id")
        .raw_after(sql::UpdateClause::Returning, ", login, name")
        .as_string();
      let expected_query = "RETURNING id , login, name";

      assert_eq!(query, expected_query);
    }
  }
}

#[cfg(feature = "postgresql")]
mod with_clause {
  mod delete_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_with_should_accept_delete_builder_as_query_argument() {
      let query = sql::Delete::new()
        .with("deleted_address", sql::Delete::new().delete_from("address"))
        .delete_from("orders")
        .as_string();
      let expected_query = "\
        WITH deleted_address AS (DELETE FROM address) \
        DELETE FROM orders\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_add_the_with_clause() {
      let deleted_users = sql::Delete::new()
        .delete_from("users")
        .where_clause("ative = false")
        .returning("id");
      let query = sql::Delete::new().with("id_list", deleted_users).as_string();
      let expected_query = "WITH id_list AS (DELETE FROM users WHERE ative = false RETURNING id)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_accept_inline_argument() {
      let query = sql::Delete::new()
        .with(
          "id_list",
          sql::Delete::new()
            .delete_from("users")
            .where_clause("ative = false")
            .returning("id"),
        )
        .as_string();
      let expected_query = "WITH id_list AS (DELETE FROM users WHERE ative = false RETURNING id)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_accumulate_values_on_consecutive_calls() {
      let deleted_users = sql::Delete::new().delete_from("users");
      let deleted_orders = sql::Delete::new().delete_from("orders");
      let query = sql::Delete::new()
        .with("deleted_users", deleted_users)
        .with("deleted_orders", deleted_orders)
        .as_string();
      let expected_query = "\
        WITH deleted_users AS (DELETE FROM users), \
             deleted_orders AS (DELETE FROM orders)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_trim_space_of_the_argument() {
      let query = sql::Delete::new()
        .with("  deleted_users  ", sql::Delete::new().delete_from("users"))
        .as_string();
      let expected_query = "WITH deleted_users AS (DELETE FROM users)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_with_should_be_after_raw() {
      let query = sql::Delete::new()
        .raw("/* the with clause */")
        .with("deleted_users", sql::Delete::new().delete_from("users"))
        .as_string();
      let expected_query = "\
        /* the with clause */ \
        WITH deleted_users AS (DELETE FROM users)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_with_clause() {
      let query = sql::Delete::new()
        .raw_before(sql::DeleteClause::With, "/* the with clause */")
        .with("deleted_orders", sql::Delete::new().delete_from("orders"))
        .as_string();
      let expected_query = "\
        /* the with clause */ \
        WITH deleted_orders AS (DELETE FROM orders)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_with_clause() {
      let query = sql::Delete::new()
        .with("deleted_address", sql::Delete::new().delete_from("address"))
        .raw_after(sql::DeleteClause::With, "select name, login")
        .as_string();
      let expected_query = "\
        WITH deleted_address AS (DELETE FROM address) \
        select name, login\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_delete_from_should_be_after_with_clause() {
      let query = sql::Delete::new()
        .with("deleted_address", sql::Delete::new().delete_from("address"))
        .delete_from("orders")
        .as_string();
      let expected_query = "\
        WITH deleted_address AS (DELETE FROM address) \
        DELETE FROM orders\
      ";

      assert_eq!(query, expected_query);
    }
  }

  mod insert_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_with_should_accept_insert_builder_as_query_argument() {
      let query = sql::Insert::new()
        .with("address", sql::Insert::new().insert_into("address"))
        .as_string();
      let expected_query = "\
        WITH address AS (INSERT INTO address)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_add_the_with_clause() {
      let inserted_users = sql::Insert::new()
        .insert_into("users(login)")
        .values("('foo')")
        .returning("id");
      let query = sql::Insert::new().with("id_list", inserted_users).as_string();
      let expected_query = "WITH id_list AS (INSERT INTO users(login) VALUES ('foo') RETURNING id)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_accept_inline_argument() {
      let query = sql::Insert::new()
        .with(
          "id_list",
          sql::Insert::new()
            .insert_into("users(login)")
            .values("('foo')")
            .returning("id"),
        )
        .as_string();
      let expected_query = "WITH id_list AS (INSERT INTO users(login) VALUES ('foo') RETURNING id)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_accumulate_values_on_consecutive_calls() {
      let inserted_users = sql::Insert::new().insert_into("users");
      let inserted_orders = sql::Insert::new().insert_into("orders");
      let query = sql::Insert::new()
        .with("inserted_users", inserted_users)
        .with("inserted_orders", inserted_orders)
        .as_string();
      let expected_query = "\
        WITH inserted_users AS (INSERT INTO users), \
             inserted_orders AS (INSERT INTO orders)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_trim_space_of_the_argument() {
      let query = sql::Insert::new()
        .with("  inserted_users  ", sql::Insert::new().insert_into("users"))
        .as_string();
      let expected_query = "WITH inserted_users AS (INSERT INTO users)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_with_should_be_after_raw() {
      let query = sql::Insert::new()
        .raw("/* the with clause */")
        .with("inserted_users", sql::Insert::new().insert_into("users"))
        .as_string();
      let expected_query = "\
        /* the with clause */ \
        WITH inserted_users AS (INSERT INTO users)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_with_clause() {
      let query = sql::Insert::new()
        .raw_before(sql::InsertClause::With, "/* the with clause */")
        .with("inserted_orders", sql::Insert::new().insert_into("orders"))
        .as_string();
      let expected_query = "\
        /* the with clause */ \
        WITH inserted_orders AS (INSERT INTO orders)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_with_clause() {
      let query = sql::Insert::new()
        .with("inserted_address", sql::Insert::new().insert_into("address"))
        .raw_after(sql::InsertClause::With, "select name, login")
        .as_string();
      let expected_query = "\
        WITH inserted_address AS (INSERT INTO address) \
        select name, login\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_insert_into_should_be_after_with_clause() {
      let query = sql::Insert::new()
        .with("inserted_address", sql::Insert::new().insert_into("address"))
        .insert_into("orders")
        .as_string();
      let expected_query = "\
        WITH inserted_address AS (INSERT INTO address) \
        INSERT INTO orders\
      ";

      assert_eq!(query, expected_query);
    }
  }

  mod select_builder_with_clause {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_with_should_accept_select_builder_as_query_argument() {
      let query = sql::Select::new()
        .with("address", sql::Select::new().select("city"))
        .as_string();
      let expected_query = "\
        WITH address AS (SELECT city)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_add_the_with_clause() {
      let select_users = sql::Select::new().select("login").from("users");
      let query = sql::Select::new().with("user_list", select_users).as_string();
      let expected_query = "WITH user_list AS (SELECT login FROM users)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_accept_inline_argument() {
      let query = sql::Select::new()
        .with("user_list", sql::Select::new().select("login").from("users"))
        .as_string();
      let expected_query = "WITH user_list AS (SELECT login FROM users)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_accumulate_values_on_consecutive_calls() {
      let select_users = sql::Select::new().select("id, login").from("users");
      let select_users_id = sql::Select::new().select("id").from("user_list");
      let query = sql::Select::new()
        .with("user_list", select_users)
        .with("user_ids", select_users_id)
        .as_string();
      let expected_query = "\
      WITH user_list AS (SELECT id, login FROM users), user_ids AS (SELECT id FROM user_list)\
    ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_trim_space_of_the_argument() {
      let query = sql::Select::new()
        .with("  date  ", sql::Select::new().select("current_date"))
        .as_string();
      let expected_query = "WITH date AS (SELECT current_date)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_with_should_be_after_raw() {
      let select_base = sql::Select::new()
        .raw("select 123 as id union")
        .with("user_list", sql::Select::new().select("*").from("users"))
        .select("id");
      let query = select_base.as_string();
      let expected_query = "\
        select 123 as id union \
        WITH user_list AS (SELECT * FROM users) \
        SELECT id\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_with_clause() {
      let query = sql::Select::new()
        .raw_before(sql::SelectClause::With, "/* the users orders */")
        .with("orders_list", sql::Select::new().select("*").from("orders"))
        .as_string();
      let expected_query = "/* the users orders */ WITH orders_list AS (SELECT * FROM orders)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_with_clause() {
      let query = sql::Select::new()
        .with("address_list", sql::Select::new().select("*").from("address"))
        .raw_after(sql::SelectClause::With, "select name, login")
        .as_string();
      let expected_query = "WITH address_list AS (SELECT * FROM address) select name, login";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_select_should_be_after_with_clause() {
      let select_users = sql::Select::new().select("*").from("users");
      let select_base = sql::Select::new().with("user_list", select_users).select("id");
      let query = select_base.as_string();
      let expected_query = "\
        WITH user_list AS (SELECT * FROM users) \
        SELECT id\
      ";

      assert_eq!(query, expected_query);
    }
  }

  mod update_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_with_should_accept_update_builder_as_query_argument() {
      let query = sql::Update::new()
        .with("address", sql::Update::new().set("city = 'foo'"))
        .as_string();
      let expected_query = "\
        WITH address AS (SET city = 'foo')\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_add_the_with_clause() {
      let update_users = sql::Update::new()
        .update("users")
        .where_clause("ative = false")
        .returning("id");
      let query = sql::Update::new().with("id_list", update_users).as_string();
      let expected_query = "WITH id_list AS (UPDATE users WHERE ative = false RETURNING id)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_accept_inline_argument() {
      let query = sql::Update::new()
        .with(
          "id_list",
          sql::Update::new()
            .update("users")
            .where_clause("ative = false")
            .returning("id"),
        )
        .as_string();
      let expected_query = "WITH id_list AS (UPDATE users WHERE ative = false RETURNING id)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_accumulate_values_on_consecutive_calls() {
      let updated_users = sql::Update::new().update("users");
      let updated_orders = sql::Update::new().update("orders");
      let query = sql::Update::new()
        .with("updated_users", updated_users)
        .with("updated_orders", updated_orders)
        .as_string();
      let expected_query = "\
        WITH updated_users AS (UPDATE users), \
             updated_orders AS (UPDATE orders)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_trim_space_of_the_argument() {
      let query = sql::Update::new()
        .with("  updated_users  ", sql::Update::new().update("users"))
        .as_string();
      let expected_query = "WITH updated_users AS (UPDATE users)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_with_should_be_after_raw() {
      let query = sql::Update::new()
        .raw("/* the with clause */")
        .with("updated_users", sql::Update::new().update("users"))
        .as_string();
      let expected_query = "\
        /* the with clause */ \
        WITH updated_users AS (UPDATE users)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_with_clause() {
      let query = sql::Update::new()
        .raw_before(sql::UpdateClause::With, "/* the with clause */")
        .with("updated_orders", sql::Update::new().update("orders"))
        .as_string();
      let expected_query = "\
        /* the with clause */ \
        WITH updated_orders AS (UPDATE orders)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_with_clause() {
      let query = sql::Update::new()
        .with("updated_address", sql::Update::new().update("address"))
        .raw_after(sql::UpdateClause::With, "select name, login")
        .as_string();
      let expected_query = "\
        WITH updated_address AS (UPDATE address) \
        select name, login\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_update_should_be_after_with_clause() {
      let query = sql::Update::new()
        .with("updated_address", sql::Update::new().update("address"))
        .update("orders")
        .as_string();
      let expected_query = "\
        WITH updated_address AS (UPDATE address) \
        UPDATE orders\
      ";

      assert_eq!(query, expected_query);
    }
  }

  mod values_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_with_should_accept_values_builder_as_query_argument() {
      let query = sql::Select::new()
        .with("address", sql::Values::new().values("('foo', 'Foo')"))
        .as_string();
      let expected_query = "\
        WITH address AS (VALUES ('foo', 'Foo'))\
      ";

      assert_eq!(query, expected_query);
    }
  }
}

#[cfg(feature = "postgresql")]
mod except_clause {
  mod select_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_except_should_add_the_except_clause() {
      let select_users = sql::Select::new().select("login").from("users");
      let select_address = sql::Select::new().select("login").from("address");
      let query = select_users.except(select_address).as_string();
      let expected_query = "(SELECT login FROM users) EXCEPT (SELECT login FROM address)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_except_should_accept_inline_argument() {
      let select_users = sql::Select::new().select("login").from("users");
      let query = select_users
        .except(sql::Select::new().select("login").from("address"))
        .as_string();
      let expected_query = "(SELECT login FROM users) EXCEPT (SELECT login FROM address)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_except_should_accumulate_values_on_consecutive_calls() {
      let select_users = sql::Select::new().select("login").from("users");
      let select_address = sql::Select::new().select("login").from("address");
      let select_orders = sql::Select::new().select("login").from("orders");
      let query = select_users.except(select_address).except(select_orders).as_string();
      let expected_query = "\
        (SELECT login FROM users) \
        EXCEPT \
        (SELECT login FROM address) \
        EXCEPT \
        (SELECT login FROM orders)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_except_should_be_after_offset_clause() {
      let select_address = sql::Select::new().select("login").from("address");
      let query = sql::Select::new().offset("10").except(select_address).as_string();
      let expected_query = "\
        (OFFSET 10) \
        EXCEPT \
        (SELECT login FROM address)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_except_clause() {
      let query = sql::Select::new()
        .raw_before(sql::SelectClause::Except, "select name from orders")
        .except(sql::Select::new().select("name"))
        .as_string();
      let expected_query = "(select name from orders) EXCEPT (SELECT name)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_except_clause() {
      let query = sql::Select::new()
        .select("name")
        .except(sql::Select::new().select("name"))
        .raw_after(sql::SelectClause::Except, "/* the name */")
        .as_string();
      let expected_query = "(SELECT name) EXCEPT (SELECT name) /* the name */";

      assert_eq!(query, expected_query);
    }
  }
}

#[cfg(feature = "postgresql")]
mod intersect_clause {
  mod select_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_intersect_should_add_the_intersect_clause() {
      let select_users = sql::Select::new().select("login").from("users");
      let select_address = sql::Select::new().select("login").from("address");
      let query = select_users.intersect(select_address).as_string();
      let expected_query = "(SELECT login FROM users) INTERSECT (SELECT login FROM address)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_intersect_should_accept_inline_argument() {
      let select_users = sql::Select::new().select("login").from("users");
      let query = select_users
        .intersect(sql::Select::new().select("login").from("address"))
        .as_string();
      let expected_query = "(SELECT login FROM users) INTERSECT (SELECT login FROM address)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_intersect_should_accumulate_values_on_consecutive_calls() {
      let select_users = sql::Select::new().select("login").from("users");
      let select_address = sql::Select::new().select("login").from("address");
      let select_orders = sql::Select::new().select("login").from("orders");
      let query = select_users
        .intersect(select_address)
        .intersect(select_orders)
        .as_string();
      let expected_query = "\
        (SELECT login FROM users) \
        INTERSECT \
        (SELECT login FROM address) \
        INTERSECT \
        (SELECT login FROM orders)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_intersect_should_be_after_offset_clause() {
      let select_address = sql::Select::new().select("login").from("address");
      let query = sql::Select::new().offset("10").intersect(select_address).as_string();
      let expected_query = "\
        (OFFSET 10) \
        INTERSECT \
        (SELECT login FROM address)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_intersect_clause() {
      let query = sql::Select::new()
        .raw_before(sql::SelectClause::Except, "select name from orders")
        .intersect(sql::Select::new().select("name"))
        .as_string();
      let expected_query = "(select name from orders) INTERSECT (SELECT name)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_intersect_clause() {
      let query = sql::Select::new()
        .select("name")
        .intersect(sql::Select::new().select("name"))
        .raw_after(sql::SelectClause::Intersect, "/* the name */")
        .as_string();
      let expected_query = "(SELECT name) INTERSECT (SELECT name) /* the name */";

      assert_eq!(query, expected_query);
    }
  }
}

#[cfg(feature = "postgresql")]
mod union_clause {
  mod select_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_union_should_add_the_union_clause() {
      let select_users = sql::Select::new().select("login").from("users");
      let select_address = sql::Select::new().select("login").from("address");
      let query = select_users.union(select_address).as_string();
      let expected_query = "(SELECT login FROM users) UNION (SELECT login FROM address)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_union_should_accept_inline_argument() {
      let select_users = sql::Select::new().select("login").from("users");
      let query = select_users
        .union(sql::Select::new().select("login").from("address"))
        .as_string();
      let expected_query = "(SELECT login FROM users) UNION (SELECT login FROM address)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_union_should_accumulate_values_on_consecutive_calls() {
      let select_users = sql::Select::new().select("login").from("users");
      let select_address = sql::Select::new().select("login").from("address");
      let select_orders = sql::Select::new().select("login").from("orders");
      let query = select_users.union(select_address).union(select_orders).as_string();
      let expected_query = "\
        (SELECT login FROM users) \
        UNION \
        (SELECT login FROM address) \
        UNION \
        (SELECT login FROM orders)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_union_should_be_after_offset_clause() {
      let select_address = sql::Select::new().select("login").from("address");
      let query = sql::Select::new().offset("10").union(select_address).as_string();
      let expected_query = "\
        (OFFSET 10) \
        UNION \
        (SELECT login FROM address)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_union_clause() {
      let query = sql::Select::new()
        .raw_before(sql::SelectClause::Union, "select name from orders")
        .union(sql::Select::new().select("name"))
        .as_string();
      let expected_query = "(select name from orders) UNION (SELECT name)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_union_clause() {
      let query = sql::Select::new()
        .select("name")
        .union(sql::Select::new().select("name"))
        .raw_after(sql::SelectClause::Union, "/* the name */")
        .as_string();
      let expected_query = "(SELECT name) UNION (SELECT name) /* the name */";

      assert_eq!(query, expected_query);
    }
  }
}

#[cfg(feature = "postgresql")]
mod begin_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_begin_should_add_a_begin_command() {
    let query = sql::Transaction::new().begin("").as_string();
    let expected_query = "BEGIN;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_begin_should_add_the_transaction_mode_argument() {
    let query = sql::Transaction::new().begin("READ WRITE").as_string();
    let expected_query = "BEGIN READ WRITE;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_begin_should_trim_space_of_the_argument() {
    let query = sql::Transaction::new().begin("  READ WRITE  ").as_string();
    let expected_query = "BEGIN READ WRITE;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_begin_should_override_the_previews_value_on_consecutive_calls() {
    let query = sql::Transaction::new()
      .begin("ISOLATION LEVEL SERIALIZABLE")
      .begin("ISOLATION LEVEL REPEATABLE READ")
      .as_string();
    let expected_query = "BEGIN ISOLATION LEVEL REPEATABLE READ;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_begin_should_not_override_the_start_transaction_on_consecutive_calls() {
    let query = sql::Transaction::new().start_transaction("").begin("").as_string();
    let expected_query = "BEGIN; START TRANSACTION;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_begin_should_not_be_overrided_by_start_transaction_method_on_consecutive_calls() {
    let query = sql::Transaction::new().begin("").start_transaction("").as_string();
    let expected_query = "BEGIN; START TRANSACTION;";

    assert_eq!(query, expected_query);
  }
}
