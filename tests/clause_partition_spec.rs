#[cfg(feature = "mysql")]
mod select_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_partition_should_define_the_partition_clause() {
    let query = sql::Select::new().partition("p0").as_string();
    let expected_query = "PARTITION (p0)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_partition_should_not_defined_the_clause_without_partition_names() {
    let query = sql::Select::new().partition("").as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_partition_should_accumulate_names_on_consecutive_calls() {
    let query = sql::Select::new().partition("p0").partition("p1").as_string();

    let expected_query = "PARTITION (p0, p1)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_partition_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::Select::new()
      .partition("")
      .partition("p0")
      .partition("")
      .as_string();

    let expected_query = "PARTITION (p0)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_partition_should_not_accumulate_names_with_the_same_content() {
    let query = sql::Select::new().partition("p0").partition("p0").as_string();
    let expected_query = "PARTITION (p0)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_partition_should_trim_space_of_the_argument() {
    let query = sql::Select::new().partition("  p0  ").as_string();
    let expected_query = "PARTITION (p0)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_partition_should_be_defined_after_from_clause() {
    let query = sql::Select::new().from("employees").partition("p0").as_string();
    let expected_query = "FROM employees PARTITION (p0)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_partition_should_be_defined_after_join_clauses() {
    let query = sql::Select::new()
      .from("employees")
      .inner_join("addresses ON employees.login = addresses.login")
      .partition("p0")
      .as_string();

    let expected_query = "\
      FROM employees \
      INNER JOIN addresses ON employees.login = addresses.login \
      PARTITION (p0)\
    ";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_partition_parameter() {
    let query = sql::Select::new()
      .partition("name")
      .raw_after(sql::SelectClause::Partition, "/* uncommon parameter */")
      .as_string();

    let expected_query = "PARTITION (name) /* uncommon parameter */";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_partition_parameter() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::Partition, "/* uncommon parameter */")
      .partition("name")
      .as_string();

    let expected_query = "/* uncommon parameter */ PARTITION (name)";

    assert_eq!(expected_query, query);
  }
}
