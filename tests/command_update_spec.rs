mod full_api {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn sql_standard_with_all_methods() {
    let query = sql::Update::new()
      // required
      .update("orders")
      .set("name = 'Foo'")
      // optional
      .where_clause("login = $1")
      .where_and("product_id = $2")
      .where_or("ref_id = $3")
      .as_string();

    let expected_query = "\
      UPDATE orders \
      SET name = 'Foo' \
      WHERE login = $1 \
      AND product_id = $2 \
      OR ref_id = $3\
    ";

    assert_eq!(expected_query, query);
  }

  #[cfg(feature = "postgresql")]
  #[test]
  fn postgres_with_all_methods() {
    let query = sql::Update::new()
      // required
      .update("orders")
      .set("name = 'Foo'")
      // optional
      .with("foo", sql::Select::new().select("login"))
      .from("products p")
      .where_clause("login = $1")
      .where_and("product_id = $2")
      .where_or("p.ref_id = $3")
      .returning("*")
      .as_string();

    let expected_query = "\
      WITH foo AS (SELECT login) \
      UPDATE orders \
      SET name = 'Foo' \
      FROM products p \
      WHERE login = $1 \
      AND product_id = $2 \
      OR p.ref_id = $3 \
      RETURNING *\
    ";

    assert_eq!(expected_query, query);
  }

  #[cfg(feature = "sqlite")]
  #[test]
  fn sqlite_with_all_methods() {
    let query = sql::Update::new()
      // one of is required
      .update("orders")
      .update_or("users")
      // required
      .set("name = 'Foo'")
      // optional
      .with("foo", sql::Select::new().select("login"))
      .from("products p")
      .cross_join("addresses")
      .inner_join("addresses on addresses.user_login = users.login")
      .left_join("addresses on addresses.user_login = users.login")
      .right_join("addresses on addresses.user_login = users.login")
      .where_clause("login = $1")
      .where_and("product_id = $2")
      .where_or("p.ref_id = $3")
      .returning("*")
      .as_string();

    let expected_query = "\
      WITH foo AS (SELECT login) \
      UPDATE OR users \
      SET name = 'Foo' \
      FROM products p \
      CROSS JOIN addresses \
      INNER JOIN addresses on addresses.user_login = users.login \
      LEFT JOIN addresses on addresses.user_login = users.login \
      RIGHT JOIN addresses on addresses.user_login = users.login \
      WHERE login = $1 \
      AND product_id = $2 \
      OR p.ref_id = $3 \
      RETURNING *\
    ";

    assert_eq!(expected_query, query);
  }

  #[cfg(feature = "mysql")]
  #[test]
  fn mysql_with_all_methods() {
    let query = sql::Update::new()
      // required
      .update("orders")
      .set("name = 'Foo'")
      // optional
      .where_clause("login = $1")
      .where_and("product_id = $2")
      .where_or("ref_id = $3")
      .order_by("id desc")
      .limit("1")
      .as_string();

    let expected_query = "\
      UPDATE orders \
      SET name = 'Foo' \
      WHERE login = $1 \
      AND product_id = $2 \
      OR ref_id = $3 \
      ORDER BY id desc \
      LIMIT 1\
    ";

    assert_eq!(expected_query, query);
  }
}

mod builder_features {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn update_builder_should_be_displayable() {
    let update = sql::Update::new().update("users").set("login = 'foo'");

    println!("{}", update);

    let query = update.as_string();
    let expected_query = "UPDATE users SET login = 'foo'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn update_builder_should_be_debuggable() {
    let update = sql::Update::new()
      .update("users")
      .set("name = 'Foo'")
      .where_clause("login = 'foo'");

    println!("{:?}", update);

    let expected_query = "UPDATE users SET name = 'Foo' WHERE login = 'foo'";
    let query = update.as_string();

    assert_eq!(query, expected_query);
  }

