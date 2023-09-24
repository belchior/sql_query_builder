mod delete_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_delete_should_add_a_delete_clause() {
    let query = sql::Delete::new().delete_from("users").as_string();
    let expected_query = "DELETE FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_delete_should_override_value_on_consecutive_calls() {
    let query = sql::Delete::new()
      .delete_from("users")
      .delete_from("orders")
      .as_string();
    let expected_query = "DELETE FROM orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_delete_should_trim_space_of_the_argument() {
    let query = sql::Delete::new().delete_from("  orders  ").as_string();
    let expected_query = "DELETE FROM orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_delete_clause() {
    let query = sql::Delete::new()
      .raw_before(sql::DeleteClause::DeleteFrom, "/* delete users */")
      .delete_from("users")
      .as_string();
    let expected_query = "/* delete users */ DELETE FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_delete_clause() {
    let query = sql::Delete::new()
      .delete_from("users")
      .raw_after(sql::DeleteClause::DeleteFrom, "where login = 'foo'")
      .as_string();
    let expected_query = "DELETE FROM users where login = 'foo'";

    assert_eq!(query, expected_query);
  }
}
