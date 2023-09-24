mod select_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_having_should_add_the_having_clause() {
    let query = sql::Select::new().having("active = true").as_string();
    let expected_query = "HAVING active = true";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_having_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Select::new()
      .having("active = true")
      .having("allow = true")
      .as_string();
    let expected_query = "HAVING active = true AND allow = true";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_having_should_trim_space_of_the_argument() {
    let query = sql::Select::new().having("  sum(amount) > 500  ").as_string();
    let expected_query = "HAVING sum(amount) > 500";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_having_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Select::new()
      .having("active = true")
      .having("active = true")
      .as_string();
    let expected_query = "HAVING active = true";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_having_should_be_after_group_by_clause() {
    let query = sql::Select::new().having("active = true").group_by("login").as_string();
    let expected_query = "\
      GROUP BY login \
      HAVING active = true\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_having_clause() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::Having, "group by id")
      .having("active = true")
      .as_string();
    let expected_query = "group by id HAVING active = true";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_having_clause() {
    let query = sql::Select::new()
      .having("active = true")
      .raw_after(sql::SelectClause::Having, "LIMIT 10")
      .as_string();
    let expected_query = "HAVING active = true LIMIT 10";

    assert_eq!(query, expected_query);
  }
}
