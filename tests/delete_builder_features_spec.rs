use pretty_assertions::assert_eq;
use sql_query_builder as sql;

#[test]
fn delete_builder_should_be_displayable() {
  let delete = sql::Delete::new().delete_from("users").where_clause("login = 'foo'");

  println!("{}", delete);

  let query = delete.as_string();
  let expected_query = "DELETE FROM users WHERE login = 'foo'";

  assert_eq!(query, expected_query);
}

#[test]
fn delete_builder_should_be_debuggable() {
  let delete = sql::Delete::new()
    .delete_from("users")
    .where_clause("name = 'Foo'")
    .where_clause("login = 'foo'");

  println!("{:?}", delete);

  let expected_query = "DELETE FROM users WHERE name = 'Foo' AND login = 'foo'";
  let query = delete.as_string();

  assert_eq!(query, expected_query);
}

#[test]
fn delete_builder_should_be_cloneable() {
  let delete_foo = sql::Delete::new()
    .raw("/* test raw */")
    .delete_from("users")
    .raw_before(sql::DeleteClause::Where, "/* test raw_before */")
    .where_clause("login = 'foo'")
    .raw_after(sql::DeleteClause::Where, "/* test raw_after */");

  let delete_foo_bar = delete_foo.clone().where_clause("name = 'Bar'");

  let query_foo = delete_foo.as_string();
  let query_foo_bar = delete_foo_bar.as_string();

  let expected_query_foo = "\
    /* test raw */ \
    DELETE FROM users \
    /* test raw_before */ \
    WHERE login = 'foo' \
    /* test raw_after */\
  ";
  let expected_query_foo_bar = "\
    /* test raw */ \
    DELETE FROM users \
    /* test raw_before */ \
    WHERE login = 'foo' AND name = 'Bar' \
    /* test raw_after */\
  ";

  assert_eq!(query_foo, expected_query_foo);
  assert_eq!(query_foo_bar, expected_query_foo_bar);
}

#[test]
fn delete_builder_should_be_able_to_conditionally_add_clauses() {
  let mut delete = sql::Delete::new().delete_from("users").where_clause("name = 'Bar'");

  if true {
    delete = delete.where_clause("login = 'bar'");
  }

  let query = delete.as_string();
  let expected_query = "DELETE FROM users WHERE name = 'Bar' AND login = 'bar'";

  assert_eq!(query, expected_query);
}

#[test]
fn delete_builder_should_be_composable() {
  fn delete(delete: sql::Delete) -> sql::Delete {
    delete.delete_from("users")
  }

  fn conditions(delete: sql::Delete) -> sql::Delete {
    delete
      .where_clause("id = $1")
      .where_clause("active = true")
      .where_clause("created_at::date = current_date")
  }

  fn as_string(delete: sql::Delete) -> String {
    delete.as_string()
  }

  let query = Some(sql::Delete::new())
    .map(delete)
    .map(conditions)
    .map(as_string)
    .unwrap();

  let expected_query = "\
      DELETE FROM users \
      WHERE \
        id = $1 \
        AND active = true \
        AND created_at::date = current_date\
    ";

  assert_eq!(query, expected_query);
}
