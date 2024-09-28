#[cfg(not(feature = "sqlite"))]
mod set_transaction_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_set_transaction_should_add_a_set_transaction_command() {
    let query = sql::Transaction::new().set_transaction("").as_string();
    let expected_query = "SET TRANSACTION;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_set_transaction_should_add_the_transaction_mode_argument() {
    let query = sql::Transaction::new().set_transaction("READ WRITE").as_string();
    let expected_query = "SET TRANSACTION READ WRITE;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_set_transaction_should_trim_space_of_the_argument() {
    let query = sql::Transaction::new().set_transaction("  READ WRITE  ").as_string();
    let expected_query = "SET TRANSACTION READ WRITE;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_set_transaction_should_overrides_the_current_value_on_consecutive_calls() {
    let query = sql::Transaction::new()
      .set_transaction("ISOLATION LEVEL SERIALIZABLE")
      .set_transaction("ISOLATION LEVEL REPEATABLE READ")
      .as_string();
    let expected_query = "SET TRANSACTION ISOLATION LEVEL REPEATABLE READ;";

    assert_eq!(query, expected_query);
  }
}
