use sql_query_builder::{UpdateBuilder, UpdateClause};

mod builder_methods {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_as_string_should_convert_the_current_state_into_string() {
    let query = UpdateBuilder::new().as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_debug_should_print_at_console_in_a_human_readable_format() {
    let query = UpdateBuilder::new().update("users").debug().as_string();
    let expected_query = "UPDATE users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_new_should_initialize_as_empty_string() {
    let query = UpdateBuilder::new().as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_print_should_print_in_one_line_the_current_state_of_builder() {
    let query = UpdateBuilder::new().update("users").print().as_string();
    let expected_query = "UPDATE users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_add_raw_sql() {
    let query = UpdateBuilder::new().raw("update address").as_string();
    let expected_query = "update address";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_accumulate_values_on_consecutive_calls() {
    let query = UpdateBuilder::new()
      .raw("update address")
      .raw("set city = 'Foo'")
      .as_string();
    let expected_query = "update address set city = 'Foo'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_be_the_first_to_be_concatenated() {
    let query = UpdateBuilder::new()
      .raw("update address")
      .set("country = 'Bar'")
      .as_string();
    let expected_query = "update address SET country = 'Bar'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_trim_space_of_the_argument() {
    let query = UpdateBuilder::new().raw("  update users  ").as_string();
    let expected_query = "update users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_trim_space_of_the_argument() {
    let query = UpdateBuilder::new()
      .raw_after(UpdateClause::Update, "  set name = 'Bar'  ")
      .as_string();
    let expected_query = "set name = 'Bar'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_trim_space_of_the_argument() {
    let query = UpdateBuilder::new()
      .raw_before(UpdateClause::Where, "  set name = 'Bar'  ")
      .as_string();
    let expected_query = "set name = 'Bar'";

    assert_eq!(query, expected_query);
  }
}

mod set_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_set_should_add_a_set_clause() {
    let query = UpdateBuilder::new().set("login = 'Foo'").as_string();
    let expected_query = "SET login = 'Foo'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_set_should_accumulate_values_on_consecutive_calls() {
    let query = UpdateBuilder::new()
      .set("login = 'foo'")
      .set("name = 'Foo'")
      .as_string();
    let expected_query = "SET login = 'foo', name = 'Foo'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_set_should_trim_space_of_the_argument() {
    let query = UpdateBuilder::new().set("  name = 'Bar'  ").as_string();
    let expected_query = "SET name = 'Bar'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_set_clause() {
    let query = UpdateBuilder::new()
      .raw_before(UpdateClause::Set, "update users")
      .set("login = 'Bar'")
      .as_string();
    let expected_query = "update users SET login = 'Bar'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_set_clause() {
    let query = UpdateBuilder::new()
      .set("name = 'Bar'")
      .raw_after(UpdateClause::Set, ", login = 'bar'")
      .as_string();
    let expected_query = "SET name = 'Bar' , login = 'bar'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_set_should_be_after_update_clause() {
    let query = UpdateBuilder::new().set("name = 'Bar'").update("users").as_string();
    let expected_query = "UPDATE users SET name = 'Bar'";

    assert_eq!(query, expected_query);
  }
}

mod update_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_update_should_add_a_update_clause() {
    let query = UpdateBuilder::new().update("users").as_string();
    let expected_query = "UPDATE users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_update_should_override_value_on_consecutive_calls() {
    let query = UpdateBuilder::new().update("users").update("orders").as_string();
    let expected_query = "UPDATE orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_update_should_trim_space_of_the_argument() {
    let query = UpdateBuilder::new().update("  orders  ").as_string();
    let expected_query = "UPDATE orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_update_clause() {
    let query = UpdateBuilder::new()
      .raw_before(UpdateClause::Update, "/* update users */")
      .update("users")
      .as_string();
    let expected_query = "/* update users */ UPDATE users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_update_clause() {
    let query = UpdateBuilder::new()
      .update("users")
      .raw_after(UpdateClause::Update, "set login = 'foo'")
      .as_string();
    let expected_query = "UPDATE users set login = 'foo'";

    assert_eq!(query, expected_query);
  }
}

mod where_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_where_should_add_the_where_clause() {
    let query = UpdateBuilder::new().where_clause("id = $1").as_string();
    let expected_query = "WHERE id = $1";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_where_should_accumulate_values_on_consecutive_calls() {
    let query = UpdateBuilder::new()
      .where_clause("id = $1")
      .where_clause("status = 'pending'")
      .as_string();
    let expected_query = "WHERE id = $1 AND status = 'pending'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_where_should_be_after_set_clause() {
    let query = UpdateBuilder::new()
      .set("name = $1")
      .where_clause("login = $2")
      .as_string();
    let expected_query = "SET name = $1 WHERE login = $2";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_where_should_trim_space_of_the_argument() {
    let query = UpdateBuilder::new().where_clause("  id = $1  ").as_string();
    let expected_query = "WHERE id = $1";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_where_clause() {
    let query = UpdateBuilder::new()
      .raw_before(UpdateClause::Where, "set name = $1")
      .where_clause("login = $2")
      .as_string();
    let expected_query = "set name = $1 WHERE login = $2";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_where_clause() {
    let query = UpdateBuilder::new()
      .where_clause("created_at::date >= $1")
      .raw_after(UpdateClause::Where, "and created_at::date < $2")
      .as_string();
    let expected_query = "WHERE created_at::date >= $1 and created_at::date < $2";

    assert_eq!(query, expected_query);
  }
}
