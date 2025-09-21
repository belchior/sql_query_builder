#[cfg(feature = "mysql")]
mod insert_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_row_should_add_a_row_clause() {
    let query = sql::Insert::new().row("('foo', 'Foo')").as_string();
    let expected_query = "VALUES ROW('foo', 'Foo')";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_row_should_accumulate_row_on_consecutive_calls() {
    let query = sql::Insert::new()
      .row("('foo', 'Foo')")
      .row("('bar', 'Bar')")
      .as_string();
    let expected_query = "VALUES ROW('foo', 'Foo'), ROW('bar', 'Bar')";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_row_should_not_accumulate_row_when_expression_is_empty() {
    let query = sql::Insert::new().row("").row("('bar', 'Bar')").row("").as_string();
    let expected_query = "VALUES ROW('bar', 'Bar')";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_row_should_not_produce_the_values_clause_when_has_only_calls_with_empty_argument() {
    let query = sql::Insert::new().row("").row("").as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_row_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().row("   ('Bar')  ").as_string();
    let expected_query = "VALUES ROW('Bar')";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_row_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Insert::new()
      .row("('bar', 'Bar')")
      .row("('bar', 'Bar')")
      .as_string();
    let expected_query = "VALUES ROW('bar', 'Bar')";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_values_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::Values, "insert into users (login, name)")
      .row("('foo', 'Foo')")
      .as_string();
    let expected_query = "insert into users (login, name) VALUES ROW('foo', 'Foo')";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_values_clause() {
    let query = sql::Insert::new()
      .row("('baz', 'Baz')")
      .raw_after(sql::InsertClause::Values, ", ROW('foo', 'Foo')")
      .as_string();
    let expected_query = "VALUES ROW('baz', 'Baz') , ROW('foo', 'Foo')";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_row_should_be_after_insert_into_clause() {
    let query = sql::Insert::new()
      .row("('bar', 'Bar')")
      .insert_into("users (login, name)")
      .as_string();
    let expected_query = "INSERT INTO users (login, name) VALUES ROW('bar', 'Bar')";

    assert_eq!(expected_query, query);
  }
}

#[cfg(feature = "mysql")]
mod values_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_row_should_add_a_values_statement_with_the_row_constructor_clause() {
    let query = sql::Values::new().row("('foo', 'Foo')").as_string();
    let expected_query = "VALUES ROW('foo', 'Foo')";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_row_should_accumulate_rows_on_consecutive_calls() {
    let query = sql::Values::new()
      .row("('foo', 'Foo')")
      .row("('bar', 'Bar')")
      .as_string();
    let expected_query = "VALUES ROW('foo', 'Foo'), ROW('bar', 'Bar')";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_row_should_not_accumulate_row_when_expression_is_empty() {
    let query = sql::Values::new().row("").row("('foo', 'Foo')").row("").as_string();
    let expected_query = "VALUES ROW('foo', 'Foo')";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_row_should_not_produce_values_statement_when_has_only_empty_rows() {
    let query = sql::Values::new().row("").row("").as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_row_should_trim_space_of_the_argument() {
    let query = sql::Values::new().row("   ('Bar')  ").as_string();
    let expected_query = "VALUES ROW('Bar')";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_row_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Values::new()
      .row("('bar', 'Bar')")
      .row("('bar', 'Bar')")
      .as_string();
    let expected_query = "VALUES ROW('bar', 'Bar')";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn when_call_raw_after_method_the_content_should_be_added_after_the_statement() {
    let query = sql::Values::new()
      .row("('foo', 'Foo')")
      .row("('bar', 'Bar')")
      .raw_before(sql::ValuesClause::Values, "insert into users (login, name)")
      .as_string();
    let expected_query = "\
      insert into users (login, name) \
      VALUES ROW('foo', 'Foo'), ROW('bar', 'Bar')\
    ";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn when_call_raw_before_method_the_content_should_be_added_before_the_statement() {
    let query = sql::Values::new()
      .row("('foo', 'Foo')")
      .row("('bar', 'Bar')")
      .raw_after(sql::ValuesClause::Values, ", ROW('ros', 'Ros')")
      .as_string();
    let expected_query = "VALUES ROW('foo', 'Foo'), ROW('bar', 'Bar') , ROW('ros', 'Ros')";

    assert_eq!(expected_query, query);
  }
}
