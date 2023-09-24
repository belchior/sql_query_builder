#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod select_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_except_should_add_the_except_clause() {
    let select_users = sql::Select::new().select("login").from("users");
    let select_addresses = sql::Select::new().select("login").from("addresses");
    let query = select_users.except(select_addresses).as_string();
    let expected_query = "(SELECT login FROM users) EXCEPT (SELECT login FROM addresses)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_except_should_accept_inline_argument() {
    let select_users = sql::Select::new().select("login").from("users");
    let query = select_users
      .except(sql::Select::new().select("login").from("addresses"))
      .as_string();
    let expected_query = "(SELECT login FROM users) EXCEPT (SELECT login FROM addresses)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_except_should_accumulate_values_on_consecutive_calls() {
    let select_users = sql::Select::new().select("login").from("users");
    let select_addresses = sql::Select::new().select("login").from("addresses");
    let select_orders = sql::Select::new().select("login").from("orders");
    let query = select_users.except(select_addresses).except(select_orders).as_string();
    let expected_query = "\
      (SELECT login FROM users) \
      EXCEPT \
      (SELECT login FROM addresses) \
      EXCEPT \
      (SELECT login FROM orders)\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_except_should_be_after_offset_clause() {
    let select_addresses = sql::Select::new().select("login").from("addresses");
    let query = sql::Select::new().offset("10").except(select_addresses).as_string();
    let expected_query = "\
      (OFFSET 10) \
      EXCEPT \
      (SELECT login FROM addresses)\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_except_clause() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::Except, "select name from orders")
      .except(sql::Select::new().select("name"))
      .as_string();
    let expected_query = "(select name from orders) EXCEPT (SELECT name)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_except_clause() {
    let query = sql::Select::new()
      .select("name")
      .except(sql::Select::new().select("name"))
      .raw_after(sql::SelectClause::Except, "/* the name */")
      .as_string();
    let expected_query = "(SELECT name) EXCEPT (SELECT name) /* the name */";

    assert_eq!(query, expected_query);
  }
}
