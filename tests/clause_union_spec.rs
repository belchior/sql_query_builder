#[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
mod select_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_union_should_add_the_union_clause() {
    let select_users = sql::Select::new().select("login").from("users");
    let select_addresses = sql::Select::new().select("login").from("addresses");
    let query = select_users.union(select_addresses).as_string();
    let expected_query = "(SELECT login FROM users) UNION (SELECT login FROM addresses)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_union_should_accept_inline_argument() {
    let select_users = sql::Select::new().select("login").from("users");
    let query = select_users
      .union(sql::Select::new().select("login").from("addresses"))
      .as_string();
    let expected_query = "(SELECT login FROM users) UNION (SELECT login FROM addresses)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_union_should_accumulate_values_on_consecutive_calls() {
    let select_users = sql::Select::new().select("login").from("users");
    let select_addresses = sql::Select::new().select("login").from("addresses");
    let select_orders = sql::Select::new().select("login").from("orders");
    let query = select_users.union(select_addresses).union(select_orders).as_string();
    let expected_query = "\
      (SELECT login FROM users) \
      UNION \
      (SELECT login FROM addresses) \
      UNION \
      (SELECT login FROM orders)\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_union_should_be_after_offset_clause() {
    let select_addresses = sql::Select::new().select("login").from("addresses");
    let query = sql::Select::new().offset("10").union(select_addresses).as_string();
    let expected_query = "\
      (OFFSET 10) \
      UNION \
      (SELECT login FROM addresses)\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_union_clause() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::Union, "select name from orders")
      .union(sql::Select::new().select("name"))
      .as_string();
    let expected_query = "(select name from orders) UNION (SELECT name)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_union_clause() {
    let query = sql::Select::new()
      .select("name")
      .union(sql::Select::new().select("name"))
      .raw_after(sql::SelectClause::Union, "/* the name */")
      .as_string();
    let expected_query = "(SELECT name) UNION (SELECT name) /* the name */";

    assert_eq!(query, expected_query);
  }
}
