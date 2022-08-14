use pretty_assertions::assert_eq;
use sql_query_builder as sql;

#[test]
fn values_builder_should_be_displayable() {
  let values = sql::Values::new().values("('foo', 'Foo')").values("('bar', 'Bar')");

  println!("{}", values);

  let query = values.as_string();
  let expected_query = "VALUES ('foo', 'Foo'), ('bar', 'Bar')";

  assert_eq!(query, expected_query);
}

#[test]
fn values_builder_should_be_debuggable() {
  let values = sql::Values::new().values("('foo', 'Foo')").values("('bar', 'Bar')");

  println!("{:?}", values);

  let expected_query = "VALUES ('foo', 'Foo'), ('bar', 'Bar')";
  let query = values.as_string();

  assert_eq!(query, expected_query);
}

#[test]
fn values_builder_should_be_cloneable() {
  let values_foo = sql::Values::new()
    .raw("/* test raw */")
    .raw_before(sql::ValuesClause::Values, "/* test raw_before */")
    .values("('foo', 'Foo')")
    .raw_after(sql::ValuesClause::Values, "/* test raw_after */");

  let values_foo_bar = values_foo.clone().values("('bar', 'Bar')");

  let query_foo = values_foo.as_string();
  let query_foo_bar = values_foo_bar.as_string();

  let expected_query_foo = "\
    /* test raw */ \
    /* test raw_before */ \
    VALUES ('foo', 'Foo') \
    /* test raw_after */\
  ";
  let expected_query_foo_bar = "\
    /* test raw */ \
    /* test raw_before */ \
    VALUES ('foo', 'Foo'), ('bar', 'Bar') \
    /* test raw_after */\
  ";

  assert_eq!(query_foo, expected_query_foo);
  assert_eq!(query_foo_bar, expected_query_foo_bar);
}

#[test]
fn values_builder_should_be_able_to_conditionally_add_clauses() {
  let mut values = sql::Values::new().values("('foo', 'Foo')");

  if true {
    values = values.values("('bar', 'Bar')");
  }

  let query = values.as_string();
  let expected_query = "VALUES ('foo', 'Foo'), ('bar', 'Bar')";

  assert_eq!(query, expected_query);
}

#[test]
fn values_builder_should_be_composable() {
  fn value_foo(values: sql::Values) -> sql::Values {
    values.values("('foo', 'Foo')")
  }

  fn value_bar(values: sql::Values) -> sql::Values {
    values.values("('bar', 'Bar')")
  }

  fn as_string(values: sql::Values) -> String {
    values.as_string()
  }

  let query = Some(sql::Values::new())
    .map(value_foo)
    .map(value_bar)
    .map(as_string)
    .unwrap();

  let expected_query = "\
      VALUES ('foo', 'Foo'), ('bar', 'Bar')\
    ";

  assert_eq!(query, expected_query);
}
