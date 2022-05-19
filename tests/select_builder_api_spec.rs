use pretty_assertions::assert_eq;
use sql_query_builder::SelectBuilder;

#[test]
fn method_new_should_initialize_as_empty_string() {
  let query = SelectBuilder::new().as_string();
  let expected_query = "";

  assert_eq!(query, expected_query);
}

#[test]
fn method_as_string_should_convert_the_current_state_to_string() {
  let query = SelectBuilder::new().as_string();
  let expected_query = "";

  assert_eq!(query, expected_query);
}

#[test]
fn method_and_should_add_a_where_clause() {
  let query = SelectBuilder::new().and("login = 'foo'").as_string();
  let expected_query = "WHERE login = 'foo'";

  assert_eq!(query, expected_query);
}

#[test]
fn method_and_should_accumulate_values_on_consecutive_calls() {
  let query = SelectBuilder::new()
    .and("login = 'foo'")
    .and("active = true")
    .as_string();
  let expected_query = "WHERE login = 'foo' AND active = true";

  assert_eq!(query, expected_query);
}

#[test]
fn method_debug_should_print_at_console_in_a_human_readable_format() {
  let query = SelectBuilder::new().select("current_date").debug().as_string();
  let expected_query = "SELECT current_date";

  assert_eq!(query, expected_query);
}

#[test]
fn method_from_should_add_the_from_clause() {
  let query = SelectBuilder::new().from("users").as_string();
  let expected_query = "FROM users";

  assert_eq!(query, expected_query);
}

#[test]
fn method_from_should_accumulate_values_on_consecutive_calls() {
  let query = SelectBuilder::new().from("users").from("address").as_string();
  let expected_query = "FROM users, address";

  assert_eq!(query, expected_query);
}

#[test]
fn method_group_by_should_add_the_group_by_clause() {
  let query = SelectBuilder::new().group_by("id, login").as_string();
  let expected_query = "GROUP BY id, login";

  assert_eq!(query, expected_query);
}

#[test]
fn method_group_by_should_accumulate_values_on_consecutive_calls() {
  let query = SelectBuilder::new()
    .group_by("id, login")
    .group_by("created_at")
    .as_string();
  let expected_query = "GROUP BY id, login, created_at";

  assert_eq!(query, expected_query);
}

#[test]
fn method_having_should_add_the_having_clause() {
  let query = SelectBuilder::new().having("active = true").as_string();
  let expected_query = "HAVING active = true";

  assert_eq!(query, expected_query);
}

#[test]
fn method_having_should_accumulate_values_on_consecutive_calls() {
  let query = SelectBuilder::new()
    .having("active = true")
    .having("allow = true")
    .as_string();
  let expected_query = "HAVING active = true AND allow = true";

  assert_eq!(query, expected_query);
}

#[test]
fn method_limit_should_add_the_limit_clause() {
  let query = SelectBuilder::new().limit("3").as_string();
  let expected_query = "LIMIT 3";

  assert_eq!(query, expected_query);
}

#[test]
fn method_limit_should_override_the_current_value() {
  let query = SelectBuilder::new().limit("3").limit("4").as_string();
  let expected_query = "LIMIT 4";

  assert_eq!(query, expected_query);
}

#[test]
fn method_offset_should_add_the_offset_clause() {
  let query = SelectBuilder::new().offset("100").as_string();
  let expected_query = "OFFSET 100";

  assert_eq!(query, expected_query);
}

#[test]
fn method_offset_should_override_the_current_value() {
  let query = SelectBuilder::new().offset("100").offset("200").as_string();
  let expected_query = "OFFSET 200";

  assert_eq!(query, expected_query);
}

#[test]
fn method_order_by_should_add_the_order_by_clause() {
  let query = SelectBuilder::new().order_by("id asc").as_string();
  let expected_query = "ORDER BY id asc";

  assert_eq!(query, expected_query);
}

#[test]
fn method_order_by_should_accumulate_values_on_consecutive_calls() {
  let query = SelectBuilder::new()
    .order_by("login asc")
    .order_by("created_at desc")
    .as_string();
  let expected_query = "ORDER BY login asc, created_at desc";

  assert_eq!(query, expected_query);
}

#[test]
fn method_print_should_print_in_one_line_the_current_state_of_builder() {
  let query = SelectBuilder::new().select("1 + 2").print().as_string();
  let expected_query = "SELECT 1 + 2";

  assert_eq!(query, expected_query);
}

#[test]
fn method_select_should_add_the_select_clause() {
  let query = SelectBuilder::new().select("id, login").as_string();
  let expected_query = "SELECT id, login";

  assert_eq!(query, expected_query);
}

#[test]
fn method_select_should_accumulate_values_on_consecutive_calls() {
  let query = SelectBuilder::new()
    .select("id, login")
    .select("created_at")
    .as_string();
  let expected_query = "SELECT id, login, created_at";

  assert_eq!(query, expected_query);
}

#[test]
fn method_where_should_add_the_where_clause() {
  let query = SelectBuilder::new()
    .where_clause("created_at::date = current_date")
    .as_string();
  let expected_query = "WHERE created_at::date = current_date";

  assert_eq!(query, expected_query);
}

#[test]
fn method_where_should_accumulate_values_on_consecutive_calls() {
  let query = SelectBuilder::new()
    .where_clause("created_at::date > current_date - INTERVAL '2 days'")
    .where_clause("created_at::date <= current_date")
    .as_string();
  let expected_query = "\
      WHERE \
        created_at::date > current_date - INTERVAL '2 days' \
        AND created_at::date <= current_date\
    ";

  assert_eq!(query, expected_query);
}

#[test]
fn method_with_should_add_the_with_clause() {
  let select_users = SelectBuilder::new().select("login").from("users");
  let query = SelectBuilder::new().with("user_list", select_users).as_string();
  let expected_query = "WITH user_list AS (SELECT login FROM users)";

  assert_eq!(query, expected_query);
}

#[test]
fn method_with_should_accept_inline_argument() {
  let query = SelectBuilder::new()
    .with("user_list", SelectBuilder::new().select("login").from("users"))
    .as_string();
  let expected_query = "WITH user_list AS (SELECT login FROM users)";

  assert_eq!(query, expected_query);
}

#[test]
fn method_with_should_accumulate_values_on_consecutive_calls() {
  let select_users = SelectBuilder::new().select("id, login").from("users");
  let select_users_id = SelectBuilder::new().select("id").from("user_list");
  let query = SelectBuilder::new()
    .with("user_list", select_users)
    .with("user_ids", select_users_id)
    .as_string();
  let expected_query = "\
      WITH user_list AS (SELECT id, login FROM users), user_ids AS (SELECT id FROM user_list)\
    ";

  assert_eq!(query, expected_query);
}
