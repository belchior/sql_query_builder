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

    assert_eq!(expected_query, query);
  }

  #[test]
  fn insert_builder_should_be_debuggable() {
    let insert = sql::Insert::new()
      .insert_into("users(login, name)")
      .values("('foo', 'Foo')");

    println!("{:?}", insert);

    let expected_query = "INSERT INTO users(login, name) VALUES ('foo', 'Foo')";
    let query = insert.as_string();

    assert_eq!(expected_query, query);
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

    assert_eq!(expected_query, query);
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

    assert_eq!(expected_query, query);
  }

  #[cfg(not(any(feature = "sqlite", feature = "mysql")))]
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

    assert_eq!(expected_query, query);
  }
}

mod builder_methods {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_as_string_should_convert_the_current_state_into_string() {
    let query = sql::Insert::new().as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_debug_should_print_at_console_in_a_human_readable_format() {
    let query = sql::Insert::new().insert_into("users").debug().as_string();
    let expected_query = "INSERT INTO users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_new_should_initialize_as_empty_string() {
    let query = sql::Insert::new().as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_print_should_print_in_one_line_the_current_state_of_builder() {
    let query = sql::Insert::new().insert_into("users").print().as_string();
    let expected_query = "INSERT INTO users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_add_raw_sql() {
    let query = sql::Insert::new()
      .raw("insert into addresses (state, country)")
      .as_string();
    let expected_query = "insert into addresses (state, country)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Insert::new()
      .raw("/* raw statement */")
      .raw("insert into addresses (state, country)")
      .as_string();
    let expected_query = "/* raw statement */ insert into addresses (state, country)";

    assert_eq!(expected_query, query);
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

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_be_the_first_to_be_concatenated() {
    let query = sql::Insert::new()
      .raw("insert into addresses (state, country)")
      .values("('foo', 'bar')")
      .as_string();
    let expected_query = "insert into addresses (state, country) VALUES ('foo', 'bar')";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().raw("  insert users (name)  ").as_string();
    let expected_query = "insert users (name)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Insert::new()
      .raw("insert users (name)")
      .raw("insert users (name)")
      .as_string();
    let expected_query = "insert users (name)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_trim_space_of_the_argument() {
    let query = sql::Insert::new()
      .raw_after(sql::InsertClause::InsertInto, "  values ('Foo')  ")
      .as_string();
    let expected_query = "values ('Foo')";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_trim_space_of_the_argument() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::Values, "  insert users (name)  ")
      .as_string();
    let expected_query = "insert users (name)";

    assert_eq!(expected_query, query);
  }
}

mod insert_into_method {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_insert_into_should_add_the_insert_into_clause() {
    let query = sql::Insert::new().insert_into("users (login, name)").as_string();
    let expected_query = "INSERT INTO users (login, name)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_insert_into_should_override_value_on_consecutive_calls() {
    let query = sql::Insert::new()
      .insert_into("users (login, name)")
      .insert_into("orders (product_name, price)")
      .as_string();
    let expected_query = "INSERT INTO orders (product_name, price)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_insert_into_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().insert_into("  users (name)  ").as_string();
    let expected_query = "INSERT INTO users (name)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_insert_into_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::InsertInto, "/* insert into users */")
      .insert_into("users")
      .as_string();
    let expected_query = "/* insert into users */ INSERT INTO users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_insert_into_clause() {
    let query = sql::Insert::new()
      .insert_into("users (name)")
      .raw_after(sql::InsertClause::InsertInto, "values ('foo')")
      .as_string();
    let expected_query = "INSERT INTO users (name) values ('foo')";

