mod savepoint_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_savepoint_should_add_a_savepoint_command() {
    let query = sql::Transaction::new().savepoint("foo").as_string();
    let expected_query = "SAVEPOINT foo;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_savepoint_should_trim_space_of_the_argument() {
    let query = sql::Transaction::new().savepoint("  bar  ").as_string();
    let expected_query = "SAVEPOINT bar;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_savepoint_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Transaction::new().savepoint("foo").savepoint("bar").as_string();
    let expected_query = "SAVEPOINT foo; SAVEPOINT bar;";

    assert_eq!(query, expected_query);
  }
}
