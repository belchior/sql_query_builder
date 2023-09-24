mod commit_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_commit_should_add_a_commit_command() {
    let query = sql::Transaction::new().commit("").as_string();
    let expected_query = "COMMIT;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_commit_should_add_the_transaction_mode_argument() {
    let query = sql::Transaction::new().commit("TRANSACTION").as_string();
    let expected_query = "COMMIT TRANSACTION;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_commit_should_trim_space_of_the_argument() {
    let query = sql::Transaction::new().commit("  TRANSACTION  ").as_string();
    let expected_query = "COMMIT TRANSACTION;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_commit_should_override_the_previews_value_on_consecutive_calls() {
    let query = sql::Transaction::new().commit("TRANSACTION").commit("WORK").as_string();
    let expected_query = "COMMIT WORK;";

    assert_eq!(query, expected_query);
  }
}
