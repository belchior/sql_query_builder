mod select_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_window_should_add_the_window_clause() {
    let query = sql::Select::new().window("win").as_string();
    let expected_query = "WINDOW win";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_window_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Select::new().window("foo").window("bar").as_string();
    let expected_query = "WINDOW foo, bar";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_window_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::Select::new().window("").window("bar").window("").as_string();
    let expected_query = "WINDOW bar";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_window_should_trim_space_of_the_argument() {
    let query = sql::Select::new().window("  foo  ").as_string();
    let expected_query = "WINDOW foo";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_window_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Select::new().window("bar").window("bar").as_string();
    let expected_query = "WINDOW bar";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_window_should_be_after_group_by_clause() {
    let query = sql::Select::new().group_by("login").window("department").as_string();
    let expected_query = "GROUP BY login WINDOW department";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_window_should_be_after_having_clause() {
    let query = sql::Select::new().having("id = $1").window("foo").as_string();
    let expected_query = "HAVING id = $1 WINDOW foo";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_window_clause() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::Window, "group by login")
      .window("orders")
      .as_string();
    let expected_query = "group by login WINDOW orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_window_clause() {
    let query = sql::Select::new()
      .window("users")
      .raw_after(sql::SelectClause::Window, "limit 10")
      .as_string();
    let expected_query = "WINDOW users limit 10";

    assert_eq!(query, expected_query);
  }
}
