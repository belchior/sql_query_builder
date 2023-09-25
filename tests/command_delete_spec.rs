mod builder_features {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn delete_builder_should_be_displayable() {
    let delete = sql::Delete::new().delete_from("users").where_clause("login = 'foo'");

    println!("{}", delete);

    let query = delete.as_string();
    let expected_query = "DELETE FROM users WHERE login = 'foo'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn delete_builder_should_be_debuggable() {
    let delete = sql::Delete::new()
      .delete_from("users")
      .where_clause("name = 'Foo'")
      .where_clause("login = 'foo'");

    println!("{:?}", delete);

    let expected_query = "DELETE FROM users WHERE name = 'Foo' AND login = 'foo'";
    let query = delete.as_string();

    assert_eq!(query, expected_query);
  }

  #[test]
  fn delete_builder_should_be_cloneable() {
    let delete_foo = sql::Delete::new()
      .raw("/* test raw */")
      .delete_from("users")
      .raw_before(sql::DeleteClause::Where, "/* test raw_before */")
      .where_clause("login = 'foo'")
      .raw_after(sql::DeleteClause::Where, "/* test raw_after */");

    let delete_foo_bar = delete_foo.clone().where_clause("name = 'Bar'");

    let query_foo = delete_foo.as_string();
    let query_foo_bar = delete_foo_bar.as_string();

    let expected_query_foo = "\
      /* test raw */ \
      DELETE FROM users \
      /* test raw_before */ \
      WHERE login = 'foo' \
      /* test raw_after */\
    ";
    let expected_query_foo_bar = "\
      /* test raw */ \
      DELETE FROM users \
      /* test raw_before */ \
      WHERE login = 'foo' AND name = 'Bar' \
      /* test raw_after */\
    ";

    assert_eq!(query_foo, expected_query_foo);
    assert_eq!(query_foo_bar, expected_query_foo_bar);
  }

  #[test]
  fn delete_builder_should_be_able_to_conditionally_add_clauses() {
    let mut delete = sql::Delete::new().delete_from("users").where_clause("name = 'Bar'");

    if true {
      delete = delete.where_clause("login = 'bar'");
    }

    let query = delete.as_string();
    let expected_query = "DELETE FROM users WHERE name = 'Bar' AND login = 'bar'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn delete_builder_should_be_composable() {
    fn delete(delete: sql::Delete) -> sql::Delete {
      delete.delete_from("users")
    }

    fn conditions(delete: sql::Delete) -> sql::Delete {
      delete
        .where_clause("id = $1")
        .where_clause("active = true")
        .where_clause("created_at::date = current_date")
    }

    fn as_string(delete: sql::Delete) -> String {
      delete.as_string()
    }

    let query = Some(sql::Delete::new())
      .map(delete)
      .map(conditions)
      .map(as_string)
      .unwrap();

    let expected_query = "\
      DELETE FROM users \
      WHERE \
        id = $1 \
        AND active = true \
        AND created_at::date = current_date\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn all_standard_clauses_concatenated_in_order() {
    let query = sql::Delete::new()
      .delete_from("users")
      .where_clause("users.login = $1")
      .as_string();

    let expected_query = "\
      DELETE FROM users \
      WHERE users.login = $1\
    ";

    assert_eq!(query, expected_query);
  }
}

mod builder_methods {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_as_string_should_convert_the_current_state_into_string() {
    let query = sql::Delete::new().as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_debug_should_print_at_console_in_a_human_readable_format() {
    let query = sql::Delete::new().delete_from("users").debug().as_string();
    let expected_query = "DELETE FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_new_should_initialize_as_empty_string() {
    let query = sql::Delete::new().as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_print_should_print_in_one_line_the_current_state_of_builder() {
    let query = sql::Delete::new().delete_from("users").print().as_string();
    let expected_query = "DELETE FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_add_raw_sql() {
    let query = sql::Delete::new().raw("delete from addresses").as_string();
    let expected_query = "delete from addresses";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Delete::new()
      .raw("delete from addresses")
      .raw("where city = 'Foo'")
      .as_string();
    let expected_query = "delete from addresses where city = 'Foo'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_be_the_first_to_be_concatenated() {
    let query = sql::Delete::new()
      .raw("delete from addresses")
      .where_clause("country = 'Bar'")
      .as_string();
    let expected_query = "delete from addresses WHERE country = 'Bar'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_trim_space_of_the_argument() {
    let query = sql::Delete::new().raw("  delete from users  ").as_string();
    let expected_query = "delete from users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Delete::new()
      .raw("delete from users")
      .raw("delete from users")
      .as_string();
    let expected_query = "delete from users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_trim_space_of_the_argument() {
    let query = sql::Delete::new()
      .raw_after(sql::DeleteClause::DeleteFrom, "  where name = 'Bar'  ")
      .as_string();
    let expected_query = "where name = 'Bar'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_trim_space_of_the_argument() {
    let query = sql::Delete::new()
      .raw_before(sql::DeleteClause::Where, "  where name = 'Bar'  ")
      .as_string();
    let expected_query = "where name = 'Bar'";

    assert_eq!(query, expected_query);
  }
}
