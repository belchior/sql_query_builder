#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod begin_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_begin_should_add_a_begin_command() {
    let query = sql::Transaction::new().begin("").as_string();
    let expected_query = "BEGIN;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_begin_should_add_the_transaction_mode_argument() {
    let query = sql::Transaction::new().begin("READ WRITE").as_string();
    let expected_query = "BEGIN READ WRITE;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_begin_should_trim_space_of_the_argument() {
    let query = sql::Transaction::new().begin("  READ WRITE  ").as_string();
    let expected_query = "BEGIN READ WRITE;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_begin_should_override_the_previews_value_on_consecutive_calls() {
    let query = sql::Transaction::new()
      .begin("ISOLATION LEVEL SERIALIZABLE")
      .begin("ISOLATION LEVEL REPEATABLE READ")
      .as_string();
    let expected_query = "BEGIN ISOLATION LEVEL REPEATABLE READ;";

    assert_eq!(query, expected_query);
  }

  #[cfg(not(feature = "sqlite"))]
  #[test]
  fn method_begin_should_not_override_the_start_transaction_on_consecutive_calls() {
    let query = sql::Transaction::new().start_transaction("").begin("").as_string();
    let expected_query = "BEGIN; START TRANSACTION;";

    assert_eq!(query, expected_query);
  }

  #[cfg(not(feature = "sqlite"))]
  #[test]
  fn method_begin_should_not_be_overrided_by_start_transaction_method_on_consecutive_calls() {
    let query = sql::Transaction::new().begin("").start_transaction("").as_string();
    let expected_query = "BEGIN; START TRANSACTION;";

    assert_eq!(query, expected_query);
  }
}