  #[test]
  fn update_builder_should_be_cloneable() {
    let update_foo = sql::Update::new()
      .raw("/* test raw */")
      .raw_before(sql::UpdateClause::Set, "/* test raw_before */")
      .raw_after(sql::UpdateClause::Set, "/* test raw_after */")
      .update("users")
      .set("login = 'foo'");
    let update_foo_bar = update_foo.clone().set("name = 'Bar'");
    let query_foo = update_foo.as_string();
    let query_foo_bar = update_foo_bar.as_string();

    let expected_query_foo = "\
      /* test raw */ \
      UPDATE users \
      /* test raw_before */ \
      SET login = 'foo' \
      /* test raw_after */\
    ";
    let expected_query_foo_bar = "\
      /* test raw */ \
      UPDATE users \
      /* test raw_before */ \
      SET login = 'foo', name = 'Bar' \
      /* test raw_after */\
    ";

    assert_eq!(query_foo, expected_query_foo);
    assert_eq!(query_foo_bar, expected_query_foo_bar);
  }

  #[test]
  fn update_builder_should_be_able_to_conditionally_add_clauses() {
    let mut update = sql::Update::new().update("users").set("name = 'Bar'");

    if true {
      update = update.set("login = 'bar'");
    }

    let query = update.as_string();
    let expected_query = "UPDATE users SET name = 'Bar', login = 'bar'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn update_builder_should_be_composable() {
    fn update(update: sql::Update) -> sql::Update {
      update.update("users")
    }

    fn sets(update: sql::Update) -> sql::Update {
      update.set("login = 'foo'").set("name = 'Bar'").set("age = 42")
    }

    fn conditions(update: sql::Update) -> sql::Update {
      update
        .where_clause("id = $1")
        .where_clause("active = true")
        .where_clause("created_at::date = current_date")
    }

    fn as_string(update: sql::Update) -> String {
      update.as_string()
    }

    let query = Some(sql::Update::new())
      .map(update)
      .map(sets)
      .map(conditions)
      .map(as_string)
      .unwrap();

    let expected_query = "\
      UPDATE users \
      SET \
        login = 'foo', \
        name = 'Bar', \
        age = 42 \
      WHERE \
        id = $1 \
        AND active = true \
        AND created_at::date = current_date\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn all_standard_clauses_concatenated_in_order() {
    let query = sql::Update::new()
      .update("users")
      .set("users.name = 'Foo'")
      .where_clause("users.login = $1")
      .as_string();

    let expected_query = "\
      UPDATE users \
      SET users.name = 'Foo' \
      WHERE users.login = $1\
    ";

    assert_eq!(query, expected_query);
  }

  /** This test can fail only at compile time
   * [More context](https://github.com/belchior/sql_query_builder/pull/53)
   */
  #[test]
  fn update_builder_should_impl_send_and_sync() {
    fn assert_impl_sync_send(_builder: impl Sync + Send) {}
    assert_impl_sync_send(sql::Update::new());
  }
}

mod builder_methods {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_as_string_should_convert_the_current_state_into_string() {
    let query = sql::Update::new().as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_debug_should_print_at_console_in_a_human_readable_format() {
    let query = sql::Update::new().update("users").debug().as_string();
    let expected_query = "UPDATE users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_new_should_initialize_as_empty_string() {
    let query = sql::Update::new().as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_print_should_print_in_one_line_the_current_state_of_builder() {
    let query = sql::Update::new().update("users").print().as_string();
    let expected_query = "UPDATE users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_add_raw_sql() {
    let query = sql::Update::new().raw("update addresses").as_string();
    let expected_query = "update addresses";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Update::new()
      .raw("update addresses")
      .raw("set city = 'Foo'")
      .as_string();
    let expected_query = "update addresses set city = 'Foo'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::Update::new().raw("").raw("update addresses").raw("").as_string();
    let expected_query = "update addresses";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_be_the_first_to_be_concatenated() {
    let query = sql::Update::new()
      .raw("update addresses")
      .set("country = 'Bar'")
      .as_string();
    let expected_query = "update addresses SET country = 'Bar'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_trim_space_of_the_argument() {
    let query = sql::Update::new().raw("  update users  ").as_string();
    let expected_query = "update users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Update::new().raw("update users").raw("update users").as_string();
    let expected_query = "update users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_trim_space_of_the_argument() {
    let query = sql::Update::new()
      .raw_after(sql::UpdateClause::Update, "  set name = 'Bar'  ")
      .as_string();
    let expected_query = "set name = 'Bar'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_trim_space_of_the_argument() {
    let query = sql::Update::new()
      .raw_before(sql::UpdateClause::Where, "  set name = 'Bar'  ")
      .as_string();
    let expected_query = "set name = 'Bar'";

    assert_eq!(query, expected_query);
  }
}
