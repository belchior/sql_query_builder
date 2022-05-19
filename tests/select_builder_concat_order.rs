use pretty_assertions::assert_eq;
use sql_query_builder::SelectBuilder;

#[test]
fn all_clauses_in_order() {
  let select_users = SelectBuilder::new().select("*").from("users");
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
      OFFSET 50\
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
fn clause_select_should_be_after_with_clause() {
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
fn clause_from_should_be_after_select_clause() {
  let query = SelectBuilder::new().select("*").from("users").as_string();
  let expected_query = "SELECT * FROM users";

  assert_eq!(query, expected_query);
}

#[test]
fn clause_cross_join_should_be_after_from_clause() {
  let query = SelectBuilder::new().from("users").cross_join("address").as_string();
  let expected_query = "FROM users CROSS JOIN address";

  assert_eq!(query, expected_query);
}

#[test]
fn clause_inner_join_should_be_after_from_clause() {
  let query = SelectBuilder::new()
    .from("users")
    .inner_join("address ON users.login = address.login")
    .as_string();
  let expected_query = "FROM users INNER JOIN address ON users.login = address.login";

  assert_eq!(query, expected_query);
}

#[test]
fn clause_left_join_should_be_after_from_clause() {
  let query = SelectBuilder::new()
    .from("users")
    .left_join("address ON users.login = address.login")
    .as_string();
  let expected_query = "FROM users LEFT JOIN address ON users.login = address.login";

  assert_eq!(query, expected_query);
}

#[test]
fn clause_right_join_should_be_after_from_clause() {
  let query = SelectBuilder::new()
    .from("users")
    .right_join("address ON users.login = address.login")
    .as_string();
  let expected_query = "FROM users RIGHT JOIN address ON users.login = address.login";

  assert_eq!(query, expected_query);
}

#[test]
fn clause_where_should_be_after_any_of_the_joins_clauses() {
  let query = SelectBuilder::new()
    .inner_join("address ON users.login = address.login")
    .where_clause("user.login = $1")
    .as_string();
  let expected_query = "INNER JOIN address ON users.login = address.login WHERE user.login = $1";

  assert_eq!(query, expected_query);
}

#[test]
fn clause_group_by_should_be_after_where_clause() {
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
fn clause_having_should_be_after_group_by_clause() {
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
fn clause_order_by_should_be_after_having_clause() {
  let query = SelectBuilder::new()
    .having("active = true")
    .order_by("created_at desc")
    .as_string();
  let expected_query = "HAVING active = true ORDER BY created_at desc";

  assert_eq!(query, expected_query);
}

#[test]
fn clause_limit_should_be_after_order_by_clause() {
  let query = SelectBuilder::new().order_by("created_at desc").limit("42").as_string();
  let expected_query = "ORDER BY created_at desc LIMIT 42";

  assert_eq!(query, expected_query);
}

#[test]
fn clause_offset_should_be_after_limit_clause() {
  let query = SelectBuilder::new().limit("500").offset("100").as_string();
  let expected_query = "LIMIT 500 OFFSET 100";

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
