mod insert_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_insert_into_should_add_a_insert_into_clause() {
    let query = sql::Insert::new().insert_into("users (login, name)").as_string();
    let expected_query = "INSERT INTO users (login, name)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_insert_into_should_override_value_on_consecutive_calls() {
    let query = sql::Insert::new()
      .insert_into("users (login, name)")
      .insert_into("orders (product_name, price)")
      .as_string();
    let expected_query = "INSERT INTO orders (product_name, price)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_insert_into_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().insert_into("  users (name)  ").as_string();
    let expected_query = "INSERT INTO users (name)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_insert_into_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::InsertInto, "/* insert into users */")
      .insert_into("users")
      .as_string();
    let expected_query = "/* insert into users */ INSERT INTO users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_insert_into_clause() {
    let query = sql::Insert::new()
      .insert_into("users (name)")
      .raw_after(sql::InsertClause::InsertInto, "values ('foo')")
      .as_string();
    let expected_query = "INSERT INTO users (name) values ('foo')";

    assert_eq!(query, expected_query);
  }
}

#[cfg(feature = "sqlite")]
mod insert_command_sqlite {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_insert_into_should_add_a_insert_into_clause() {
    let query = sql::Insert::new().insert_into("users (login, name)").as_string();
    let expected_query = "INSERT INTO users (login, name)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_insert_into_should_override_value_on_consecutive_calls() {
    let query = sql::Insert::new()
      .insert_into("users (login, name)")
      .insert_into("orders (product_name, price)")
      .as_string();
    let expected_query = "INSERT INTO orders (product_name, price)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_insert_into_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().insert_into("  users (name)  ").as_string();
    let expected_query = "INSERT INTO users (name)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_insert_into_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::InsertInto, "/* insert into users */")
      .insert_into("users")
      .as_string();
    let expected_query = "/* insert into users */ INSERT INTO users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_insert_into_clause() {
    let query = sql::Insert::new()
      .insert_into("users (name)")
      .raw_after(sql::InsertClause::InsertInto, "values ('foo')")
      .as_string();
    let expected_query = "INSERT INTO users (name) values ('foo')";

    assert_eq!(query, expected_query);
  }
}
