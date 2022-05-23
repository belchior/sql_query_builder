use pretty_assertions::assert_eq;
use sql_query_builder::{SelectBuilder, SelectClause};

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
    .select("name")
    .except(SelectBuilder::new().select("name"))
    .raw_after(SelectClause::Except, "/* the name */")
    .as_string();
  let expected_query = "(SELECT name) EXCEPT (SELECT name) /* the name */";

  assert_eq!(query, expected_query);
}

#[test]
fn method_raw_after_should_add_raw_sql_after_from_clause() {
  let query = SelectBuilder::new()
    .from("users")
    .raw_after(SelectClause::From, "inner join address on users.login = address.login")
    .as_string();
  let expected_query = "FROM users inner join address on users.login = address.login";

  assert_eq!(query, expected_query);
}

#[test]
fn method_raw_after_should_add_raw_sql_after_group_by_clause() {
  let query = SelectBuilder::new()
    .group_by("login")
    .raw_after(SelectClause::GroupBy, "LIMIT 10")
    .as_string();
  let expected_query = "GROUP BY login LIMIT 10";

  assert_eq!(query, expected_query);
}

#[test]
fn method_raw_after_should_add_raw_sql_after_having_clause() {
  let query = SelectBuilder::new()
    .having("active = true")
    .raw_after(SelectClause::Having, "LIMIT 10")
    .as_string();
  let expected_query = "HAVING active = true LIMIT 10";

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

#[test]
fn method_raw_after_should_add_raw_sql_after_join_clause() {
  let query = SelectBuilder::new()
    .inner_join("address ON users.login = address.login")
    .raw_after(SelectClause::Join, "where id = $1")
    .as_string();
  let expected_query = "INNER JOIN address ON users.login = address.login where id = $1";

  assert_eq!(query, expected_query);
}

#[test]
fn method_raw_after_should_add_raw_sql_after_limit_clause() {
  let query = SelectBuilder::new()
    .limit("10")
    .raw_after(SelectClause::Limit, "except select id, login")
    .as_string();
  let expected_query = "LIMIT 10 except select id, login";

  assert_eq!(query, expected_query);
}

#[test]
fn method_raw_after_should_add_raw_sql_after_offset_clause() {
  let query = SelectBuilder::new()
    .offset("10")
    .raw_after(SelectClause::Offset, "/* the end */")
    .as_string();
  let expected_query = "OFFSET 10 /* the end */";

  assert_eq!(query, expected_query);
}

#[test]
fn method_raw_after_should_add_raw_sql_after_order_by_clause() {
  let query = SelectBuilder::new()
    .order_by("id desc")
    .raw_after(SelectClause::OrderBy, "limit 20")
    .as_string();
  let expected_query = "ORDER BY id desc limit 20";

  assert_eq!(query, expected_query);
}

#[test]
fn method_raw_after_should_add_raw_sql_after_select_clause() {
  let query = SelectBuilder::new()
    .select("id, name")
    .raw_after(SelectClause::Select, "from address")
    .as_string();
  let expected_query = "SELECT id, name from address";

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

#[test]
fn method_raw_after_should_add_raw_sql_after_where_clause() {
  let query = SelectBuilder::new()
    .where_clause("created_at::date = current_date")
    .raw_after(SelectClause::Where, "limit 10")
    .as_string();
  let expected_query = "WHERE created_at::date = current_date limit 10";

  assert_eq!(query, expected_query);
}

#[test]
fn method_raw_after_should_add_raw_sql_with_clause() {
  let query = SelectBuilder::new()
    .with("address_list", SelectBuilder::new().select("*").from("address"))
    .raw_after(SelectClause::With, "select name, login")
    .as_string();
  let expected_query = "WITH address_list AS (SELECT * FROM address) select name, login";

  assert_eq!(query, expected_query);
}

// Raw before method

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
fn method_raw_before_should_add_raw_sql_before_from_clause() {
  let query = SelectBuilder::new()
    .raw_before(SelectClause::From, "select amount")
    .from("orders")
    .as_string();
  let expected_query = "select amount FROM orders";

  assert_eq!(query, expected_query);
}

#[test]
fn method_raw_before_should_add_raw_sql_before_group_by_clause() {
  let query = SelectBuilder::new()
    .raw_before(SelectClause::GroupBy, "where id = $1")
    .group_by("login")
    .as_string();
  let expected_query = "where id = $1 GROUP BY login";

  assert_eq!(query, expected_query);
}

#[test]
fn method_raw_before_should_add_raw_sql_before_having_clause() {
  let query = SelectBuilder::new()
    .raw_before(SelectClause::Having, "group by id")
    .having("active = true")
    .as_string();
  let expected_query = "group by id HAVING active = true";

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
fn method_raw_before_should_add_raw_sql_before_join_clause() {
  let query = SelectBuilder::new()
    .raw_before(SelectClause::Join, "from orders")
    .inner_join("address ON address.user_login = orders.user_login")
    .as_string();
  let expected_query = "from orders INNER JOIN address ON address.user_login = orders.user_login";

  assert_eq!(query, expected_query);
}

#[test]
fn method_raw_before_should_add_raw_sql_before_limit_clause() {
  let query = SelectBuilder::new()
    .raw_before(SelectClause::Limit, "group by id")
    .limit("10")
    .as_string();
  let expected_query = "group by id LIMIT 10";

  assert_eq!(query, expected_query);
}

#[test]
fn method_raw_before_should_add_raw_sql_before_offset_clause() {
  let query = SelectBuilder::new()
    .raw_before(SelectClause::Limit, "limit 1000")
    .offset("50")
    .as_string();
  let expected_query = "limit 1000 OFFSET 50";

  assert_eq!(query, expected_query);
}

#[test]
fn method_raw_before_should_add_raw_sql_before_order_by_clause() {
  let query = SelectBuilder::new()
    .raw_before(SelectClause::OrderBy, "where orders.user_login = $1")
    .order_by("id desc")
    .as_string();
  let expected_query = "where orders.user_login = $1 ORDER BY id desc";

  assert_eq!(query, expected_query);
}

#[test]
fn method_raw_before_should_add_raw_sql_before_select_clause() {
  let query = SelectBuilder::new()
    .raw_before(SelectClause::Select, "/* list orders */")
    .select("id, name")
    .as_string();
  let expected_query = "/* list orders */ SELECT id, name";

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
fn method_raw_before_should_add_raw_sql_before_where_clause() {
  let query = SelectBuilder::new()
    .raw_before(SelectClause::Where, "from orders")
    .where_clause("created_at::date = current_date")
    .as_string();
  let expected_query = "from orders WHERE created_at::date = current_date";

  assert_eq!(query, expected_query);
}

#[test]
fn method_raw_before_should_add_raw_sql_before_with_clause() {
  let query = SelectBuilder::new()
    .raw_before(SelectClause::With, "/* the users orders */")
    .with("orders_list", SelectBuilder::new().select("*").from("orders"))
    .as_string();
  let expected_query = "/* the users orders */ WITH orders_list AS (SELECT * FROM orders)";

  assert_eq!(query, expected_query);
}
