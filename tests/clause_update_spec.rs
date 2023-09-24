mod update_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_update_should_add_a_update_clause() {
    let query = sql::Update::new().update("users").as_string();
    let expected_query = "UPDATE users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_update_should_override_value_on_consecutive_calls() {
    let query = sql::Update::new().update("users").update("orders").as_string();
    let expected_query = "UPDATE orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_update_should_trim_space_of_the_argument() {
    let query = sql::Update::new().update("  orders  ").as_string();
    let expected_query = "UPDATE orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_update_clause() {
    let query = sql::Update::new()
      .raw_before(sql::UpdateClause::Update, "/* update users */")
      .update("users")
      .as_string();
    let expected_query = "/* update users */ UPDATE users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_update_clause() {
    let query = sql::Update::new()
      .update("users")
      .raw_after(sql::UpdateClause::Update, "set login = 'foo'")
      .as_string();
    let expected_query = "UPDATE users set login = 'foo'";

    assert_eq!(query, expected_query);
  }
}
