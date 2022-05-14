use sql_query_builder::SelectBuilder;

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
  fn method_except_should_add_the_except_clause() {
    let select_users = SelectBuilder::new().select("login").from("users");
    let select_address = SelectBuilder::new().select("login").from("address");
    let query = select_users.except(select_address).as_string();
    let expected_query = "SELECT login FROM users EXCEPT SELECT login FROM address";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_except_should_accept_inline_argument() {
    let select_users = SelectBuilder::new().select("login").from("users");
    let query = select_users
      .except(SelectBuilder::new().select("login").from("address"))
      .as_string();
    let expected_query = "SELECT login FROM users EXCEPT SELECT login FROM address";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_except_should_accumulate_values_on_consecutive_calls() {
    let select_users = SelectBuilder::new().select("login").from("users");
    let select_address = SelectBuilder::new().select("login").from("address");
    let select_orders = SelectBuilder::new().select("login").from("orders");
    let query = select_users.except(select_address).except(select_orders).as_string();
    let expected_query = "\
      SELECT login FROM users \
      EXCEPT \
      SELECT login FROM address \
      EXCEPT \
      SELECT login FROM orders\
    ";

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
  fn method_intersect_should_add_the_intersect_clause() {
    let select_users = SelectBuilder::new().select("login").from("users");
    let select_address = SelectBuilder::new().select("login").from("address");
    let query = select_users.intersect(select_address).as_string();
    let expected_query = "SELECT login FROM users INTERSECT SELECT login FROM address";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_intersect_should_accept_inline_argument() {
    let select_users = SelectBuilder::new().select("login").from("users");
    let query = select_users
      .intersect(SelectBuilder::new().select("login").from("address"))
      .as_string();
    let expected_query = "SELECT login FROM users INTERSECT SELECT login FROM address";

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
      SELECT login FROM users \
      INTERSECT \
      SELECT login FROM address \
      INTERSECT \
      SELECT login FROM orders\
    ";

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
  fn method_union_should_add_the_union_clause() {
    let select_users = SelectBuilder::new().select("login").from("users");
    let select_address = SelectBuilder::new().select("login").from("address");
    let query = select_users.union(select_address).as_string();
    let expected_query = "SELECT login FROM users UNION SELECT login FROM address";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_union_should_accept_inline_argument() {
    let select_users = SelectBuilder::new().select("login").from("users");
    let query = select_users
      .union(SelectBuilder::new().select("login").from("address"))
      .as_string();
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
}

#[cfg(test)]
mod public_api_raw {
  use super::*;
  use pretty_assertions::assert_eq;
  use sql_query_builder::Clause;

  // Raw method

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

  // Raw after method

  #[test]
  fn method_raw_after_should_add_raw_sql_after_except_clause() {
    let query = SelectBuilder::new()
      .except(SelectBuilder::new().select("name"))
      .raw_after(Clause::Except, "/* the name */")
      .as_string();
    let expected_query = "EXCEPT SELECT name /* the name */";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_from_clause() {
    let query = SelectBuilder::new()
      .from("users")
      .raw_after(Clause::From, "inner join address on users.login = address.login")
      .as_string();
    let expected_query = "FROM users inner join address on users.login = address.login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_group_by_clause() {
    let query = SelectBuilder::new()
      .group_by("login")
      .raw_after(Clause::GroupBy, "LIMIT 10")
      .as_string();
    let expected_query = "GROUP BY login LIMIT 10";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_having_clause() {
    let query = SelectBuilder::new()
      .having("active = true")
      .raw_after(Clause::Having, "LIMIT 10")
      .as_string();
    let expected_query = "HAVING active = true LIMIT 10";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_intersect_clause() {
    let query = SelectBuilder::new()
      .intersect(SelectBuilder::new().select("name"))
      .raw_after(Clause::Intersect, "/* the name */")
      .as_string();
    let expected_query = "INTERSECT SELECT name /* the name */";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_join_clause() {
    let query = SelectBuilder::new()
      .inner_join("address ON users.login = address.login")
      .raw_after(Clause::Join, "where id = $1")
      .as_string();
    let expected_query = "INNER JOIN address ON users.login = address.login where id = $1";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_limit_clause() {
    let query = SelectBuilder::new()
      .limit("10")
      .raw_after(Clause::Limit, "except select id, login")
      .as_string();
    let expected_query = "LIMIT 10 except select id, login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_offset_clause() {
    let query = SelectBuilder::new()
      .offset("10")
      .raw_after(Clause::Offset, "/* the end */")
      .as_string();
    let expected_query = "OFFSET 10 /* the end */";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_order_by_clause() {
    let query = SelectBuilder::new()
      .order_by("id desc")
      .raw_after(Clause::OrderBy, "limit 20")
      .as_string();
    let expected_query = "ORDER BY id desc limit 20";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_select_clause() {
    let query = SelectBuilder::new()
      .select("id, name")
      .raw_after(Clause::Select, "from address")
      .as_string();
    let expected_query = "SELECT id, name from address";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_union_clause() {
    let query = SelectBuilder::new()
      .union(SelectBuilder::new().select("name"))
      .raw_after(Clause::Union, "/* the name */")
      .as_string();
    let expected_query = "UNION SELECT name /* the name */";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_where_clause() {
    let query = SelectBuilder::new()
      .where_clause("created_at::date = current_date")
      .raw_after(Clause::Where, "limit 10")
      .as_string();
    let expected_query = "WHERE created_at::date = current_date limit 10";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_with_clause() {
    let query = SelectBuilder::new()
      .with("address_list", SelectBuilder::new().select("*").from("address"))
      .raw_after(Clause::With, "select name, login")
      .as_string();
    let expected_query = "WITH address_list AS (SELECT * FROM address) select name, login";

    assert_eq!(query, expected_query);
  }

  // Raw before method

  #[test]
  fn method_raw_before_should_add_raw_sql_before_except_clause() {
    let query = SelectBuilder::new()
      .raw_before(Clause::Except, "select name from orders")
      .except(SelectBuilder::new().select("name"))
      .as_string();
    let expected_query = "select name from orders EXCEPT SELECT name";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_from_clause() {
    let query = SelectBuilder::new()
      .raw_before(Clause::From, "select amount")
      .from("orders")
      .as_string();
    let expected_query = "select amount FROM orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_group_by_clause() {
    let query = SelectBuilder::new()
      .raw_before(Clause::GroupBy, "where id = $1")
      .group_by("login")
      .as_string();
    let expected_query = "where id = $1 GROUP BY login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_having_clause() {
    let query = SelectBuilder::new()
      .raw_before(Clause::Having, "group by id")
      .having("active = true")
      .as_string();
    let expected_query = "group by id HAVING active = true";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_intersect_clause() {
    let query = SelectBuilder::new()
      .raw_before(Clause::Except, "select name from orders")
      .intersect(SelectBuilder::new().select("name"))
      .as_string();
    let expected_query = "select name from orders INTERSECT SELECT name";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_join_clause() {
    let query = SelectBuilder::new()
      .raw_before(Clause::Join, "from orders")
      .inner_join("address ON address.user_login = orders.user_login")
      .as_string();
    let expected_query = "from orders INNER JOIN address ON address.user_login = orders.user_login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_limit_clause() {
    let query = SelectBuilder::new()
      .raw_before(Clause::Limit, "group by id")
      .limit("10")
      .as_string();
    let expected_query = "group by id LIMIT 10";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_offset_clause() {
    let query = SelectBuilder::new()
      .raw_before(Clause::Limit, "limit 1000")
      .offset("50")
      .as_string();
    let expected_query = "limit 1000 OFFSET 50";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_order_by_clause() {
    let query = SelectBuilder::new()
      .raw_before(Clause::OrderBy, "where orders.user_login = $1")
      .order_by("id desc")
      .as_string();
    let expected_query = "where orders.user_login = $1 ORDER BY id desc";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_select_clause() {
    let query = SelectBuilder::new()
      .raw_before(Clause::Select, "/* list orders */")
      .select("id, name")
      .as_string();
    let expected_query = "/* list orders */ SELECT id, name";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_union_clause() {
    let query = SelectBuilder::new()
      .raw_before(Clause::Union, "select name from orders")
      .union(SelectBuilder::new().select("name"))
      .as_string();
    let expected_query = "select name from orders UNION SELECT name";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_where_clause() {
    let query = SelectBuilder::new()
      .raw_before(Clause::Where, "from orders")
      .where_clause("created_at::date = current_date")
      .as_string();
    let expected_query = "from orders WHERE created_at::date = current_date";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_with_clause() {
    let query = SelectBuilder::new()
      .raw_before(Clause::With, "/* the users orders */")
      .with("orders_list", SelectBuilder::new().select("*").from("orders"))
      .as_string();
    let expected_query = "/* the users orders */ WITH orders_list AS (SELECT * FROM orders)";

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
    let expected_query_city = "SELECT zipcode, city FROM address WHERE login = $1";

    assert_eq!(query_zipcode, expected_query_zipcode);
    assert_eq!(query_city, expected_query_city);
  }

  #[test]
  fn select_builder_should_be_able_to_dynamically_add_clauses() {
    let mut select = SelectBuilder::new().select("zipcode").from("address");

    if true {
      select = select.where_clause("login = $1").limit("$2");
    }

    let query = select.as_string();
    let expected_query = "SELECT zipcode FROM address WHERE login = $1 LIMIT $2";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn select_builder_should_be_composable() {
    fn project(select: SelectBuilder) -> SelectBuilder {
      select
        .select("u.id, u.name as user_name, u.login")
        .select("a.name as address_name")
        .select("o.name as product_name")
    }

    fn joins(select: SelectBuilder) -> SelectBuilder {
      select
        .inner_join("address a ON a.user_login = u.login")
        .inner_join("orders o ON o.user_login = u.login")
    }

    fn conditions(select: SelectBuilder) -> SelectBuilder {
      select.where_clause("u.login = $1").and("o.id = $2")
    }

    fn as_string(select: SelectBuilder) -> String {
      select.as_string()
    }

    let query = Some(SelectBuilder::new())
      .map(project)
      .map(joins)
      .map(conditions)
      .map(as_string)
      .unwrap();

    let expected_query = "\
      SELECT u.id, u.name as user_name, u.login, a.name as address_name, o.name as product_name \
      INNER JOIN address a ON a.user_login = u.login \
      INNER JOIN orders o ON o.user_login = u.login \
      WHERE u.login = $1 AND o.id = $2\
    ";

    assert_eq!(query, expected_query);
  }
}

#[cfg(test)]
mod concat_order {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn all_clauses_in_order() {
    let select_users = SelectBuilder::new().select("*").from("users");
    let select_address = SelectBuilder::new().select("city").from("address");
    let query = SelectBuilder::new()
      .raw("/* all clauses in order */")
      .with("user_list", select_users)
      .select("*")
      .from("user_list")
      .inner_join("orders ON users.login = orders.login")
      .where_clause("user.login = $1")
      .group_by("login")
      .having("active = true")
      .order_by("created_at desc")
      .limit("1000")
      .offset("50")
      .union(select_address)
      .as_string();

    let expected_query = "\
      /* all clauses in order */ \
      WITH user_list AS (SELECT * FROM users) \
      SELECT * \
      FROM user_list \
      INNER JOIN orders ON users.login = orders.login \
      WHERE user.login = $1 \
      GROUP BY login \
      HAVING active = true \
      ORDER BY created_at desc \
      LIMIT 1000 \
      OFFSET 50 \
      UNION \
      SELECT city FROM address\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_raw_should_be_the_first() {
    let query = SelectBuilder::new().raw("select *").from("users").as_string();
    let expected_query = "select * FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_with_should_be_after_raw() {
    let select_base = SelectBuilder::new()
      .raw("select 123 as id union")
      .with("user_list", SelectBuilder::new().select("*").from("users"))
      .select("id");
    let query = select_base.as_string();
    let expected_query = "\
      select 123 as id union \
      WITH user_list AS (SELECT * FROM users) \
      SELECT id\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_select_should_be_after_with() {
    let select_users = SelectBuilder::new().select("*").from("users");
    let select_base = SelectBuilder::new().with("user_list", select_users).select("id");
    let query = select_base.as_string();
    let expected_query = "\
      WITH user_list AS (SELECT * FROM users) \
      SELECT id\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_from_should_be_after_select() {
    let query = SelectBuilder::new().select("*").from("users").as_string();
    let expected_query = "SELECT * FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_inner_join_should_be_after_from() {
    let query = SelectBuilder::new()
      .from("users")
      .inner_join("address ON users.login = address.login")
      .as_string();
    let expected_query = "FROM users INNER JOIN address ON users.login = address.login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_where_should_be_after_any_of_the_joins() {
    let query = SelectBuilder::new()
      .inner_join("address ON users.login = address.login")
      .where_clause("user.login = $1")
      .as_string();
    let expected_query = "INNER JOIN address ON users.login = address.login WHERE user.login = $1";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_group_by_should_be_after_where() {
    let query = SelectBuilder::new()
      .group_by("login")
      .where_clause("login = $1")
      .as_string();
    let expected_query = "\
      WHERE login = $1 \
      GROUP BY login\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_having_should_be_after_group_by() {
    let query = SelectBuilder::new()
      .having("active = true")
      .group_by("login")
      .as_string();
    let expected_query = "\
      GROUP BY login \
      HAVING active = true\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_order_by_should_be_after_having() {
    let query = SelectBuilder::new()
      .having("active = true")
      .order_by("created_at desc")
      .as_string();
    let expected_query = "HAVING active = true ORDER BY created_at desc";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_limit_should_be_after_order_by() {
    let query = SelectBuilder::new().order_by("created_at desc").limit("42").as_string();
    let expected_query = "ORDER BY created_at desc LIMIT 42";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_offset_should_be_after_limit() {
    let query = SelectBuilder::new().limit("500").offset("100").as_string();
    let expected_query = "LIMIT 500 OFFSET 100";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_except_should_be_after_offset() {
    let select_address = SelectBuilder::new().select("login").from("address");
    let query = SelectBuilder::new().offset("10").except(select_address).as_string();
    let expected_query = "\
      OFFSET 10 \
      EXCEPT \
      SELECT login FROM address\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_intersect_should_be_after_offset() {
    let select_address = SelectBuilder::new().select("login").from("address");
    let query = SelectBuilder::new().offset("10").intersect(select_address).as_string();
    let expected_query = "\
      OFFSET 10 \
      INTERSECT \
      SELECT login FROM address\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_union_should_be_after_offset() {
    let select_address = SelectBuilder::new().select("login").from("address");
    let query = SelectBuilder::new().offset("10").union(select_address).as_string();
    let expected_query = "\
      OFFSET 10 \
      UNION \
      SELECT login FROM address\
    ";

    assert_eq!(query, expected_query);
  }
}
