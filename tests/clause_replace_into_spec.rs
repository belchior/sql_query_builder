#[cfg(feature = "sqlite")]
mod insert_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_replace_into_should_add_a_replace_into_clause() {
    let query = sql::Insert::new().replace_into("users (login, name)").as_string();
    let expected_query = "REPLACE INTO users (login, name)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_replace_into_should_override_value_on_consecutive_calls() {
    let query = sql::Insert::new()
      .replace_into("users (login, name)")
      .replace_into("orders (product_name, price)")
      .as_string();
    let expected_query = "REPLACE INTO orders (product_name, price)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_replace_into_should_not_add_clause_when_argument_is_empty() {
    let query = sql::Insert::new().replace_into("").as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_replace_into_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().replace_into("  users (name)  ").as_string();
    let expected_query = "REPLACE INTO users (name)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_replace_into_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::ReplaceInto, "/* replace into users */")
      .replace_into("users")
      .as_string();
    let expected_query = "/* replace into users */ REPLACE INTO users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_replace_into_clause() {
    let query = sql::Insert::new()
      .replace_into("users (name)")
      .raw_after(sql::InsertClause::ReplaceInto, "values ('foo')")
      .as_string();
    let expected_query = "REPLACE INTO users (name) values ('foo')";

    assert_eq!(query, expected_query);
  }
}
