mod release_savepoint_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_release_savepoint_should_add_a_release_savepoint_command() {
    let query = sql::Transaction::new().release_savepoint("foo").as_string();
    let expected_query = "RELEASE SAVEPOINT foo;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_release_savepoint_should_trim_space_of_the_argument() {
    let query = sql::Transaction::new().release_savepoint("  bar  ").as_string();
    let expected_query = "RELEASE SAVEPOINT bar;";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_release_savepoint_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Transaction::new()
      .release_savepoint("foo")
      .release_savepoint("bar")
      .as_string();
    let expected_query = "RELEASE SAVEPOINT foo; RELEASE SAVEPOINT bar;";

    assert_eq!(query, expected_query);
  }
}
