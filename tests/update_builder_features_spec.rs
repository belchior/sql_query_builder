use pretty_assertions::assert_eq;
use sql_query_builder as sql;

#[test]
fn update_builder_should_be_displayable() {
  let update = sql::Update::new().update("users").set("login = 'foo'");

  println!("{}", update);

  let query = update.as_string();
  let expected_query = "UPDATE users SET login = 'foo'";

  assert_eq!(query, expected_query);
}

#[test]
fn update_builder_should_be_debuggable() {
  let update = sql::Update::new()
    .update("users")
    .set("name = 'Foo'")
    .where_clause("login = 'foo'");

  println!("{:?}", update);

  let expected_query = "UPDATE users SET name = 'Foo' WHERE login = 'foo'";
  let query = update.as_string();

  assert_eq!(query, expected_query);
}

#[test]
fn update_builder_should_be_cloneable() {
  let update_foo = sql::Update::new()
    .raw("/* test raw */")
    .raw_before(sql::UpdateClause::Set, "/* test raw_before */")
    .raw_after(sql::UpdateClause::Set, "/* test raw_after */")
    .update("users")
    .set("login = 'foo'");
  let update_foo_bar = update_foo.clone().set("name = 'Bar'");
  let query_foo = update_foo.as_string();
  let query_foo_bar = update_foo_bar.as_string();

  let expected_query_foo = "\
    /* test raw */ \
    UPDATE users \
    /* test raw_before */ \
    SET login = 'foo' \
    /* test raw_after */\
  ";
  let expected_query_foo_bar = "\
    /* test raw */ \
    UPDATE users \
    /* test raw_before */ \
    SET login = 'foo', name = 'Bar' \
    /* test raw_after */\
  ";

  assert_eq!(query_foo, expected_query_foo);
  assert_eq!(query_foo_bar, expected_query_foo_bar);
}

#[test]
fn update_builder_should_be_able_to_conditionally_add_clauses() {
  let mut update = sql::Update::new().update("users").set("name = 'Bar'");

  if true {
    update = update.set("login = 'bar'");
  }

  let query = update.as_string();
  let expected_query = "UPDATE users SET name = 'Bar', login = 'bar'";

  assert_eq!(query, expected_query);
}

#[test]
fn update_builder_should_be_composable() {
  fn update(update: sql::Update) -> sql::Update {
    update.update("users")
  }

  fn sets(update: sql::Update) -> sql::Update {
    update.set("login = 'foo'").set("name = 'Bar'").set("age = 42")
  }

  fn conditions(update: sql::Update) -> sql::Update {
    update
      .where_clause("id = $1")
      .where_clause("active = true")
      .where_clause("created_at::date = current_date")
  }

  fn as_string(update: sql::Update) -> String {
    update.as_string()
  }

  let query = Some(sql::Update::new())
    .map(update)
    .map(sets)
    .map(conditions)
    .map(as_string)
    .unwrap();

  let expected_query = "\
      UPDATE users \
      SET \
        login = 'foo', \
        name = 'Bar', \
        age = 42 \
      WHERE \
        id = $1 \
        AND active = true \
        AND created_at::date = current_date\
    ";

  assert_eq!(query, expected_query);
}
