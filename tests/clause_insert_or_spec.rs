#[cfg(feature = "sqlite")]
mod insert_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_insert_or_should_add_the_insert_or_clause() {
    let query = sql::Insert::new()
      .insert_or("ABORT INTO users (login, name)")
      .as_string();
    let expected_query = "INSERT OR ABORT INTO users (login, name)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_insert_or_should_override_value_on_consecutive_calls() {
    let query = sql::Insert::new()
      .insert_or("FAIL INTO users (login, name)")
      .insert_or("FAIL INTO orders (product_name, price)")
      .as_string();
    let expected_query = "INSERT OR FAIL INTO orders (product_name, price)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_insert_or_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().insert_or("  IGNORE INTO users (name)  ").as_string();
    let expected_query = "INSERT OR IGNORE INTO users (name)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_insert_or_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::InsertOr, "/* insert or replace */")
      .insert_or("REPLACE INTO users (login)")
      .as_string();
    let expected_query = "/* insert or replace */ INSERT OR REPLACE INTO users (login)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_insert_or_clause() {
    let query = sql::Insert::new()
      .insert_or("ROLLBACK INTO users (name)")
      .raw_after(sql::InsertClause::InsertOr, "values ('foo')")
      .as_string();
    let expected_query = "INSERT OR ROLLBACK INTO users (name) values ('foo')";

    assert_eq!(query, expected_query);
  }
}
