use crate::SelectBuilder;

#[cfg(test)]
mod public_api {
  use super::*;
  use pretty_assertions::assert_eq;

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
  fn method_from_should_add_the_from_statement() {
    let query = SelectBuilder::new().from("users").as_string();
    let expected_query = "FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_from_should_accumulate_values_on_consecutive_calls() {
    let query = SelectBuilder::new().from("users").from("address").as_string();
    let expected_query = "FROM users,address";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_inner_join_should_add_the_inner_join_statement() {
    let query = SelectBuilder::new()
      .inner_join("address", "users.login = address.login")
      .as_string();
    let expected_query = "INNER JOIN address ON users.login = address.login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_inner_join_should_accumulate_values_on_consecutive_calls() {
    let query = SelectBuilder::new()
      .inner_join("address", "users.login = address.login")
      .inner_join("orders", "users.login = orders.login")
      .as_string();
    let expected_query = "\
      INNER JOIN address ON users.login = address.login \
      INNER JOIN orders ON users.login = orders.login\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_limit_should_add_the_limit_statement() {
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
  fn method_order_by_should_add_the_order_by_statement() {
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
    let expected_query = "ORDER BY login asc,created_at desc";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_print_should_print_in_one_line_the_current_state_of_builder() {
    let query = SelectBuilder::new().select("1 + 2").print().as_string();
    let expected_query = "SELECT 1 + 2";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_add_raw_sql() {
    let query = SelectBuilder::new().raw("select id from users").as_string();
    let expected_query = "select id from users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_accumulate_values_on_consecutive_calls() {
    let query = SelectBuilder::new().raw("select id").raw("from users").as_string();
    let expected_query = "select id from users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_select_should_add_the_select_statement() {
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
    let expected_query = "SELECT id, login,created_at";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_union_should_add_the_union_statement() {
    let select_users = SelectBuilder::new().select("login").from("users");
    let select_address = SelectBuilder::new().select("login").from("address");
    let query = select_users.union(select_address).as_string();
    let expected_query = "SELECT login FROM users UNION SELECT login FROM address";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_union_should_accumulate_values_on_consecutive_calls() {
    let select_users = SelectBuilder::new().select("login").from("users");
    let select_address = SelectBuilder::new().select("login").from("address");
    let select_orders = SelectBuilder::new().select("login").from("orders");
    let query = select_users.union(select_address).union(select_orders).as_string();
    let expected_query = "\
      SELECT login FROM users \
      UNION \
      SELECT login FROM address \
      UNION \
      SELECT login FROM orders\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_where_should_add_the_where_statement() {
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
  fn method_with_should_add_the_with_statement() {
    let select_users = SelectBuilder::new().select("login").from("users");
    let query = SelectBuilder::new().with("user_list", select_users).as_string();
    let expected_query = "WITH user_list AS ( SELECT login FROM users )";

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
      WITH user_list AS ( SELECT id, login FROM users ),user_ids AS ( SELECT id FROM user_list )\
    ";

    assert_eq!(query, expected_query);
  }
}

#[cfg(test)]
mod public_features {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn select_builder_should_be_displayable() {
    let select = SelectBuilder::new().select("id, login").from("users");

    println!("{}", select);

    let query = select.as_string();
    let expected_query = "SELECT id, login FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn select_builder_should_be_debuggable() {
    let select = SelectBuilder::new().select("*").from("orders").where_clause("id = $1");

    println!("{:?}", select);

    let expected_query = "SELECT * FROM orders WHERE id = $1";
    let query = select.as_string();

    assert_eq!(query, expected_query);
  }

  #[test]
  fn select_builder_should_be_cloneable() {
    let select_zipcode = SelectBuilder::new()
      .select("zipcode")
      .from("address")
      .where_clause("login = $1");
    let select_city = select_zipcode.clone().select("city");
    let query_zipcode = select_zipcode.as_string();
    let query_city = select_city.as_string();

    let expected_query_zipcode = "SELECT zipcode FROM address WHERE login = $1";
    let expected_query_city = "SELECT zipcode,city FROM address WHERE login = $1";

    assert_eq!(query_zipcode, expected_query_zipcode);
    assert_eq!(query_city, expected_query_city);
  }

  #[test]
  fn select_builder_should_be_able_to_dynamically_add_statements() {
    let mut select = SelectBuilder::new().select("zipcode").from("address");

    if true {
      select = select.where_clause("login = $1").limit("$2");
    }

    let query = select.as_string();
    let expected_query = "SELECT zipcode FROM address WHERE login = $1 LIMIT $2";

    assert_eq!(query, expected_query);
  }
}

#[cfg(test)]
mod concat_order {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn all_statements_in_order() {
    let select_users = SelectBuilder::new().select("*").from("users");
    let select_address = SelectBuilder::new().select("city").from("address");
    let query = SelectBuilder::new()
      .raw("/* all statements in order */")
      .with("user_list", select_users)
      .select("*")
      .from("user_list")
      .inner_join("orders", "users.login = orders.login")
      .where_clause("user.login = $1")
      .order_by("created_at desc")
      .limit("10")
      .union(select_address)
      .as_string();

    let expected_query = "\
      /* all statements in order */ \
      WITH user_list AS ( SELECT * FROM users ) \
      SELECT * \
      FROM user_list \
      INNER JOIN orders ON users.login = orders.login \
      WHERE user.login = $1 \
      ORDER BY created_at desc \
      LIMIT 10 \
      UNION \
      SELECT city FROM address\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn statement_raw_should_be_the_first() {
    let query = SelectBuilder::new().raw("select *").from("users").as_string();
    let expected_query = "select * FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn statement_with_should_be_after_raw() {
    let select_base = SelectBuilder::new()
      .raw("select 123 as id union")
      .with("user_list", SelectBuilder::new().select("*").from("users"))
      .select("id");
    let query = select_base.as_string();
    let expected_query = "\
      select 123 as id union \
      WITH user_list AS ( SELECT * FROM users ) \
      SELECT id\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn statement_select_should_be_after_with() {
    let select_users = SelectBuilder::new().select("*").from("users");
    let select_base = SelectBuilder::new().with("user_list", select_users).select("id");
    let query = select_base.as_string();
    let expected_query = "\
      WITH user_list AS ( SELECT * FROM users ) \
      SELECT id\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn statement_from_should_be_after_select() {
    let query = SelectBuilder::new().select("*").from("users").as_string();
    let expected_query = "SELECT * FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn statement_inner_join_should_be_after_from() {
    let query = SelectBuilder::new()
      .from("users")
      .inner_join("address", "users.login = address.login")
      .as_string();
    let expected_query = "FROM users INNER JOIN address ON users.login = address.login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn statement_where_should_be_after_any_of_the_joins() {
    let query = SelectBuilder::new()
      .inner_join("address", "users.login = address.login")
      .where_clause("user.login = $1")
      .as_string();
    let expected_query = "INNER JOIN address ON users.login = address.login WHERE user.login = $1";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn statement_order_by_should_be_after_where() {
    let query = SelectBuilder::new()
      .where_clause("user.login = $1")
      .order_by("created_at desc")
      .as_string();
    let expected_query = "WHERE user.login = $1 ORDER BY created_at desc";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn statement_limit_should_be_after_order_by() {
    let query = SelectBuilder::new().order_by("created_at desc").limit("42").as_string();
    let expected_query = "ORDER BY created_at desc LIMIT 42";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn statement_union_should_be_after_limit() {
    let select_address = SelectBuilder::new().select("login").from("address");
    let query = SelectBuilder::new().limit("10").union(select_address).as_string();
    let expected_query = "\
      LIMIT 10 \
      UNION \
      SELECT login FROM address\
    ";

    assert_eq!(query, expected_query);
  }
}
