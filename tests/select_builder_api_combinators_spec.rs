use pretty_assertions::assert_eq;
use sql_query_builder::SelectBuilder;

// Except

#[test]
fn method_except_should_add_the_except_clause() {
  let select_users = SelectBuilder::new().select("login").from("users");
  let select_address = SelectBuilder::new().select("login").from("address");
  let query = select_users.except(select_address).as_string();
  let expected_query = "(SELECT login FROM users) EXCEPT (SELECT login FROM address)";

  assert_eq!(query, expected_query);
}

#[test]
fn method_except_should_accept_inline_argument() {
  let select_users = SelectBuilder::new().select("login").from("users");
  let query = select_users
    .except(SelectBuilder::new().select("login").from("address"))
    .as_string();
  let expected_query = "(SELECT login FROM users) EXCEPT (SELECT login FROM address)";

  assert_eq!(query, expected_query);
}

#[test]
fn method_except_should_accumulate_values_on_consecutive_calls() {
  let select_users = SelectBuilder::new().select("login").from("users");
  let select_address = SelectBuilder::new().select("login").from("address");
  let select_orders = SelectBuilder::new().select("login").from("orders");
  let query = select_users.except(select_address).except(select_orders).as_string();
  let expected_query = "\
    (SELECT login FROM users) \
    EXCEPT \
    (SELECT login FROM address) \
    EXCEPT \
    (SELECT login FROM orders)\
  ";

  assert_eq!(query, expected_query);
}

// Intersect

#[test]
fn method_intersect_should_add_the_intersect_clause() {
  let select_users = SelectBuilder::new().select("login").from("users");
  let select_address = SelectBuilder::new().select("login").from("address");
  let query = select_users.intersect(select_address).as_string();
  let expected_query = "(SELECT login FROM users) INTERSECT (SELECT login FROM address)";

  assert_eq!(query, expected_query);
}

#[test]
fn method_intersect_should_accept_inline_argument() {
  let select_users = SelectBuilder::new().select("login").from("users");
  let query = select_users
    .intersect(SelectBuilder::new().select("login").from("address"))
    .as_string();
  let expected_query = "(SELECT login FROM users) INTERSECT (SELECT login FROM address)";

  assert_eq!(query, expected_query);
}

#[test]
fn method_intersect_should_accumulate_values_on_consecutive_calls() {
  let select_users = SelectBuilder::new().select("login").from("users");
  let select_address = SelectBuilder::new().select("login").from("address");
  let select_orders = SelectBuilder::new().select("login").from("orders");
  let query = select_users
    .intersect(select_address)
    .intersect(select_orders)
    .as_string();
  let expected_query = "\
    (SELECT login FROM users) \
    INTERSECT \
    (SELECT login FROM address) \
    INTERSECT \
    (SELECT login FROM orders)\
  ";

  assert_eq!(query, expected_query);
}

// Union

#[test]
fn method_union_should_add_the_union_clause() {
  let select_users = SelectBuilder::new().select("login").from("users");
  let select_address = SelectBuilder::new().select("login").from("address");
  let query = select_users.union(select_address).as_string();
  let expected_query = "(SELECT login FROM users) UNION (SELECT login FROM address)";

  assert_eq!(query, expected_query);
}

#[test]
fn method_union_should_accept_inline_argument() {
  let select_users = SelectBuilder::new().select("login").from("users");
  let query = select_users
    .union(SelectBuilder::new().select("login").from("address"))
    .as_string();
  let expected_query = "(SELECT login FROM users) UNION (SELECT login FROM address)";

  assert_eq!(query, expected_query);
}

#[test]
fn method_union_should_accumulate_values_on_consecutive_calls() {
  let select_users = SelectBuilder::new().select("login").from("users");
  let select_address = SelectBuilder::new().select("login").from("address");
  let select_orders = SelectBuilder::new().select("login").from("orders");
  let query = select_users.union(select_address).union(select_orders).as_string();
  let expected_query = "\
    (SELECT login FROM users) \
    UNION \
    (SELECT login FROM address) \
    UNION \
    (SELECT login FROM orders)\
  ";

  assert_eq!(query, expected_query);
}
