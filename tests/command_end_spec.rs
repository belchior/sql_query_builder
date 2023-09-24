#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod end_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_end_should_add_a_end_command() {
    let query = sql::Transaction::new().end("").as_string();
    let expected_query = "END;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_end_should_trim_space_of_the_argument() {
    let query = sql::Transaction::new().end("  TRANSACTION  ").as_string();
    let expected_query = "END TRANSACTION;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_end_should_override_the_previews_value_on_consecutive_calls() {
    let query = sql::Transaction::new().end("TRANSACTION").end("").as_string();
    let expected_query = "END;";

    assert_eq!(query, expected_query);
  }
}
