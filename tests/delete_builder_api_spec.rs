use sql_query_builder::{DeleteBuilder, DeleteClause};

mod builder_methods {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_as_string_should_convert_the_current_state_into_string() {
    let query = DeleteBuilder::new().as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_debug_should_print_at_console_in_a_human_readable_format() {
    let query = DeleteBuilder::new().delete_from("users").debug().as_string();
    let expected_query = "DELETE FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_new_should_initialize_as_empty_string() {
    let query = DeleteBuilder::new().as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_print_should_print_in_one_line_the_current_state_of_builder() {
    let query = DeleteBuilder::new().delete_from("users").print().as_string();
    let expected_query = "DELETE FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_add_raw_sql() {
    let query = DeleteBuilder::new().raw("delete from address").as_string();
    let expected_query = "delete from address";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_accumulate_values_on_consecutive_calls() {
    let query = DeleteBuilder::new()
      .raw("delete from address")
      .raw("where city = 'Foo'")
      .as_string();
    let expected_query = "delete from address where city = 'Foo'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_be_the_first_to_be_concatenated() {
    let query = DeleteBuilder::new()
      .raw("delete from address")
      .where_clause("country = 'Bar'")
      .as_string();
    let expected_query = "delete from address WHERE country = 'Bar'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_trim_space_of_the_argument() {
    let query = DeleteBuilder::new().raw("  delete from users  ").as_string();
    let expected_query = "delete from users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_not_accumulate_arguments_with_the_same_content() {
    let query = DeleteBuilder::new()
      .raw("delete from users")
      .raw("delete from users")
      .as_string();
    let expected_query = "delete from users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_trim_space_of_the_argument() {
    let query = DeleteBuilder::new()
      .raw_after(DeleteClause::DeleteFrom, "  where name = 'Bar'  ")
      .as_string();
    let expected_query = "where name = 'Bar'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_trim_space_of_the_argument() {
    let query = DeleteBuilder::new()
      .raw_before(DeleteClause::Where, "  where name = 'Bar'  ")
      .as_string();
    let expected_query = "where name = 'Bar'";

    assert_eq!(query, expected_query);
  }
}

mod and_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_and_should_be_an_alias_to_where_clause() {
    let query = DeleteBuilder::new().and("login = 'foo'").as_string();
    let expected_query = "WHERE login = 'foo'";

    assert_eq!(query, expected_query);
  }
}

mod delete_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_delete_should_add_a_delete_clause() {
    let query = DeleteBuilder::new().delete_from("users").as_string();
    let expected_query = "DELETE FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_delete_should_override_value_on_consecutive_calls() {
    let query = DeleteBuilder::new()
      .delete_from("users")
      .delete_from("orders")
      .as_string();
    let expected_query = "DELETE FROM orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_delete_should_trim_space_of_the_argument() {
    let query = DeleteBuilder::new().delete_from("  orders  ").as_string();
    let expected_query = "DELETE FROM orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_delete_clause() {
    let query = DeleteBuilder::new()
      .raw_before(DeleteClause::DeleteFrom, "/* delete users */")
      .delete_from("users")
      .as_string();
    let expected_query = "/* delete users */ DELETE FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_delete_clause() {
    let query = DeleteBuilder::new()
      .delete_from("users")
      .raw_after(DeleteClause::DeleteFrom, "where login = 'foo'")
      .as_string();
    let expected_query = "DELETE FROM users where login = 'foo'";

    assert_eq!(query, expected_query);
  }
}

mod where_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_where_should_add_the_where_clause() {
    let query = DeleteBuilder::new().where_clause("id = $1").as_string();
    let expected_query = "WHERE id = $1";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_where_should_accumulate_values_on_consecutive_calls() {
    let query = DeleteBuilder::new()
      .where_clause("id = $1")
      .where_clause("status = 'pending'")
      .as_string();
    let expected_query = "WHERE id = $1 AND status = 'pending'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_where_should_be_after_delete_from_clause() {
    let query = DeleteBuilder::new()
      .where_clause("name = $1")
      .delete_from("users")
      .as_string();
    let expected_query = "DELETE FROM users WHERE name = $1";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_where_clause_should_not_accumulate_arguments_with_the_same_content() {
    let query = DeleteBuilder::new()
      .where_clause("id = $1")
      .where_clause("id = $1")
      .as_string();
    let expected_query = "WHERE id = $1";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_where_should_trim_space_of_the_argument() {
    let query = DeleteBuilder::new().where_clause("  id = $1  ").as_string();
    let expected_query = "WHERE id = $1";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_where_clause() {
    let query = DeleteBuilder::new()
      .raw_before(DeleteClause::Where, "delete from users")
      .where_clause("login = $1")
      .as_string();
    let expected_query = "delete from users WHERE login = $1";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_where_clause() {
    let query = DeleteBuilder::new()
      .where_clause("created_at::date >= $1")
      .raw_after(DeleteClause::Where, "and created_at::date < $2")
      .as_string();
    let expected_query = "WHERE created_at::date >= $1 and created_at::date < $2";

    assert_eq!(query, expected_query);
  }
}
