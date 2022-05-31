use sql_query_builder::{SelectBuilder, SelectClause};

mod except_clause {
  use super::*;
  use pretty_assertions::assert_eq;

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

  #[test]
  fn clause_except_should_be_after_offset_clause() {
    let select_address = SelectBuilder::new().select("login").from("address");
    let query = SelectBuilder::new().offset("10").except(select_address).as_string();
    let expected_query = "\
    (OFFSET 10) \
    EXCEPT \
    (SELECT login FROM address)\
  ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_except_clause() {
    let query = SelectBuilder::new()
      .raw_before(SelectClause::Except, "select name from orders")
      .except(SelectBuilder::new().select("name"))
      .as_string();
    let expected_query = "(select name from orders) EXCEPT (SELECT name)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_except_clause() {
    let query = SelectBuilder::new()
      .select("name")
      .except(SelectBuilder::new().select("name"))
      .raw_after(SelectClause::Except, "/* the name */")
      .as_string();
    let expected_query = "(SELECT name) EXCEPT (SELECT name) /* the name */";

    assert_eq!(query, expected_query);
  }
}

mod intersect_clause {
  use super::*;
  use pretty_assertions::assert_eq;

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

  #[test]
  fn clause_intersect_should_be_after_offset_clause() {
    let select_address = SelectBuilder::new().select("login").from("address");
    let query = SelectBuilder::new().offset("10").intersect(select_address).as_string();
    let expected_query = "\
    (OFFSET 10) \
    INTERSECT \
    (SELECT login FROM address)\
  ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_intersect_clause() {
    let query = SelectBuilder::new()
      .raw_before(SelectClause::Except, "select name from orders")
      .intersect(SelectBuilder::new().select("name"))
      .as_string();
    let expected_query = "(select name from orders) INTERSECT (SELECT name)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_intersect_clause() {
    let query = SelectBuilder::new()
      .select("name")
      .intersect(SelectBuilder::new().select("name"))
      .raw_after(SelectClause::Intersect, "/* the name */")
      .as_string();
    let expected_query = "(SELECT name) INTERSECT (SELECT name) /* the name */";

    assert_eq!(query, expected_query);
  }
}

mod union_clause {
  use super::*;
  use pretty_assertions::assert_eq;

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

  #[test]
  fn clause_union_should_be_after_offset_clause() {
    let select_address = SelectBuilder::new().select("login").from("address");
    let query = SelectBuilder::new().offset("10").union(select_address).as_string();
    let expected_query = "\
    (OFFSET 10) \
    UNION \
    (SELECT login FROM address)\
  ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_union_clause() {
    let query = SelectBuilder::new()
      .raw_before(SelectClause::Union, "select name from orders")
      .union(SelectBuilder::new().select("name"))
      .as_string();
    let expected_query = "(select name from orders) UNION (SELECT name)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_union_clause() {
    let query = SelectBuilder::new()
      .select("name")
      .union(SelectBuilder::new().select("name"))
      .raw_after(SelectClause::Union, "/* the name */")
      .as_string();
    let expected_query = "(SELECT name) UNION (SELECT name) /* the name */";

    assert_eq!(query, expected_query);
  }
}
