mod insert_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_values_should_add_a_values_clause() {
    let query = sql::Insert::new().values("('foo', 'Foo')").as_string();
    let expected_query = "VALUES ('foo', 'Foo')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_values_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Insert::new()
      .values("('foo', 'Foo')")
      .values("('bar', 'Bar')")
      .as_string();
    let expected_query = "VALUES ('foo', 'Foo'), ('bar', 'Bar')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_values_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().values("   ('Bar')  ").as_string();
    let expected_query = "VALUES ('Bar')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_values_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Insert::new()
      .values("('bar', 'Bar')")
      .values("('bar', 'Bar')")
      .as_string();
    let expected_query = "VALUES ('bar', 'Bar')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_values_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::Values, "insert into users (login, name)")
      .values("('foo', 'Foo')")
      .as_string();
    let expected_query = "insert into users (login, name) VALUES ('foo', 'Foo')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_values_clause() {
    let query = sql::Insert::new()
      .values("('baz', 'Baz')")
      .raw_after(sql::InsertClause::Values, ", ('foo', 'Foo')")
      .as_string();
    let expected_query = "VALUES ('baz', 'Baz') , ('foo', 'Foo')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_values_should_be_after_insert_into_clause() {
    let query = sql::Insert::new()
      .values("('bar', 'Bar')")
      .insert_into("users (login, name)")
      .as_string();
    let expected_query = "INSERT INTO users (login, name) VALUES ('bar', 'Bar')";

    assert_eq!(query, expected_query);
  }
}

mod values_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_values_should_add_a_values_clause() {
    let query = sql::Values::new().values("('foo', 'Foo')").as_string();
    let expected_query = "VALUES ('foo', 'Foo')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_values_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Values::new()
      .values("('foo', 'Foo')")
      .values("('bar', 'Bar')")
      .as_string();
    let expected_query = "VALUES ('foo', 'Foo'), ('bar', 'Bar')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_values_should_trim_space_of_the_argument() {
    let query = sql::Values::new().values("   ('Bar')  ").as_string();
    let expected_query = "VALUES ('Bar')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_values_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Values::new()
      .values("('bar', 'Bar')")
      .values("('bar', 'Bar')")
      .as_string();
    let expected_query = "VALUES ('bar', 'Bar')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_values_clause() {
    let query = sql::Values::new()
      .raw_before(sql::ValuesClause::Values, "insert into users (login, name)")
      .values("('foo', 'Foo')")
      .as_string();
    let expected_query = "insert into users (login, name) VALUES ('foo', 'Foo')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_values_clause() {
    let query = sql::Values::new()
      .values("('baz', 'Baz')")
      .raw_after(sql::ValuesClause::Values, ", ('foo', 'Foo')")
      .as_string();
    let expected_query = "VALUES ('baz', 'Baz') , ('foo', 'Foo')";

    assert_eq!(query, expected_query);
  }
}
