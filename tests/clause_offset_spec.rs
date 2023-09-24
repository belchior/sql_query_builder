#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod select_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_offset_should_add_the_offset_clause() {
    let query = sql::Select::new().offset("100").as_string();
    let expected_query = "OFFSET 100";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_offset_should_override_the_current_value() {
    let query = sql::Select::new().offset("100").offset("200").as_string();
    let expected_query = "OFFSET 200";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_offset_should_trim_space_of_the_argument() {
    let query = sql::Select::new().offset("  2000  ").as_string();
    let expected_query = "OFFSET 2000";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_offset_should_be_after_limit_clause() {
    let query = sql::Select::new().limit("500").offset("100").as_string();
    let expected_query = "LIMIT 500 OFFSET 100";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_offset_clause() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::Limit, "limit 1000")
      .offset("50")
      .as_string();
    let expected_query = "limit 1000 OFFSET 50";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_offset_clause() {
    let query = sql::Select::new()
      .offset("10")
      .raw_after(sql::SelectClause::Offset, "/* the end */")
      .as_string();
    let expected_query = "OFFSET 10 /* the end */";

    assert_eq!(query, expected_query);
  }
}
