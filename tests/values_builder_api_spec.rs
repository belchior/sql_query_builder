mod builder_methods {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_new_should_initialize_as_empty_string() {
    let query = sql::Values::new().as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_as_string_should_convert_the_current_state_into_string() {
    let query = sql::Values::new().as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_debug_should_print_at_console_in_a_human_readable_format() {
    let query = sql::Values::new()
      .values("(1, 'one')")
      .values("(2, 'two')")
      .debug()
      .as_string();
    let expected_query = "VALUES (1, 'one'), (2, 'two')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_print_should_print_in_one_line_the_current_state_of_builder() {
    let query = sql::Values::new()
      .values("(1, 'one')")
      .values("(2, 'two')")
      .print()
      .as_string();
    let expected_query = "VALUES (1, 'one'), (2, 'two')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_add_raw_sql_on_top_of_the_command() {
    let query = sql::Values::new()
      .raw("/* the values command */")
      .values("(1, 'one')")
      .as_string();
    let expected_query = "/* the values command */ VALUES (1, 'one')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Values::new().raw("/* raw one */").raw("/* raw two */").as_string();
    let expected_query = "/* raw one */ /* raw two */";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_be_the_first_to_be_concatenated() {
    let query = sql::Values::new()
      .values("(1, 'one')")
      .raw("insert into my_table(num, txt)")
      .as_string();
    let expected_query = "insert into my_table(num, txt) VALUES (1, 'one')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_trim_space_of_the_argument() {
    let query = sql::Values::new().raw("  /* raw one */  ").as_string();
    let expected_query = "/* raw one */";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Values::new()
      .raw("/* should not be repeat */")
      .raw("/* should not be repeat */")
      .as_string();
    let expected_query = "/* should not be repeat */";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_trim_space_of_the_argument() {
    let query = sql::Values::new()
      .raw_after(sql::ValuesClause::Values, "  /* raw one */  ")
      .as_string();
    let expected_query = "/* raw one */";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_trim_space_of_the_argument() {
    let query = sql::Values::new()
      .raw_before(sql::ValuesClause::Values, "  /* raw one */  ")
      .as_string();
    let expected_query = "/* raw one */";

    assert_eq!(query, expected_query);
  }
}

mod values_clause {
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
