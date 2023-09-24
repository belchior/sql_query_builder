#[cfg(feature = "sqlite")]
mod insert_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_default_values_should_add_a_default_values_clause() {
    let query = sql::Insert::new().insert_into("users").default_values().as_string();
    let expected_query = "INSERT INTO users DEFAULT VALUES";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_default_values_should_do_nothing_on_consecutive_calls() {
    let query = sql::Insert::new().default_values().default_values().as_string();
    let expected_query = "DEFAULT VALUES";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_default_values_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::DefaultValues, "insert into users (login, name)")
      .default_values()
      .as_string();
    let expected_query = "insert into users (login, name) DEFAULT VALUES";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_values_clause() {
    let query = sql::Insert::new()
      .default_values()
      .raw_after(sql::InsertClause::DefaultValues, "-- default values test")
      .as_string();
    let expected_query = "DEFAULT VALUES -- default values test";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_values_should_be_after_insert_into_clause() {
    let query = sql::Insert::new()
      .default_values()
      .insert_into("users (login, name)")
      .as_string();
    let expected_query = "INSERT INTO users (login, name) DEFAULT VALUES";

    assert_eq!(query, expected_query);
  }
}
