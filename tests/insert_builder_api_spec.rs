use sql_query_builder::{InsertBuilder, InsertClause};

mod builder_methods {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_as_string_should_convert_the_current_state_into_string() {
    let query = InsertBuilder::new().as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_debug_should_print_at_console_in_a_human_readable_format() {
    let query = InsertBuilder::new().insert_into("users").debug().as_string();
    let expected_query = "INSERT INTO users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_new_should_initialize_as_empty_string() {
    let query = InsertBuilder::new().as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_print_should_print_in_one_line_the_current_state_of_builder() {
    let query = InsertBuilder::new().insert_into("users").print().as_string();
    let expected_query = "INSERT INTO users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_add_raw_sql() {
    let query = InsertBuilder::new()
      .raw("insert into address (state, country)")
      .as_string();
    let expected_query = "insert into address (state, country)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_accumulate_values_on_consecutive_calls() {
    let query = InsertBuilder::new()
      .raw("/* raw statement */")
      .raw("insert into address (state, country)")
      .as_string();
    let expected_query = "/* raw statement */ insert into address (state, country)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_be_the_first_to_be_concatenated() {
    let query = InsertBuilder::new()
      .raw("insert into address (state, country)")
      .values("('foo', 'bar')")
      .as_string();
    let expected_query = "insert into address (state, country) VALUES ('foo', 'bar')";

    assert_eq!(query, expected_query);
  }
}

mod insert_into_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_insert_into_should_add_a_insert_into_clause() {
    let query = InsertBuilder::new().insert_into("users").as_string();
    let expected_query = "INSERT INTO users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_insert_into_should_override_value_on_consecutive_calls() {
    let query = InsertBuilder::new()
      .insert_into("users")
      .insert_into("orders")
      .as_string();
    let expected_query = "INSERT INTO orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_insert_into_clause() {
    let query = InsertBuilder::new()
      .raw_before(InsertClause::InsertInto, "/* insert into users */")
      .insert_into("users")
      .as_string();
    let expected_query = "/* insert into users */ INSERT INTO users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_insert_into_clause() {
    let query = InsertBuilder::new()
      .insert_into("users (name)")
      .raw_after(InsertClause::InsertInto, "values ('foo')")
      .as_string();
    let expected_query = "INSERT INTO users (name) values ('foo')";

    assert_eq!(query, expected_query);
  }
}

mod values_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_values_should_add_a_values_clause() {
    let query = InsertBuilder::new().values("('foo', 'Foo')").as_string();
    let expected_query = "VALUES ('foo', 'Foo')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_values_should_accumulate_values_on_consecutive_calls() {
    let query = InsertBuilder::new()
      .values("('foo', 'Foo')")
      .values("('bar', 'Bar')")
      .as_string();
    let expected_query = "VALUES ('foo', 'Foo'), ('bar', 'Bar')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_values_clause() {
    let query = InsertBuilder::new()
      .raw_before(InsertClause::Values, "insert into users (login, name)")
      .values("('foo', 'Foo')")
      .as_string();
    let expected_query = "insert into users (login, name) VALUES ('foo', 'Foo')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_values_clause() {
    let query = InsertBuilder::new()
      .values("('baz', 'Baz')")
      .raw_after(InsertClause::Values, ", ('foo', 'Foo')")
      .as_string();
    let expected_query = "VALUES ('baz', 'Baz') , ('foo', 'Foo')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_values_should_be_after_insert_into_clause() {
    let query = InsertBuilder::new()
      .values("('bar', 'Bar')")
      .insert_into("users (login, name)")
      .as_string();
    let expected_query = "INSERT INTO users (login, name) VALUES ('bar', 'Bar')";

    assert_eq!(query, expected_query);
  }
}

mod overriding_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_overriding_should_add_a_overriding_clause() {
    let query = InsertBuilder::new().overriding("user value").as_string();
    let expected_query = "OVERRIDING user value";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_overriding_should_override_value_on_consecutive_calls() {
    let query = InsertBuilder::new()
      .overriding("user value")
      .overriding("system value")
      .as_string();
    let expected_query = "OVERRIDING system value";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_overriding_clause() {
    let query = InsertBuilder::new()
      .raw_before(InsertClause::Overriding, "insert into users (login, name)")
      .overriding("system value")
      .as_string();
    let expected_query = "insert into users (login, name) OVERRIDING system value";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_overriding_clause() {
    let query = InsertBuilder::new()
      .overriding("user value")
      .raw_after(InsertClause::Overriding, "values ('baz', 'Baz')")
      .as_string();
    let expected_query = "OVERRIDING user value values ('baz', 'Baz')";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_overriding_should_be_after_insert_into_clause() {
    let query = InsertBuilder::new()
      .overriding("system value")
      .insert_into("users (login, name)")
      .as_string();
    let expected_query = "INSERT INTO users (login, name) OVERRIDING system value";

    assert_eq!(query, expected_query);
  }
}
