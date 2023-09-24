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
