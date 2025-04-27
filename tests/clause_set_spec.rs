mod update_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_set_should_add_a_set_clause() {
    let query = sql::Update::new().set("login = 'Foo'").as_string();
    let expected_query = "SET login = 'Foo'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_set_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Update::new().set("login = 'foo'").set("name = 'Foo'").as_string();
    let expected_query = "SET login = 'foo', name = 'Foo'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_set_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::Update::new().set("").set("name = 'Foo'").set("").as_string();
    let expected_query = "SET name = 'Foo'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_set_should_trim_space_of_the_argument() {
    let query = sql::Update::new().set("  name = 'Bar'  ").as_string();
    let expected_query = "SET name = 'Bar'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_set_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Update::new().set("name = 'Bar'").set("name = 'Bar'").as_string();
    let expected_query = "SET name = 'Bar'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_set_clause() {
    let query = sql::Update::new()
      .raw_before(sql::UpdateClause::Set, "update users")
      .set("login = 'Bar'")
      .as_string();
    let expected_query = "update users SET login = 'Bar'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_set_clause() {
    let query = sql::Update::new()
      .set("name = 'Bar'")
      .raw_after(sql::UpdateClause::Set, ", login = 'bar'")
      .as_string();
    let expected_query = "SET name = 'Bar' , login = 'bar'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_set_should_be_after_update_clause() {
    let query = sql::Update::new().set("name = 'Bar'").update("users").as_string();
    let expected_query = "UPDATE users SET name = 'Bar'";

    assert_eq!(expected_query, query);
  }
}

#[cfg(feature = "mysql")]
mod insert_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_set_should_add_a_set_clause() {
    let query = sql::Insert::new().set("login = 'Foo'").as_string();
    let expected_query = "SET login = 'Foo'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_set_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Insert::new().set("login = 'foo'").set("name = 'Foo'").as_string();
    let expected_query = "SET login = 'foo', name = 'Foo'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_set_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::Insert::new().set("").set("name = 'Foo'").set("").as_string();
    let expected_query = "SET name = 'Foo'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_set_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().set("  name = 'Bar'  ").as_string();
    let expected_query = "SET name = 'Bar'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_set_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Insert::new().set("name = 'Bar'").set("name = 'Bar'").as_string();
    let expected_query = "SET name = 'Bar'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_set_should_be_after_into_clause() {
    let query = sql::Insert::new().set("name = 'Bar'").into("users").as_string();
    let expected_query = "INTO users SET name = 'Bar'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_set_should_be_after_partition_clause() {
    let query = sql::Insert::new().set("name = 'Bar'").partition("p1").as_string();
    let expected_query = "PARTITION (p1) SET name = 'Bar'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_set_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::Set, "insert into users")
      .set("login = 'Bar'")
      .as_string();
    let expected_query = "insert into users SET login = 'Bar'";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_set_clause() {
    let query = sql::Insert::new()
      .set("name = 'Bar'")
      .raw_after(sql::InsertClause::Set, ", login = 'bar'")
      .as_string();
    let expected_query = "SET name = 'Bar' , login = 'bar'";

    assert_eq!(expected_query, query);
  }
}
