use pretty_assertions::assert_eq;
use sql_query_builder as sql;

#[test]
fn insert_builder_should_be_displayable() {
  let insert = sql::Insert::new()
    .insert_into("users(login, name)")
    .values("('foo', 'Foo')");

  println!("{}", insert);

  let query = insert.as_string();
  let expected_query = "INSERT INTO users(login, name) VALUES ('foo', 'Foo')";

  assert_eq!(query, expected_query);
}

#[test]
fn insert_builder_should_be_debuggable() {
  let insert = sql::Insert::new()
    .insert_into("users(login, name)")
    .values("('foo', 'Foo')");

  println!("{:?}", insert);

  let expected_query = "INSERT INTO users(login, name) VALUES ('foo', 'Foo')";
  let query = insert.as_string();

  assert_eq!(query, expected_query);
}

#[test]
fn insert_builder_should_be_cloneable() {
  let insert_foo = sql::Insert::new()
    .raw("/* test raw */")
    .raw_before(sql::InsertClause::Values, "/* test raw_before */")
    .raw_after(sql::InsertClause::Values, "/* test raw_after */")
    .insert_into("users(login, name)")
    .values("('foo', 'Foo')");
  let insert_foo_bar = insert_foo.clone().values("('bar', 'Bar')");
  let query_foo = insert_foo.as_string();
  let query_foo_bar = insert_foo_bar.as_string();

  let expected_query_foo = "\
    /* test raw */ \
    INSERT INTO users(login, name) \
    /* test raw_before */ \
    VALUES ('foo', 'Foo') \
    /* test raw_after */\
  ";
  let expected_query_foo_bar = "\
    /* test raw */ \
    INSERT INTO users(login, name) \
    /* test raw_before */ \
    VALUES ('foo', 'Foo'), ('bar', 'Bar') \
    /* test raw_after */\
  ";

  assert_eq!(query_foo, expected_query_foo);
  assert_eq!(query_foo_bar, expected_query_foo_bar);
}

#[test]
fn insert_builder_should_be_able_to_conditionally_add_clauses() {
  let mut insert = sql::Insert::new()
    .insert_into("users (login, name)")
    .values("('bar', 'Bar')");

  if true {
    insert = insert.values("('foo', 'Foo')");
  }

  let query = insert.as_string();
  let expected_query = "INSERT INTO users (login, name) VALUES ('bar', 'Bar'), ('foo', 'Foo')";

  assert_eq!(query, expected_query);
}

#[test]
fn insert_builder_should_be_composable() {
  fn insert(insert: sql::Insert) -> sql::Insert {
    insert.insert_into("users (login, name)")
  }

  fn values(insert: sql::Insert) -> sql::Insert {
    insert
      .values("('foo', 'Foo')")
      .values("('bar', 'Bar')")
      .values("('max', 'Max')")
  }

  fn as_string(insert: sql::Insert) -> String {
    insert.as_string()
  }

  let query = Some(sql::Insert::new()).map(insert).map(values).map(as_string).unwrap();

  let expected_query = "\
      INSERT INTO users (login, name) \
      VALUES \
      ('foo', 'Foo'), \
      ('bar', 'Bar'), \
      ('max', 'Max')\
    ";

  assert_eq!(query, expected_query);
}
