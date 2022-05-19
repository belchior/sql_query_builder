use pretty_assertions::assert_eq;
use sql_query_builder::SelectBuilder;

#[test]
fn method_cross_join_should_add_the_cross_join_clause() {
  let query = SelectBuilder::new().cross_join("address").as_string();
  let expected_query = "CROSS JOIN address";

  assert_eq!(query, expected_query);
}

#[test]
fn method_cross_join_should_accumulate_values_on_consecutive_calls() {
  let query = SelectBuilder::new()
    .cross_join("address")
    .cross_join("orders")
    .as_string();
  let expected_query = "\
      CROSS JOIN address \
      CROSS JOIN orders\
    ";

  assert_eq!(query, expected_query);
}

#[test]
fn method_inner_join_should_add_the_inner_join_clause() {
  let query = SelectBuilder::new()
    .inner_join("address ON users.login = address.login")
    .as_string();
  let expected_query = "INNER JOIN address ON users.login = address.login";

  assert_eq!(query, expected_query);
}

#[test]
fn method_inner_join_should_accumulate_values_on_consecutive_calls() {
  let query = SelectBuilder::new()
    .inner_join("address ON users.login = address.login")
    .inner_join("orders ON users.login = orders.login")
    .as_string();
  let expected_query = "\
      INNER JOIN address ON users.login = address.login \
      INNER JOIN orders ON users.login = orders.login\
    ";

  assert_eq!(query, expected_query);
}

#[test]
fn method_left_join_should_add_the_left_join_clause() {
  let query = SelectBuilder::new()
    .left_join("address ON users.login = address.login")
    .as_string();
  let expected_query = "LEFT JOIN address ON users.login = address.login";

  assert_eq!(query, expected_query);
}

#[test]
fn method_left_join_should_accumulate_values_on_consecutive_calls() {
  let query = SelectBuilder::new()
    .left_join("address ON users.login = address.login")
    .left_join("orders ON users.login = orders.login")
    .as_string();
  let expected_query = "\
      LEFT JOIN address ON users.login = address.login \
      LEFT JOIN orders ON users.login = orders.login\
    ";

  assert_eq!(query, expected_query);
}

#[test]
fn method_right_join_should_add_the_right_join_clause() {
  let query = SelectBuilder::new()
    .right_join("address ON users.login = address.login")
    .as_string();
  let expected_query = "RIGHT JOIN address ON users.login = address.login";

  assert_eq!(query, expected_query);
}

#[test]
fn method_right_join_should_accumulate_values_on_consecutive_calls() {
  let query = SelectBuilder::new()
    .right_join("address ON users.login = address.login")
    .right_join("orders ON users.login = orders.login")
    .as_string();
  let expected_query = "\
      RIGHT JOIN address ON users.login = address.login \
      RIGHT JOIN orders ON users.login = orders.login\
    ";

  assert_eq!(query, expected_query);
}
