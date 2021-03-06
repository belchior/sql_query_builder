use pretty_assertions::assert_eq;
use sql_query_builder::DeleteBuilder;

#[test]
fn delete_builder_should_be_displayable() {
  let delete = DeleteBuilder::new().delete_from("users").where_clause("login = 'foo'");

  println!("{}", delete);

  let query = delete.as_string();
  let expected_query = "DELETE FROM users WHERE login = 'foo'";

  assert_eq!(query, expected_query);
}

#[test]
fn delete_builder_should_be_debuggable() {
  let delete = DeleteBuilder::new()
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
  let delete_foo = DeleteBuilder::new().delete_from("users").where_clause("login = 'foo'");
  let delete_foo_bar = delete_foo.clone().where_clause("name = 'Bar'");
  let query_foo = delete_foo.as_string();
  let query_foo_bar = delete_foo_bar.as_string();

  let expected_query_foo = "DELETE FROM users WHERE login = 'foo'";
  let expected_query_foo_bar = "DELETE FROM users WHERE login = 'foo' AND name = 'Bar'";

  assert_eq!(query_foo, expected_query_foo);
  assert_eq!(query_foo_bar, expected_query_foo_bar);
}

#[test]
fn delete_builder_should_be_able_to_conditionally_add_clauses() {
  let mut delete = DeleteBuilder::new().delete_from("users").where_clause("name = 'Bar'");

  if true {
    delete = delete.where_clause("login = 'bar'");
  }

  let query = delete.as_string();
  let expected_query = "DELETE FROM users WHERE name = 'Bar' AND login = 'bar'";

  assert_eq!(query, expected_query);
}

#[test]
fn delete_builder_should_be_composable() {
  fn delete(delete: DeleteBuilder) -> DeleteBuilder {
    delete.delete_from("users")
  }

  fn conditions(delete: DeleteBuilder) -> DeleteBuilder {
    delete
      .where_clause("id = $1")
      .where_clause("active = true")
      .where_clause("created_at::date = current_date")
  }

  fn as_string(delete: DeleteBuilder) -> String {
    delete.as_string()
  }

  let query = Some(DeleteBuilder::new())
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
