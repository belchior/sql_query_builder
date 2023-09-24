#[cfg(not(feature = "sqlite"))]
mod start_transaction_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_start_transaction_should_add_a_start_transaction_command() {
    let query = sql::Transaction::new().start_transaction("").as_string();
    let expected_query = "START TRANSACTION;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_start_transaction_should_add_the_transaction_mode_argument() {
    let query = sql::Transaction::new().start_transaction("READ WRITE").as_string();
    let expected_query = "START TRANSACTION READ WRITE;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_start_transaction_should_trim_space_of_the_argument() {
    let query = sql::Transaction::new().start_transaction("  READ WRITE  ").as_string();
    let expected_query = "START TRANSACTION READ WRITE;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_start_transaction_should_override_the_previews_value_on_consecutive_calls() {
    let query = sql::Transaction::new()
      .start_transaction("ISOLATION LEVEL SERIALIZABLE")
      .start_transaction("ISOLATION LEVEL REPEATABLE READ")
      .as_string();
    let expected_query = "START TRANSACTION ISOLATION LEVEL REPEATABLE READ;";

    assert_eq!(query, expected_query);
  }
}
