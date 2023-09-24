#[cfg(not(feature = "sqlite"))]
mod insert_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_overriding_should_add_a_overriding_clause() {
    let query = sql::Insert::new().overriding("user value").as_string();
    let expected_query = "OVERRIDING user value";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_overriding_should_override_value_on_consecutive_calls() {
    let query = sql::Insert::new()
      .overriding("user value")
      .overriding("system value")
      .as_string();
    let expected_query = "OVERRIDING system value";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_overrinding_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().overriding("  system value  ").as_string();
    let expected_query = "OVERRIDING system value";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_overriding_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::Overriding, "insert into users (login, name)")
      .overriding("system value")
      .as_string();
    let expected_query = "insert into users (login, name) OVERRIDING system value";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_overriding_clause() {
    let query = sql::Insert::new()
      .overriding("user value")
      .raw_after(sql::InsertClause::Overriding, "values ('baz', 'Baz')")
      .as_string();
    let expected_query = "OVERRIDING user value values ('baz', 'Baz')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_overriding_should_be_after_insert_into_clause() {
    let query = sql::Insert::new()
      .overriding("system value")
      .insert_into("users (login, name)")
      .as_string();
    let expected_query = "INSERT INTO users (login, name) OVERRIDING system value";

    assert_eq!(query, expected_query);
  }
}
