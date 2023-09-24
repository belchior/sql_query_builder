mod select_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_order_by_should_add_the_order_by_clause() {
    let query = sql::Select::new().order_by("id asc").as_string();
    let expected_query = "ORDER BY id asc";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_order_by_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Select::new()
      .order_by("login asc")
      .order_by("created_at desc")
      .as_string();
    let expected_query = "ORDER BY login asc, created_at desc";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_order_by_should_trim_space_of_the_argument() {
    let query = sql::Select::new().order_by("  id desc  ").as_string();
    let expected_query = "ORDER BY id desc";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_order_by_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Select::new().order_by("id desc").order_by("id desc").as_string();
    let expected_query = "ORDER BY id desc";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_order_by_should_be_after_having_clause() {
    let query = sql::Select::new()
      .having("active = true")
      .order_by("created_at desc")
      .as_string();
    let expected_query = "HAVING active = true ORDER BY created_at desc";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_order_by_clause() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::OrderBy, "where orders.user_login = $1")
      .order_by("id desc")
      .as_string();
    let expected_query = "where orders.user_login = $1 ORDER BY id desc";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_order_by_clause() {
    let query = sql::Select::new()
      .order_by("id desc")
      .raw_after(sql::SelectClause::OrderBy, "limit 20")
      .as_string();
    let expected_query = "ORDER BY id desc limit 20";

    assert_eq!(query, expected_query);
  }
}
