mod full_api {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[cfg(not(feature = "mysql"))]
  #[test]
  fn sql_standard_with_all_methods() {
    let query = sql::Values::new().values("(1, 'one')").as_string();

    let expected_query = "VALUES (1, 'one')";

    assert_eq!(expected_query, query);
  }

  #[cfg(feature = "postgresql")]
  #[test]
  fn postgres_with_all_methods() {
    let query = sql::Values::new().values("(1, 'one')").as_string();

    let expected_query = "VALUES (1, 'one')";

    assert_eq!(expected_query, query);
  }

  #[cfg(feature = "mysql")]
  #[test]
  fn mysql_with_all_methods() {
    let query = sql::Values::new().row("(1, 'one')").as_string();

    let expected_query = "VALUES ROW(1, 'one')";

    assert_eq!(expected_query, query);
  }
}

mod builder_features {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn values_builder_should_be_displayable() {
    #[cfg(not(feature = "mysql"))]
    {
      let values = sql::Values::new().values("('foo', 'Foo')").values("('bar', 'Bar')");

      println!("{}", values);

      let query = values.as_string();
      let expected_query = "VALUES ('foo', 'Foo'), ('bar', 'Bar')";

      assert_eq!(expected_query, query);
    }

    #[cfg(feature = "mysql")]
    {
      let values = sql::Values::new().row("('foo', 'Foo')").row("('bar', 'Bar')");

      println!("{}", values);

      let query = values.as_string();
      let expected_query = "VALUES ROW('foo', 'Foo'), ROW('bar', 'Bar')";

      assert_eq!(expected_query, query);
    }
  }

  #[test]
  fn values_builder_should_be_debuggable() {
    #[cfg(not(feature = "mysql"))]
    {
      let values = sql::Values::new().values("('foo', 'Foo')").values("('bar', 'Bar')");

      println!("{:?}", values);

      let expected_query = "VALUES ('foo', 'Foo'), ('bar', 'Bar')";
      let query = values.as_string();

      assert_eq!(expected_query, query);
    }

    #[cfg(feature = "mysql")]
    {
      let values = sql::Values::new().row("('foo', 'Foo')").row("('bar', 'Bar')");

      println!("{:?}", values);

      let expected_query = "VALUES ROW('foo', 'Foo'), ROW('bar', 'Bar')";
      let query = values.as_string();

      assert_eq!(expected_query, query);
    }
  }

  #[test]
  fn values_builder_should_be_cloneable() {
    #[cfg(not(feature = "mysql"))]
    {
      let values_foo = sql::Values::new()
        .raw("/* test raw */")
        .raw_before(sql::ValuesClause::Values, "/* test raw_before */")
        .values("('foo', 'Foo')")
        .raw_after(sql::ValuesClause::Values, "/* test raw_after */");

      let values_foo_bar = values_foo.clone().values("('bar', 'Bar')");

      let query_foo = values_foo.as_string();
      let query_foo_bar = values_foo_bar.as_string();

      let expected_query_foo = "\
        /* test raw */ \
        /* test raw_before */ \
        VALUES ('foo', 'Foo') \
        /* test raw_after */\
      ";
      let expected_query_foo_bar = "\
        /* test raw */ \
        /* test raw_before */ \
        VALUES ('foo', 'Foo'), ('bar', 'Bar') \
        /* test raw_after */\
      ";

      assert_eq!(expected_query_foo, query_foo);
      assert_eq!(expected_query_foo_bar, query_foo_bar);
    }

    #[cfg(feature = "mysql")]
    {
      let values_foo = sql::Values::new()
        .raw("/* test raw */")
        .raw_before(sql::ValuesClause::Values, "/* test raw_before */")
        .row("('foo', 'Foo')")
        .raw_after(sql::ValuesClause::Values, "/* test raw_after */");

      let values_foo_bar = values_foo.clone().row("('bar', 'Bar')");

      let query_foo = values_foo.as_string();
      let query_foo_bar = values_foo_bar.as_string();

      let expected_query_foo = "\
        /* test raw */ \
        /* test raw_before */ \
        VALUES ROW('foo', 'Foo') \
        /* test raw_after */\
      ";
      let expected_query_foo_bar = "\
        /* test raw */ \
        /* test raw_before */ \
        VALUES ROW('foo', 'Foo'), ROW('bar', 'Bar') \
        /* test raw_after */\
      ";

      assert_eq!(expected_query_foo, query_foo);
      assert_eq!(expected_query_foo_bar, query_foo_bar);
    }
  }

  #[test]
  fn values_builder_should_be_able_to_conditionally_add_clauses() {
    #[cfg(not(feature = "mysql"))]
    {
      let mut values = sql::Values::new().values("('foo', 'Foo')");

      if true {
        values = values.values("('bar', 'Bar')");
      }

      let query = values.as_string();
      let expected_query = "VALUES ('foo', 'Foo'), ('bar', 'Bar')";

      assert_eq!(expected_query, query);
    }

    #[cfg(feature = "mysql")]
    {
      let mut values = sql::Values::new().row("('foo', 'Foo')");

      if true {
        values = values.row("('bar', 'Bar')");
      }

      let query = values.as_string();
      let expected_query = "VALUES ROW('foo', 'Foo'), ROW('bar', 'Bar')";

      assert_eq!(expected_query, query);
    }
  }

