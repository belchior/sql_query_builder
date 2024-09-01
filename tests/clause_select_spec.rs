mod insert_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_select_should_add_a_select_clause() {
    let query = sql::Insert::new()
      .insert_into("users (login, name)")
      .select(
        sql::Select::new()
          .select("login, name")
          .from("users_bk")
          .where_clause("active = true"),
      )
      .as_string();

    let expected_query = "\
      INSERT INTO users (login, name) \
      SELECT login, name \
      FROM users_bk \
      WHERE active = true\
    ";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_select_should_override_value_on_consecutive_calls() {
    let query = sql::Insert::new()
      .insert_into("users")
      .select(sql::Select::new().select("login, name"))
      .select(sql::Select::new().select("*"))
      .as_string();

    let expected_query = "INSERT INTO users SELECT *";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_select_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::Select, "insert into users")
      .select(sql::Select::new().select("*"))
      .as_string();

    let expected_query = "insert into users SELECT *";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_select_clause() {
    let query = sql::Insert::new()
      .insert_into("users")
      .select(sql::Select::new().select("*"))
      .raw_after(sql::InsertClause::Select, "from users_bk")
      .as_string();
    let expected_query = "INSERT INTO users SELECT * from users_bk";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_select_should_be_after_insert_into_clause() {
    let query = sql::Insert::new()
      .select(sql::Select::new().select("login, name"))
      .insert_into("users (login, name)")
      .as_string();
    let expected_query = "INSERT INTO users (login, name) SELECT login, name";

    assert_eq!(expected_query, query);
  }
}

mod select_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_select_should_add_the_select_clause() {
    let query = sql::Select::new().select("id, login").as_string();
    let expected_query = "SELECT id, login";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_select_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Select::new().select("id, login").select("created_at").as_string();
    let expected_query = "SELECT id, login, created_at";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_select_by_should_trim_space_of_the_argument() {
    let query = sql::Select::new().select("  login, name  ").as_string();
    let expected_query = "SELECT login, name";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_select_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Select::new()
      .select("login, name")
      .select("login, name")
      .as_string();
    let expected_query = "SELECT login, name";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_select_clause() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::Select, "/* list orders */")
      .select("id, name")
      .as_string();
    let expected_query = "/* list orders */ SELECT id, name";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_select_clause() {
    let query = sql::Select::new()
      .select("id, name")
      .raw_after(sql::SelectClause::Select, "from addresses")
      .as_string();
    let expected_query = "SELECT id, name from addresses";

    assert_eq!(expected_query, query);
  }
}
