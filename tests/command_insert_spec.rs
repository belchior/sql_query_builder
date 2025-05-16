mod builder_features {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn insert_builder_should_be_displayable() {
    let insert = sql::Insert::new()
      .insert_into("users(login, name)")
      .values("('foo', 'Foo')");

    println!("{}", insert);

    let query = insert.as_string();
    let expected_query = "INSERT INTO users(login, name) VALUES ('foo', 'Foo')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn insert_builder_should_be_debuggable() {
    let insert = sql::Insert::new()
      .insert_into("users(login, name)")
      .values("('foo', 'Foo')");

    println!("{:?}", insert);

    let expected_query = "INSERT INTO users(login, name) VALUES ('foo', 'Foo')";
    let query = insert.as_string();

    assert_eq!(query, expected_query);
  }

  #[test]
  fn insert_builder_should_be_cloneable() {
    let insert_foo = sql::Insert::new()
      .raw("/* test raw */")
      .raw_before(sql::InsertClause::Values, "/* test raw_before */")
      .raw_after(sql::InsertClause::Values, "/* test raw_after */")
      .insert_into("users(login, name)")
      .values("('foo', 'Foo')");
    let insert_foo_bar = insert_foo.clone().values("('bar', 'Bar')");
    let query_foo = insert_foo.as_string();
    let query_foo_bar = insert_foo_bar.as_string();

    let expected_query_foo = "\
      /* test raw */ \
      INSERT INTO users(login, name) \
      /* test raw_before */ \
      VALUES ('foo', 'Foo') \
      /* test raw_after */\
    ";
    let expected_query_foo_bar = "\
      /* test raw */ \
      INSERT INTO users(login, name) \
      /* test raw_before */ \
      VALUES ('foo', 'Foo'), ('bar', 'Bar') \
      /* test raw_after */\
    ";

    assert_eq!(query_foo, expected_query_foo);
    assert_eq!(query_foo_bar, expected_query_foo_bar);
  }

  #[test]
  fn insert_builder_should_be_able_to_conditionally_add_clauses() {
    let mut insert = sql::Insert::new()
      .insert_into("users (login, name)")
      .values("('bar', 'Bar')");

    if true {
      insert = insert.values("('foo', 'Foo')");
    }

    let query = insert.as_string();
    let expected_query = "INSERT INTO users (login, name) VALUES ('bar', 'Bar'), ('foo', 'Foo')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn insert_builder_should_be_composable() {
    fn insert(insert: sql::Insert) -> sql::Insert {
      insert.insert_into("users (login, name)")
    }

    fn values(insert: sql::Insert) -> sql::Insert {
      insert
        .values("('foo', 'Foo')")
        .values("('bar', 'Bar')")
        .values("('max', 'Max')")
    }

    fn as_string(insert: sql::Insert) -> String {
      insert.as_string()
    }

    let query = Some(sql::Insert::new()).map(insert).map(values).map(as_string).unwrap();

    let expected_query = "\
      INSERT INTO users (login, name) \
      VALUES \
      ('foo', 'Foo'), \
      ('bar', 'Bar'), \
      ('max', 'Max')\
    ";

    assert_eq!(query, expected_query);
  }

  #[cfg(not(feature = "sqlite"))]
  #[test]
  fn all_standard_clauses_concatenated_in_order() {
    let query = sql::Insert::new()
      .insert_into("users (login, name)")
      .overriding("user value")
      .values("('foo', 'Foo')")
      .as_string();

    let expected_query = "\
      INSERT INTO users (login, name) \
      OVERRIDING user value \
      VALUES ('foo', 'Foo')\
    ";

    assert_eq!(query, expected_query);
  }

  /** This test can fail only at compile time
   * [More context](https://github.com/belchior/sql_query_builder/pull/53)
   */
  #[test]
  fn select_builder_should_impl_send_and_sync() {
    fn assert_impl_sync_send(_builder: impl Sync + Send) {}
    assert_impl_sync_send(sql::Insert::new());
  }
}

mod builder_methods {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_as_string_should_convert_the_current_state_into_string() {
    let query = sql::Insert::new().as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_debug_should_print_at_console_in_a_human_readable_format() {
    let query = sql::Insert::new().insert_into("users").debug().as_string();
    let expected_query = "INSERT INTO users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_new_should_initialize_as_empty_string() {
    let query = sql::Insert::new().as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_print_should_print_in_one_line_the_current_state_of_builder() {
    let query = sql::Insert::new().insert_into("users").print().as_string();
    let expected_query = "INSERT INTO users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_add_raw_sql() {
    let query = sql::Insert::new()
      .raw("insert into addresses (state, country)")
      .as_string();
    let expected_query = "insert into addresses (state, country)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Insert::new()
      .raw("/* raw statement */")
      .raw("insert into addresses (state, country)")
      .as_string();
    let expected_query = "/* raw statement */ insert into addresses (state, country)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::Insert::new()
      .raw("")
      .raw("/* raw statement */")
      .raw("insert into addresses (state, country)")
      .raw("")
      .as_string();
    let expected_query = "/* raw statement */ insert into addresses (state, country)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_be_the_first_to_be_concatenated() {
    let query = sql::Insert::new()
      .raw("insert into addresses (state, country)")
      .values("('foo', 'bar')")
      .as_string();
    let expected_query = "insert into addresses (state, country) VALUES ('foo', 'bar')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().raw("  insert users (name)  ").as_string();
    let expected_query = "insert users (name)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Insert::new()
      .raw("insert users (name)")
      .raw("insert users (name)")
      .as_string();
    let expected_query = "insert users (name)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_trim_space_of_the_argument() {
    let query = sql::Insert::new()
      .raw_after(sql::InsertClause::InsertInto, "  values ('Foo')  ")
      .as_string();
    let expected_query = "values ('Foo')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_trim_space_of_the_argument() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::Values, "  insert users (name)  ")
      .as_string();
    let expected_query = "insert users (name)";

    assert_eq!(query, expected_query);
  }
}

#[cfg(feature = "sqlite")]
mod insert_variances {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn when_more_than_one_insert_variances_are_defined_the_last_one_should_overrides_the_previous_ones() {
    let query = sql::Insert::new()
      .insert_into("users (login, name)")
      .insert_or("ABORT INTO users (login, name)")
      .replace_into("users (login, name)")
      .as_string();
    let expected_query = "REPLACE INTO users (login, name)";
    assert_eq!(query, expected_query);

    let query = sql::Insert::new()
      .replace_into("users (login, name)")
      .insert_into("users (login, name)")
      .insert_or("ABORT INTO users (login, name)")
      .as_string();
    let expected_query = "INSERT OR ABORT INTO users (login, name)";
    assert_eq!(query, expected_query);

    let query = sql::Insert::new()
      .insert_or("ABORT INTO users (login, name)")
      .replace_into("users (login, name)")
      .insert_into("users (login, name)")
      .as_string();
    let expected_query = "INSERT INTO users (login, name)";
    assert_eq!(query, expected_query);
  }
}
