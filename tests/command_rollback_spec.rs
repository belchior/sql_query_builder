mod rollback_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_rollback_should_add_a_rollback_command() {
    let query = sql::Transaction::new().rollback("TRANSACTION").as_string();
    let expected_query = "ROLLBACK TRANSACTION;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_rollback_should_trim_space_of_the_argument() {
    let query = sql::Transaction::new().rollback("  WORK  ").as_string();
    let expected_query = "ROLLBACK WORK;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_rollback_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Transaction::new()
      .rollback("TRANSACTION")
      .rollback("TO SAVEPOINT foo")
      .as_string();
    let expected_query = "ROLLBACK TRANSACTION; ROLLBACK TO SAVEPOINT foo;";

    assert_eq!(query, expected_query);
  }
}