  #[test]
  fn values_builder_should_be_composable() {
    #[cfg(not(feature = "mysql"))]
    {
      fn value_foo(values: sql::Values) -> sql::Values {
        values.values("('foo', 'Foo')")
      }

      fn value_bar(values: sql::Values) -> sql::Values {
        values.values("('bar', 'Bar')")
      }

      fn as_string(values: sql::Values) -> String {
        values.as_string()
      }

      let query = Some(sql::Values::new())
        .map(value_foo)
        .map(value_bar)
        .map(as_string)
        .unwrap();

      let expected_query = "VALUES ('foo', 'Foo'), ('bar', 'Bar')";

      assert_eq!(expected_query, query);
    }

    #[cfg(feature = "mysql")]
    {
      fn value_foo(values: sql::Values) -> sql::Values {
        values.row("('foo', 'Foo')")
      }

      fn value_bar(values: sql::Values) -> sql::Values {
        values.row("('bar', 'Bar')")
      }

      fn as_string(values: sql::Values) -> String {
        values.as_string()
      }

      let query = Some(sql::Values::new())
        .map(value_foo)
        .map(value_bar)
        .map(as_string)
        .unwrap();

      let expected_query = "VALUES ROW('foo', 'Foo'), ROW('bar', 'Bar')";

      assert_eq!(expected_query, query);
    }
  }
}

mod builder_methods {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_new_should_initialize_as_empty_string() {
    let query = sql::Values::new().as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_as_string_should_convert_the_current_state_into_string() {
    let query = sql::Values::new().as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_debug_should_print_at_console_in_a_human_readable_format() {
    #[cfg(not(feature = "mysql"))]
    {
      let query = sql::Values::new()
        .values("(1, 'one')")
        .values("(2, 'two')")
        .debug()
        .as_string();
      let expected_query = "VALUES (1, 'one'), (2, 'two')";

      assert_eq!(expected_query, query);
    }

    #[cfg(feature = "mysql")]
    {
      let query = sql::Values::new()
        .row("(1, 'one')")
        .row("(2, 'two')")
        .debug()
        .as_string();
      let expected_query = "VALUES ROW(1, 'one'), ROW(2, 'two')";

      assert_eq!(expected_query, query);
    }
  }

  #[test]
  fn method_print_should_print_in_one_line_the_current_state_of_builder() {
    #[cfg(not(feature = "mysql"))]
    {
      let query = sql::Values::new()
        .values("(1, 'one')")
        .values("(2, 'two')")
        .print()
        .as_string();
      let expected_query = "VALUES (1, 'one'), (2, 'two')";

      assert_eq!(expected_query, query);
    }

    #[cfg(feature = "mysql")]
    {
      let query = sql::Values::new()
        .row("(1, 'one')")
        .row("(2, 'two')")
        .print()
        .as_string();
      let expected_query = "VALUES ROW(1, 'one'), ROW(2, 'two')";

      assert_eq!(expected_query, query);
    }
  }

  #[test]
  fn method_raw_should_add_raw_sql_on_top_of_the_command() {
    #[cfg(not(feature = "mysql"))]
    {
      let query = sql::Values::new()
        .raw("/* the values command */")
        .values("(1, 'one')")
        .as_string();
      let expected_query = "/* the values command */ VALUES (1, 'one')";

      assert_eq!(expected_query, query);
    }

    #[cfg(feature = "mysql")]
    {
      let query = sql::Values::new()
        .raw("/* the values command */")
        .row("(1, 'one')")
        .as_string();
      let expected_query = "/* the values command */ VALUES ROW(1, 'one')";

      assert_eq!(expected_query, query);
    }
  }

  #[test]
  fn method_raw_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Values::new().raw("/* raw one */").raw("/* raw two */").as_string();
    let expected_query = "/* raw one */ /* raw two */";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::Values::new().raw("").raw("/* raw one */").raw("").as_string();
    let expected_query = "/* raw one */";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_be_the_first_to_be_concatenated() {
    #[cfg(not(feature = "mysql"))]
    {
      let query = sql::Values::new()
        .values("(1, 'one')")
        .raw("insert into my_table(num, txt)")
        .as_string();
      let expected_query = "insert into my_table(num, txt) VALUES (1, 'one')";

      assert_eq!(expected_query, query);
    }

    #[cfg(feature = "mysql")]
    {
      let query = sql::Values::new()
        .row("(1, 'one')")
        .raw("insert into my_table(num, txt)")
        .as_string();
      let expected_query = "insert into my_table(num, txt) VALUES ROW(1, 'one')";

      assert_eq!(expected_query, query);
    }
  }

  #[test]
  fn method_raw_should_trim_space_of_the_argument() {
    let query = sql::Values::new().raw("  /* raw one */  ").as_string();
    let expected_query = "/* raw one */";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Values::new()
      .raw("/* should not be repeat */")
      .raw("/* should not be repeat */")
      .as_string();
    let expected_query = "/* should not be repeat */";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_trim_space_of_the_argument() {
    let query = sql::Values::new()
      .raw_after(sql::ValuesClause::Values, "  /* raw one */  ")
      .as_string();
    let expected_query = "/* raw one */";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_trim_space_of_the_argument() {
    let query = sql::Values::new()
      .raw_before(sql::ValuesClause::Values, "  /* raw one */  ")
      .as_string();
    let expected_query = "/* raw one */";

    assert_eq!(expected_query, query);
  }
}
