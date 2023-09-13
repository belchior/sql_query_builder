use sql_query_builder as sql;

mod builder_methods {
  use super::*;
  use pretty_assertions::assert_eq;

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
      .raw("insert into address (state, country)")
      .as_string();
    let expected_query = "insert into address (state, country)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Insert::new()
      .raw("/* raw statement */")
      .raw("insert into address (state, country)")
      .as_string();
    let expected_query = "/* raw statement */ insert into address (state, country)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_be_the_first_to_be_concatenated() {
    let query = sql::Insert::new()
      .raw("insert into address (state, country)")
      .values("('foo', 'bar')")
      .as_string();
    let expected_query = "insert into address (state, country) VALUES ('foo', 'bar')";

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

mod insert_into_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_insert_into_should_add_a_insert_into_clause() {
    let query = sql::Insert::new().insert_into("users (login, name)").as_string();
    let expected_query = "INSERT INTO users (login, name)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_insert_into_should_override_value_on_consecutive_calls() {
    let query = sql::Insert::new()
      .insert_into("users (login, name)")
      .insert_into("orders (product_name, price)")
      .as_string();
    let expected_query = "INSERT INTO orders (product_name, price)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_insert_into_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().insert_into("  users (name)  ").as_string();
    let expected_query = "INSERT INTO users (name)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_insert_into_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::InsertInto, "/* insert into users */")
      .insert_into("users")
      .as_string();
    let expected_query = "/* insert into users */ INSERT INTO users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_insert_into_clause() {
    let query = sql::Insert::new()
      .insert_into("users (name)")
      .raw_after(sql::InsertClause::InsertInto, "values ('foo')")
      .as_string();
    let expected_query = "INSERT INTO users (name) values ('foo')";

    assert_eq!(query, expected_query);
  }
}

#[cfg(not(feature = "sqlite"))]
mod overriding_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_overriding_should_add_a_overriding_clause() {
    let query = sql::Insert::new().overriding("user value").as_string();
    let expected_query = "OVERRIDING user value";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_overriding_should_override_value_on_consecutive_calls() {
    let query = sql::Insert::new()
      .overriding("user value")
      .overriding("system value")
      .as_string();
    let expected_query = "OVERRIDING system value";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_overrinding_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().overriding("  system value  ").as_string();
    let expected_query = "OVERRIDING system value";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_overriding_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::Overriding, "insert into users (login, name)")
      .overriding("system value")
      .as_string();
    let expected_query = "insert into users (login, name) OVERRIDING system value";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_overriding_clause() {
    let query = sql::Insert::new()
      .overriding("user value")
      .raw_after(sql::InsertClause::Overriding, "values ('baz', 'Baz')")
      .as_string();
    let expected_query = "OVERRIDING user value values ('baz', 'Baz')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_overriding_should_be_after_insert_into_clause() {
    let query = sql::Insert::new()
      .overriding("system value")
      .insert_into("users (login, name)")
      .as_string();
    let expected_query = "INSERT INTO users (login, name) OVERRIDING system value";

    assert_eq!(query, expected_query);
  }
}

mod on_conflict_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_on_conflict_should_add_a_on_conflict_clause() {
    let query = sql::Insert::new().on_conflict("DO NOTHING").as_string();
    let expected_query = "ON CONFLICT DO NOTHING";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_on_conflict_should_override_value_on_consecutive_calls() {
    let query = sql::Insert::new()
      .on_conflict("do nothing")
      .on_conflict("on constraint users_name_key do nothing")
      .as_string();
    let expected_query = "ON CONFLICT on constraint users_name_key do nothing";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_on_conflict_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().on_conflict("  DO NOTHING  ").as_string();
    let expected_query = "ON CONFLICT DO NOTHING";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_on_conflict_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::OnConflict, "values ('foo', 'Foo')")
      .on_conflict("DO NOTHING")
      .as_string();
    let expected_query = "values ('foo', 'Foo') ON CONFLICT DO NOTHING";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_on_conflict_clause() {
    let query = sql::Insert::new()
      .on_conflict("do nothing")
      .raw_after(sql::InsertClause::OnConflict, "/* raw after */")
      .as_string();
    let expected_query = "ON CONFLICT do nothing /* raw after */";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_on_conflict_should_be_after_values_clause() {
    let query = sql::Insert::new()
      .values("('foo', 'Foo')")
      .on_conflict("DO NOTHING")
      .as_string();
    let expected_query = "VALUES ('foo', 'Foo') ON CONFLICT DO NOTHING";

    assert_eq!(query, expected_query);
  }
}

mod select_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_select_should_add_a_select_clause() {
    let query = sql::Insert::new()
      .insert_into("users (login, name)")
      .select(
        sql::Select::new()
          .select("login, name")
          .from("users_bk")
          .where_clause("active = true"),
      )
      .as_string();

    let expected_query = "\
      INSERT INTO users (login, name) \
      SELECT login, name \
      FROM users_bk \
      WHERE active = true\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_select_should_override_value_on_consecutive_calls() {
    let query = sql::Insert::new()
      .insert_into("users")
      .select(sql::Select::new().select("login, name"))
      .select(sql::Select::new().select("*"))
      .as_string();

    let expected_query = "INSERT INTO users SELECT *";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_select_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::Select, "insert into users")
      .select(sql::Select::new().select("*"))
      .as_string();

    let expected_query = "insert into users SELECT *";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_select_clause() {
    let query = sql::Insert::new()
      .insert_into("users")
      .select(sql::Select::new().select("*"))
      .raw_after(sql::InsertClause::Select, "from users_bk")
      .as_string();
    let expected_query = "INSERT INTO users SELECT * from users_bk";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_select_should_be_after_insert_into_clause() {
    let query = sql::Insert::new()
      .select(sql::Select::new().select("login, name"))
      .insert_into("users (login, name)")
      .as_string();
    let expected_query = "INSERT INTO users (login, name) SELECT login, name";

    assert_eq!(query, expected_query);
  }
}

mod values_clause {
  use super::*;
  use pretty_assertions::assert_eq;

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
