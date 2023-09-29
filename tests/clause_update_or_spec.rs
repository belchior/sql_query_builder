#[cfg(feature = "sqlite")]
mod update_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_update_or_should_add_the_update_or_clause() {
    let query = sql::Update::new().update_or("ABORT users (login, name)").as_string();
    let expected_query = "UPDATE OR ABORT users (login, name)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_update_or_should_override_value_on_consecutive_calls() {
    let query = sql::Update::new()
      .update_or("FAIL users")
      .update_or("IGNORE orders")
      .as_string();
    let expected_query = "UPDATE OR IGNORE orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn methods_update_and_update_or_should_override_each_other() {
    let query = sql::Update::new()
      .update("users")
      .update_or("IGNORE orders")
      .as_string();
    let expected_query = "UPDATE OR IGNORE orders";

    assert_eq!(query, expected_query);

    let query = sql::Update::new()
      .update_or("IGNORE orders")
      .update("users")
      .as_string();
    let expected_query = "UPDATE users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_update_or_should_not_add_clause_when_argument_is_empty() {
    let query = sql::Update::new().update_or("").as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_update_or_should_trim_space_of_the_argument() {
    let query = sql::Update::new().update_or("  REPLACE orders  ").as_string();
    let expected_query = "UPDATE OR REPLACE orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_update_or_clause() {
    let query = sql::Update::new()
      .raw_before(sql::UpdateClause::UpdateOr, "/* update_or users */")
      .update_or("ROLLBACK users")
      .as_string();
    let expected_query = "/* update_or users */ UPDATE OR ROLLBACK users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_update_or_clause() {
    let query = sql::Update::new()
      .update_or("ABORT users")
      .raw_after(sql::UpdateClause::UpdateOr, "set login = 'foo'")
      .as_string();
    let expected_query = "UPDATE OR ABORT users set login = 'foo'";

    assert_eq!(query, expected_query);
  }
}
