mod select_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_group_by_should_add_the_group_by_clause() {
    let query = sql::Select::new().group_by("id, login").as_string();
    let expected_query = "GROUP BY id, login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_group_by_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Select::new()
      .group_by("id, login")
      .group_by("created_at")
      .as_string();
    let expected_query = "GROUP BY id, login, created_at";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_group_by_should_trim_space_of_the_argument() {
    let query = sql::Select::new().group_by("  id, login  ").as_string();
    let expected_query = "GROUP BY id, login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_group_by_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Select::new().group_by("status").group_by("status").as_string();
    let expected_query = "GROUP BY status";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_group_by_should_be_after_where_clause() {
    let query = sql::Select::new()
      .group_by("login")
      .where_clause("login = $1")
      .as_string();
    let expected_query = "\
      WHERE login = $1 \
      GROUP BY login\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_group_by_clause() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::GroupBy, "where id = $1")
      .group_by("login")
      .as_string();
    let expected_query = "where id = $1 GROUP BY login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_group_by_clause() {
    let query = sql::Select::new()
      .group_by("login")
      .raw_after(sql::SelectClause::GroupBy, "LIMIT 10")
      .as_string();
    let expected_query = "GROUP BY login LIMIT 10";

    assert_eq!(query, expected_query);
  }
}
