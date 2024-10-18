#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod delete_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_returning_should_add_the_returning_clause() {
    let query = sql::Delete::new().returning("*").as_string();
    let expected_query = "RETURNING *";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_returning_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Delete::new().returning("login").returning("name").as_string();
    let expected_query = "RETURNING login, name";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_returning_should_not_accumulate_values_when_column_name_is_empty() {
    let query = sql::Delete::new()
      .returning("")
      .returning("name")
      .returning("")
      .as_string();
    let expected_query = "RETURNING name";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_returning_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Delete::new().returning("id").returning("id").as_string();
    let expected_query = "RETURNING id";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_returning_should_trim_space_of_the_argument() {
    let query = sql::Delete::new().returning("  login  ").as_string();
    let expected_query = "RETURNING login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_returning_should_be_after_where_clause() {
    let query = sql::Delete::new().returning("id").where_clause("name = $1").as_string();
    let expected_query = "WHERE name = $1 RETURNING id";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_returning_clause() {
    let query = sql::Delete::new()
      .raw_before(sql::DeleteClause::Returning, "delete from users")
      .returning("login")
      .as_string();
    let expected_query = "delete from users RETURNING login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_returning_clause() {
    let query = sql::Delete::new()
      .returning("id")
      .raw_after(sql::DeleteClause::Returning, ", login, name")
      .as_string();
    let expected_query = "RETURNING id , login, name";

    assert_eq!(query, expected_query);
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod insert_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_returning_should_add_the_returning_clause() {
    let query = sql::Insert::new().returning("*").as_string();
    let expected_query = "RETURNING *";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_returning_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Insert::new().returning("login").returning("name").as_string();
    let expected_query = "RETURNING login, name";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_returning_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Insert::new().returning("id").returning("id").as_string();
    let expected_query = "RETURNING id";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_returning_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().returning("  login  ").as_string();
    let expected_query = "RETURNING login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_returning_should_be_after_values_clause() {
    let query = sql::Insert::new()
      .insert_into("(login, name)")
      .returning("login")
      .values("('foo', 'Foo')")
      .as_string();
    let expected_query = "INSERT INTO (login, name) VALUES ('foo', 'Foo') RETURNING login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_returning_should_be_after_on_conflict_clause() {
    let query = sql::Insert::new()
      .insert_into("(login, name)")
      .values("('foo', 'Foo')")
      .on_conflict("do nothing")
      .returning("login")
      .as_string();
    let expected_query = "INSERT INTO (login, name) VALUES ('foo', 'Foo') ON CONFLICT do nothing RETURNING login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_returning_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::Returning, "values ('foo')")
      .returning("login")
      .as_string();
    let expected_query = "values ('foo') RETURNING login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_returning_clause() {
    let query = sql::Insert::new()
      .returning("id")
      .raw_after(sql::InsertClause::Returning, ", login, name")
      .as_string();
    let expected_query = "RETURNING id , login, name";

    assert_eq!(query, expected_query);
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod update_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_returning_should_add_the_returning_clause() {
    let query = sql::Update::new().returning("*").as_string();
    let expected_query = "RETURNING *";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_returning_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Update::new().returning("login").returning("name").as_string();
    let expected_query = "RETURNING login, name";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_returning_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Update::new().returning("id").returning("id").as_string();
    let expected_query = "RETURNING id";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_returning_should_trim_space_of_the_argument() {
    let query = sql::Update::new().returning("  login  ").as_string();
    let expected_query = "RETURNING login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_returning_should_be_after_where_clause() {
    let query = sql::Update::new().returning("id").where_clause("name = $1").as_string();
    let expected_query = "WHERE name = $1 RETURNING id";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_returning_clause() {
    let query = sql::Update::new()
      .raw_before(sql::UpdateClause::Returning, "where login = $1")
      .returning("login")
      .as_string();
    let expected_query = "where login = $1 RETURNING login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_returning_clause() {
    let query = sql::Update::new()
      .returning("id")
      .raw_after(sql::UpdateClause::Returning, ", login, name")
      .as_string();
    let expected_query = "RETURNING id , login, name";

    assert_eq!(query, expected_query);
  }
}
