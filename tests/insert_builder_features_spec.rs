use pretty_assertions::assert_eq;
use sql_query_builder::InsertBuilder;

#[test]
fn insert_builder_should_be_displayable() {
  let insert = InsertBuilder::new()
    .insert_into("users(login, name)")
    .values("('foo', 'Foo')");

  println!("{}", insert);

  let query = insert.as_string();
  let expected_query = "INSERT INTO users(login, name) VALUES ('foo', 'Foo')";

  assert_eq!(query, expected_query);
}

#[test]
fn insert_builder_should_be_debuggable() {
  let insert = InsertBuilder::new()
    .insert_into("users(login, name)")
    .values("('foo', 'Foo')")
    .overriding("user value");

  println!("{:?}", insert);

  let expected_query = "INSERT INTO users(login, name) OVERRIDING user value VALUES ('foo', 'Foo')";
  let query = insert.as_string();

  assert_eq!(query, expected_query);
}

#[test]
fn insert_builder_should_be_cloneable() {
  let insert_foo = InsertBuilder::new()
    .insert_into("users(login, name)")
    .values("('foo', 'Foo')");
  let insert_foo_bar = insert_foo.clone().values("('bar', 'Bar')");
  let query_foo = insert_foo.as_string();
  let query_foo_bar = insert_foo_bar.as_string();

  let expected_query_foo = "INSERT INTO users(login, name) VALUES ('foo', 'Foo')";
  let expected_query_foo_bar = "INSERT INTO users(login, name) VALUES ('foo', 'Foo'), ('bar', 'Bar')";

  assert_eq!(query_foo, expected_query_foo);
  assert_eq!(query_foo_bar, expected_query_foo_bar);
}

#[test]
fn insert_builder_should_be_able_to_dynamically_add_clauses() {
  let mut insert = InsertBuilder::new()
    .insert_into("users(login, name)")
    .values("('bar', 'Bar')");

  if true {
    insert = insert.overriding("system value");
  }

  let query = insert.as_string();
  let expected_query = "INSERT INTO users(login, name) OVERRIDING system value VALUES ('bar', 'Bar')";

  assert_eq!(query, expected_query);
}

#[test]
fn insert_builder_should_be_composable() {
  fn insert(insert: InsertBuilder) -> InsertBuilder {
    insert.insert_into("users (login, name)")
  }

  fn values(insert: InsertBuilder) -> InsertBuilder {
    insert
      .values("('foo', 'Foo')")
      .values("('bar', 'Bar')")
      .values("('max', 'Max')")
  }

  fn as_string(insert: InsertBuilder) -> String {
    insert.as_string()
  }

  let query = Some(InsertBuilder::new())
    .map(insert)
    .map(values)
    .map(as_string)
    .unwrap();

  let expected_query = "\
      INSERT INTO users (login, name) \
      VALUES \
      ('foo', 'Foo'), \
      ('bar', 'Bar'), \
      ('max', 'Max')\
    ";

  assert_eq!(query, expected_query);
}
