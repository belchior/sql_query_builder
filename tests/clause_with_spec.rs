#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod delete_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_with_should_accept_delete_builder_as_query_argument() {
    let query = sql::Delete::new()
      .with("deleted_addresses", sql::Delete::new().delete_from("addresses"))
      .delete_from("orders")
      .as_string();
    let expected_query = "\
      WITH deleted_addresses AS (DELETE FROM addresses) \
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
  fn method_with_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::Delete::new()
      .with("deleted_products", sql::Delete::new())
      .with("deleted_users", sql::Delete::new().delete_from("users"))
      .with("deleted_orders", sql::Delete::new())
      .as_string();
    let expected_query = "WITH deleted_users AS (DELETE FROM users)";

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
      .with("deleted_addresses", sql::Delete::new().delete_from("addresses"))
      .raw_after(sql::DeleteClause::With, "select name, login")
      .as_string();
    let expected_query = "\
      WITH deleted_addresses AS (DELETE FROM addresses) \
      select name, login\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_delete_from_should_be_after_with_clause() {
    let query = sql::Delete::new()
      .with("deleted_addresses", sql::Delete::new().delete_from("addresses"))
      .delete_from("orders")
      .as_string();
    let expected_query = "\
      WITH deleted_addresses AS (DELETE FROM addresses) \
      DELETE FROM orders\
    ";

    assert_eq!(query, expected_query);
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod insert_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_with_should_accept_insert_builder_as_query_argument() {
    let query = sql::Insert::new()
      .with("addresses", sql::Insert::new().insert_into("addresses"))
      .as_string();
    let expected_query = "\
      WITH addresses AS (INSERT INTO addresses)\
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
  fn method_with_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::Insert::new()
      .with("inserted_users", sql::Insert::new())
      .with("inserted_orders", sql::Insert::new().insert_into("orders"))
      .with("inserted_products", sql::Insert::new())
      .as_string();
    let expected_query = "WITH inserted_orders AS (INSERT INTO orders)";

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
      .with("inserted_addresses", sql::Insert::new().insert_into("addresses"))
      .raw_after(sql::InsertClause::With, "select name, login")
      .as_string();
    let expected_query = "\
      WITH inserted_addresses AS (INSERT INTO addresses) \
      select name, login\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_insert_into_should_be_after_with_clause() {
    let query = sql::Insert::new()
      .with("inserted_addresses", sql::Insert::new().insert_into("addresses"))
      .insert_into("orders")
      .as_string();
    let expected_query = "\
      WITH inserted_addresses AS (INSERT INTO addresses) \
      INSERT INTO orders\
    ";

    assert_eq!(query, expected_query);
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
mod select_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_with_should_accept_select_builder_as_query_argument() {
    let query = sql::Select::new()
      .with("addresses", sql::Select::new().select("city"))
      .as_string();
    let expected_query = "\
      WITH addresses AS (SELECT city)\
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
  fn method_with_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::Select::new()
      .with("user_list", sql::Select::new())
      .with("user_ids", sql::Select::new().select("id").from("user_list"))
      .with("user_list2", sql::Select::new())
      .as_string();
    let expected_query = "WITH user_ids AS (SELECT id FROM user_list)";

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
      .with("addresses_list", sql::Select::new().select("*").from("addresses"))
      .raw_after(sql::SelectClause::With, "select name, login")
      .as_string();
    let expected_query = "WITH addresses_list AS (SELECT * FROM addresses) select name, login";

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

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod update_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_with_should_accept_update_builder_as_query_argument() {
    let query = sql::Update::new()
      .with("addresses", sql::Update::new().set("city = 'foo'"))
      .as_string();
    let expected_query = "\
      WITH addresses AS (SET city = 'foo')\
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
  fn method_with_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::Update::new()
      .with("updated_users", sql::Update::new())
      .with("updated_orders", sql::Update::new().update("orders"))
      .with("updated_users2", sql::Update::new())
      .as_string();
    let expected_query = "WITH updated_orders AS (UPDATE orders)";

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
      .with("updated_addresses", sql::Update::new().update("addresses"))
      .raw_after(sql::UpdateClause::With, "select name, login")
      .as_string();
    let expected_query = "\
      WITH updated_addresses AS (UPDATE addresses) \
      select name, login\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_update_should_be_after_with_clause() {
    let query = sql::Update::new()
      .with("updated_addresses", sql::Update::new().update("addresses"))
      .update("orders")
      .as_string();
    let expected_query = "\
      WITH updated_addresses AS (UPDATE addresses) \
      UPDATE orders\
    ";

    assert_eq!(query, expected_query);
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod values_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_with_should_accept_values_builder_as_query_argument() {
    let query = sql::Select::new()
      .with("addresses", sql::Values::new().values("('foo', 'Foo')"))
      .as_string();
    let expected_query = "\
      WITH addresses AS (VALUES ('foo', 'Foo'))\
    ";

    assert_eq!(query, expected_query);
  }
}
