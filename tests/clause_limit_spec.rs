#[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
mod select_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_limit_should_add_the_limit_clause() {
    let query = sql::Select::new().limit("3").as_string();
    let expected_query = "LIMIT 3";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_limit_should_override_the_current_value() {
    let query = sql::Select::new().limit("3").limit("4").as_string();
    let expected_query = "LIMIT 4";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_limit_should_trim_space_of_the_argument() {
    let query = sql::Select::new().limit("  50  ").as_string();
    let expected_query = "LIMIT 50";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_limit_should_be_after_order_by_clause() {
    let query = sql::Select::new().order_by("created_at desc").limit("42").as_string();
    let expected_query = "ORDER BY created_at desc LIMIT 42";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_limit_clause() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::Limit, "group by id")
      .limit("10")
      .as_string();
    let expected_query = "group by id LIMIT 10";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_limit_clause() {
    let query = sql::Select::new()
      .limit("10")
      .raw_after(sql::SelectClause::Limit, "except select id, login")
      .as_string();
    let expected_query = "LIMIT 10 except select id, login";

    assert_eq!(expected_query, query);
  }
}

#[cfg(any(feature = "sqlite", feature = "mysql"))]
mod delete_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_limit_should_add_the_limit_clause() {
    let query = sql::Delete::new().limit("3").as_string();
    let expected_query = "LIMIT 3";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_limit_should_override_the_current_value() {
    let query = sql::Delete::new().limit("3").limit("4").as_string();
    let expected_query = "LIMIT 4";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_limit_should_trim_space_of_the_argument() {
    let query = sql::Delete::new().limit("  50  ").as_string();
    let expected_query = "LIMIT 50";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_limit_should_be_after_order_by_clause() {
    let query = sql::Delete::new().order_by("created_at desc").limit("42").as_string();
    let expected_query = "ORDER BY created_at desc LIMIT 42";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_limit_clause() {
    let query = sql::Delete::new()
      .raw_before(sql::DeleteClause::Limit, "order by id")
      .limit("10")
      .as_string();
    let expected_query = "order by id LIMIT 10";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_limit_clause() {
    let query = sql::Delete::new()
      .limit("10")
      .raw_after(sql::DeleteClause::Limit, "/* uncommon argument */")
      .as_string();
    let expected_query = "LIMIT 10 /* uncommon argument */";

    assert_eq!(expected_query, query);
  }
}

#[cfg(any(feature = "sqlite", feature = "mysql"))]
mod update_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_limit_should_add_the_limit_clause() {
    let query = sql::Update::new().limit("3").as_string();
    let expected_query = "LIMIT 3";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_limit_should_override_the_current_value() {
    let query = sql::Update::new().limit("3").limit("4").as_string();
    let expected_query = "LIMIT 4";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_limit_should_trim_space_of_the_argument() {
    let query = sql::Update::new().limit("  50  ").as_string();
    let expected_query = "LIMIT 50";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_limit_should_be_after_order_by_clause() {
    let query = sql::Update::new().order_by("created_at desc").limit("42").as_string();
    let expected_query = "ORDER BY created_at desc LIMIT 42";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_limit_clause() {
    let query = sql::Update::new()
      .raw_before(sql::UpdateClause::Limit, "order by id")
      .limit("10")
      .as_string();
    let expected_query = "order by id LIMIT 10";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_limit_clause() {
    let query = sql::Update::new()
      .limit("10")
      .raw_after(sql::UpdateClause::Limit, "/* uncommon argument */")
      .as_string();
    let expected_query = "LIMIT 10 /* uncommon argument */";

    assert_eq!(expected_query, query);
  }
}

#[cfg(feature = "sqlite")]
mod update_command_sqlite {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn clause_limit_should_be_after_returning_clause() {
    let query = sql::Update::new().limit("42").returning("*").as_string();
    let expected_query = "RETURNING * LIMIT 42";

    assert_eq!(expected_query, query);
  }
}