    assert_eq!(expected_query, query);
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod on_conflict_method {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_on_conflict_should_add_a_on_conflict_clause() {
    let query = sql::Insert::new().on_conflict("DO NOTHING").as_string();
    let expected_query = "ON CONFLICT DO NOTHING";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_on_conflict_should_override_value_on_consecutive_calls() {
    let query = sql::Insert::new()
      .on_conflict("do nothing")
      .on_conflict("on constraint users_name_key do nothing")
      .as_string();
    let expected_query = "ON CONFLICT on constraint users_name_key do nothing";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_on_conflict_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().on_conflict("  DO NOTHING  ").as_string();
    let expected_query = "ON CONFLICT DO NOTHING";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_on_conflict_should_be_after_values_clause() {
    let query = sql::Insert::new()
      .values("('foo', 'Foo')")
      .on_conflict("DO NOTHING")
      .as_string();
    let expected_query = "VALUES ('foo', 'Foo') ON CONFLICT DO NOTHING";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_on_conflict_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::OnConflict, "values ('foo', 'Foo')")
      .on_conflict("DO NOTHING")
      .as_string();
    let expected_query = "values ('foo', 'Foo') ON CONFLICT DO NOTHING";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_on_conflict_clause() {
    let query = sql::Insert::new()
      .on_conflict("do nothing")
      .raw_after(sql::InsertClause::OnConflict, "/* raw after */")
      .as_string();
    let expected_query = "ON CONFLICT do nothing /* raw after */";

    assert_eq!(expected_query, query);
  }
}

#[cfg(not(any(feature = "sqlite", feature = "mysql")))]
mod overriding_method {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_overriding_should_add_the_overriding_clause() {
    let query = sql::Insert::new().overriding("user value").as_string();
    let expected_query = "OVERRIDING user value";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_overriding_should_override_value_on_consecutive_calls() {
    let query = sql::Insert::new()
      .overriding("user value")
      .overriding("system value")
      .as_string();
    let expected_query = "OVERRIDING system value";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_overrinding_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().overriding("  system value  ").as_string();
    let expected_query = "OVERRIDING system value";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_overriding_should_be_after_insert_into_clause() {
    let query = sql::Insert::new()
      .overriding("system value")
      .insert_into("users (login, name)")
      .as_string();
    let expected_query = "INSERT INTO users (login, name) OVERRIDING system value";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_overriding_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::Overriding, "insert into users (login, name)")
      .overriding("system value")
      .as_string();
    let expected_query = "insert into users (login, name) OVERRIDING system value";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_overriding_clause() {
    let query = sql::Insert::new()
      .overriding("user value")
      .raw_after(sql::InsertClause::Overriding, "values ('baz', 'Baz')")
      .as_string();
    let expected_query = "OVERRIDING user value values ('baz', 'Baz')";

    assert_eq!(expected_query, query);
  }
}

#[cfg(feature = "sqlite")]
mod insert_or_method {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_insert_or_should_add_the_insert_or_clause() {
    let query = sql::Insert::new()
      .insert_or("ABORT INTO users (login, name)")
      .as_string();
    let expected_query = "INSERT OR ABORT INTO users (login, name)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_insert_or_should_override_value_on_consecutive_calls() {
    let query = sql::Insert::new()
      .insert_or("FAIL INTO users (login, name)")
      .insert_or("FAIL INTO orders (product_name, price)")
      .as_string();
    let expected_query = "INSERT OR FAIL INTO orders (product_name, price)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_insert_or_should_not_add_clause_when_argument_is_empty() {
    let query = sql::Insert::new().insert_or("").as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_insert_or_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().insert_or("  IGNORE INTO users (name)  ").as_string();
    let expected_query = "INSERT OR IGNORE INTO users (name)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_insert_or_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::InsertOr, "/* insert or replace */")
      .insert_or("REPLACE INTO users (login)")
      .as_string();
    let expected_query = "/* insert or replace */ INSERT OR REPLACE INTO users (login)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_insert_or_clause() {
    let query = sql::Insert::new()
      .insert_or("ROLLBACK INTO users (name)")
      .raw_after(sql::InsertClause::InsertOr, "values ('foo')")
      .as_string();
    let expected_query = "INSERT OR ROLLBACK INTO users (name) values ('foo')";

    assert_eq!(expected_query, query);
  }
}

#[cfg(feature = "sqlite")]
mod sqlite_insert_variances {
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
    assert_eq!(expected_query, query);

    let query = sql::Insert::new()
      .replace_into("users (login, name)")
      .insert_into("users (login, name)")
      .insert_or("ABORT INTO users (login, name)")
      .as_string();
    let expected_query = "INSERT OR ABORT INTO users (login, name)";
    assert_eq!(expected_query, query);

    let query = sql::Insert::new()
      .insert_or("ABORT INTO users (login, name)")
      .replace_into("users (login, name)")
      .insert_into("users (login, name)")
      .as_string();
    let expected_query = "INSERT INTO users (login, name)";
    assert_eq!(expected_query, query);
  }
}

#[cfg(not(feature = "mysql"))]
mod default_values_method {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_default_values_should_add_a_default_values_clause() {
    let query = sql::Insert::new().default_values().as_string();
    let expected_query = "DEFAULT VALUES";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_default_values_should_do_nothing_on_consecutive_calls() {
    let query = sql::Insert::new().default_values().default_values().as_string();
    let expected_query = "DEFAULT VALUES";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_default_values_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::DefaultValues, "insert into users (login, name)")
      .default_values()
      .as_string();
    let expected_query = "insert into users (login, name) DEFAULT VALUES";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_values_clause() {
    let query = sql::Insert::new()
      .default_values()
      .raw_after(sql::InsertClause::DefaultValues, "-- default values test")
      .as_string();
    let expected_query = "DEFAULT VALUES -- default values test";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_default_values_should_be_after_insert_into_clause() {
    let query = sql::Insert::new()
      .default_values()
      .insert_into("users (login, name)")
      .as_string();
    let expected_query = "INSERT INTO users (login, name) DEFAULT VALUES";

    assert_eq!(expected_query, query);
  }
}

#[cfg(feature = "mysql")]
mod column_method {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_column_should_define_the_columns_of_the_table() {
    let query = sql::Insert::new().column("login").as_string();
    let expected_query = "(login)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_column_should_not_add_the_column_part_when_has_no_column_name_at_list() {
    let query = sql::Insert::new().column("").as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_column_should_accumulate_column_names_on_consecutive_calls() {
    let query = sql::Insert::new().column("login").column("name").as_string();

    let expected_query = "(login, name)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_column_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::Insert::new().column("").column("login").column("").as_string();

    let expected_query = "(login)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_column_should_not_accumulate_columns_with_the_same_content() {
    let query = sql::Insert::new().column("login").column("login").as_string();
    let expected_query = "(login)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_column_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().column("  login  ").as_string();
    let expected_query = "(login)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn parameter_column_should_be_after_insert_clause() {
    let query = sql::Insert::new().column("name").insert("LOW_PRIORITY").as_string();
    let expected_query = "INSERT LOW_PRIORITY (name)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn parameter_column_should_be_after_into_clause() {
    let query = sql::Insert::new().column("name").into("users").as_string();
    let expected_query = "INTO users (name)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn parameter_column_should_be_after_partition_clause() {
    let query = sql::Insert::new().column("name").partition("p1").as_string();
    let expected_query = "PARTITION (p1) (name)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_column_parameter() {
    let query = sql::Insert::new()
      .column("name")
      .raw_after(sql::InsertClause::Column, "/* uncommon clause */")
      .as_string();

    let expected_query = "(name) /* uncommon clause */";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_column_parameter() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::Column, "into users")
      .column("name")
      .as_string();

    let expected_query = "into users (name)";

    assert_eq!(expected_query, query);
  }
}

#[cfg(feature = "mysql")]
mod insert_method {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_insert_should_add_the_insert_clause() {
    let query = sql::Insert::new().insert("LOW_PRIORITY").as_string();
    let expected_query = "INSERT LOW_PRIORITY";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_insert_should_override_the_current_value() {
    let query = sql::Insert::new()
      .insert("LOW_PRIORITY")
      .insert("HIGH_PRIORITY")
      .as_string();
    let expected_query = "INSERT HIGH_PRIORITY";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_insert_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().insert("  LOW_PRIORITY  ").as_string();
    let expected_query = "INSERT LOW_PRIORITY";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_insert_should_be_before_into_clause() {
    let query = sql::Insert::new().insert("LOW_PRIORITY").into("users").as_string();
    let expected_query = "INSERT LOW_PRIORITY INTO users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_insert_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::Insert, "/* comment */")
      .insert("IGNORE")
      .as_string();
    let expected_query = "/* comment */ INSERT IGNORE";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_insert_clause() {
    let query = sql::Insert::new()
      .insert("IGNORE")
      .raw_after(sql::InsertClause::Insert, "into users")
      .as_string();
    let expected_query = "INSERT IGNORE into users";

    assert_eq!(expected_query, query);
  }
}

#[cfg(feature = "mysql")]
mod into_method {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_into_should_add_the_into_clause() {
    let query = sql::Insert::new().into("users").as_string();
    let expected_query = "INTO users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_into_should_override_the_current_value() {
    let query = sql::Insert::new().into("users").into("employees").as_string();
    let expected_query = "INTO employees";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_into_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().into("  users  ").as_string();
    let expected_query = "INTO users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_into_should_be_after_insert_clause() {
    let query = sql::Insert::new().insert("LOW_PRIORITY").into("users").as_string();
    let expected_query = "INSERT LOW_PRIORITY INTO users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_into_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::Into, "order by id")
      .into("users")
      .as_string();
    let expected_query = "order by id INTO users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_into_clause() {
    let query = sql::Insert::new()
      .into("users")
      .raw_after(sql::InsertClause::Into, "/* uncommon argument */")
      .as_string();
    let expected_query = "INTO users /* uncommon argument */";

    assert_eq!(expected_query, query);
  }
}

#[cfg(feature = "mysql")]
mod on_duplicate_key_update_method {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_on_duplicate_key_update_should_add_a_on_duplicate_key_update_clause() {
    let query = sql::Insert::new().on_duplicate_key_update("login = 'Foo'").as_string();
    let expected_query = "ON DUPLICATE KEY UPDATE login = 'Foo'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_on_duplicate_key_update_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Insert::new()
      .on_duplicate_key_update("login = 'foo'")
      .on_duplicate_key_update("name = 'Foo'")
      .as_string();
    let expected_query = "ON DUPLICATE KEY UPDATE login = 'foo', name = 'Foo'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_on_duplicate_key_update_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::Insert::new()
      .on_duplicate_key_update("")
      .on_duplicate_key_update("name = 'Foo'")
      .on_duplicate_key_update("")
      .as_string();
    let expected_query = "ON DUPLICATE KEY UPDATE name = 'Foo'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_on_duplicate_key_update_should_trim_space_of_the_argument() {
    let query = sql::Insert::new()
      .on_duplicate_key_update("  name = 'Bar'  ")
      .as_string();
    let expected_query = "ON DUPLICATE KEY UPDATE name = 'Bar'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_on_duplicate_key_update_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Insert::new()
      .on_duplicate_key_update("name = 'Bar'")
      .on_duplicate_key_update("name = 'Bar'")
      .as_string();
    let expected_query = "ON DUPLICATE KEY UPDATE name = 'Bar'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_on_duplicate_key_update_should_be_after_into_clause() {
    let query = sql::Insert::new()
      .on_duplicate_key_update("name = 'Bar'")
      .into("users")
      .as_string();
    let expected_query = "INTO users ON DUPLICATE KEY UPDATE name = 'Bar'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_on_duplicate_key_update_should_be_after_partition_clause() {
    let query = sql::Insert::new()
      .on_duplicate_key_update("name = 'Bar'")
      .partition("p1")
      .as_string();
    let expected_query = "PARTITION (p1) ON DUPLICATE KEY UPDATE name = 'Bar'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_on_duplicate_key_update_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::OnDuplicateKeyUpdate, "insert into users")
      .on_duplicate_key_update("login = 'Bar'")
      .as_string();
    let expected_query = "insert into users ON DUPLICATE KEY UPDATE login = 'Bar'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_on_duplicate_key_update_clause() {
    let query = sql::Insert::new()
      .on_duplicate_key_update("name = 'Bar'")
      .raw_after(sql::InsertClause::OnDuplicateKeyUpdate, ", login = 'bar'")
      .as_string();
    let expected_query = "ON DUPLICATE KEY UPDATE name = 'Bar' , login = 'bar'";

    assert_eq!(expected_query, query);
  }
}

#[cfg(feature = "mysql")]
mod mysql_insert_variances {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn methods_insert_into_column_partition() {
    let query = sql::Insert::new()
      .insert("low_priority")
      .into("users")
      .partition("p1")
      .column("name")
      .as_string();
    let expected_query = "INSERT low_priority INTO users PARTITION (p1) (name)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_insert_into_and_method_insert_the_last_call_should_override_the_previous_one() {
    let query = sql::Insert::new()
      .insert_into("users (name)")
      .insert("low_priority")
      .as_string();
    let expected_query = "INSERT low_priority";

    assert_eq!(expected_query, query);

    let query = sql::Insert::new()
      .insert("low_priority")
      .insert_into("users (name)")
      .as_string();
    let expected_query = "INSERT INTO users (name)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_insert_into_and_method_into_the_last_call_should_override_the_previous_one() {
    let query = sql::Insert::new()
      .insert_into("users (name)")
      .into("employees")
      .as_string();
    let expected_query = "INTO employees";

    assert_eq!(expected_query, query);

    let query = sql::Insert::new()
      .into("employees")
      .insert_into("users (name)")
      .as_string();
    let expected_query = "INSERT INTO users (name)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_insert_into_and_method_partition_the_last_call_should_override_the_previous_one() {
    let query = sql::Insert::new()
      .insert_into("users (name)")
      .partition("p1")
      .as_string();
    let expected_query = "PARTITION (p1)";

    assert_eq!(expected_query, query);

    let query = sql::Insert::new()
      .partition("p1")
      .insert_into("users (name)")
      .as_string();
    let expected_query = "INSERT INTO users (name)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_insert_into_and_method_column_the_last_call_should_override_the_previous_one() {
    let query = sql::Insert::new()
      .insert_into("users (name)")
      .column("login")
      .as_string();
    let expected_query = "(login)";

    assert_eq!(expected_query, query);

    let query = sql::Insert::new()
      .column("login")
      .insert_into("users (name)")
      .as_string();
    let expected_query = "INSERT INTO users (name)";

    assert_eq!(expected_query, query);
  }
}
