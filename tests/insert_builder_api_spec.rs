use pretty_assertions::assert_eq;
use sql_query_builder::{InsertBuilder, InsertClause};

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

mod insert_into {
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
  fn method_raw_after_should_add_raw_sql_after_insert_into_clause() {
    let query = InsertBuilder::new()
      .insert_into("users (name)")
      .raw_after(InsertClause::InsertInto, "values ('foo')")
      .as_string();
    let expected_query = "INSERT INTO users (name) values ('foo')";

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
}
