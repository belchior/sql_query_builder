#[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
mod select_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_intersect_should_add_the_intersect_clause() {
    let select_users = sql::Select::new().select("login").from("users");
    let select_addresses = sql::Select::new().select("login").from("addresses");
    let query = select_users.intersect(select_addresses).as_string();
    let expected_query = "(SELECT login FROM users) INTERSECT (SELECT login FROM addresses)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_intersect_should_accept_inline_argument() {
    let select_users = sql::Select::new().select("login").from("users");
    let query = select_users
      .intersect(sql::Select::new().select("login").from("addresses"))
      .as_string();
    let expected_query = "(SELECT login FROM users) INTERSECT (SELECT login FROM addresses)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_intersect_should_accumulate_values_on_consecutive_calls() {
    let select_users = sql::Select::new().select("login").from("users");
    let select_addresses = sql::Select::new().select("login").from("addresses");
    let select_orders = sql::Select::new().select("login").from("orders");
    let query = select_users
      .intersect(select_addresses)
      .intersect(select_orders)
      .as_string();
    let expected_query = "\
      (SELECT login FROM users) \
      INTERSECT \
      (SELECT login FROM addresses) \
      INTERSECT \
      (SELECT login FROM orders)\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_intersect_should_be_after_offset_clause() {
    let select_addresses = sql::Select::new().select("login").from("addresses");
    let query = sql::Select::new().offset("10").intersect(select_addresses).as_string();
    let expected_query = "\
      (OFFSET 10) \
      INTERSECT \
      (SELECT login FROM addresses)\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_intersect_clause() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::Except, "select name from orders")
      .intersect(sql::Select::new().select("name"))
      .as_string();
    let expected_query = "(select name from orders) INTERSECT (SELECT name)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_intersect_clause() {
    let query = sql::Select::new()
      .select("name")
      .intersect(sql::Select::new().select("name"))
      .raw_after(sql::SelectClause::Intersect, "/* the name */")
      .as_string();
    let expected_query = "(SELECT name) INTERSECT (SELECT name) /* the name */";

    assert_eq!(query, expected_query);
  }
}
