mod select_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_from_should_add_the_from_clause() {
    let query = sql::Select::new().from("users").as_string();
    let expected_query = "FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_from_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Select::new().from("users").from("addresses").as_string();
    let expected_query = "FROM users, addresses";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_from_should_trim_space_of_the_argument() {
    let query = sql::Select::new().from("  users  ").as_string();
    let expected_query = "FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_from_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Select::new().from("addresses").from("addresses").as_string();
    let expected_query = "FROM addresses";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_from_should_be_after_select_clause() {
    let query = sql::Select::new().select("*").from("users").as_string();
    let expected_query = "SELECT * FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_from_clause() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::From, "select amount")
      .from("orders")
      .as_string();
    let expected_query = "select amount FROM orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_from_clause() {
    let query = sql::Select::new()
      .from("users")
      .raw_after(
        sql::SelectClause::From,
        "inner join addresses on users.login = addresses.login",
      )
      .as_string();
    let expected_query = "FROM users inner join addresses on users.login = addresses.login";

    assert_eq!(query, expected_query);
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod update_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_from_should_add_the_from_clause() {
    let query = sql::Update::new().from("users").as_string();
    let expected_query = "FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_from_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Update::new().from("users").from("addresses").as_string();
    let expected_query = "FROM users, addresses";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_from_should_trim_space_of_the_argument() {
    let query = sql::Update::new().from("  users  ").as_string();
    let expected_query = "FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_from_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Update::new().from("addresses").from("addresses").as_string();
    let expected_query = "FROM addresses";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_from_should_be_after_set_clause() {
    let query = sql::Update::new().set("country = 'Bar'").from("addresses").as_string();
    let expected_query = "SET country = 'Bar' FROM addresses";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_from_clause() {
    let query = sql::Update::new()
      .raw_before(sql::UpdateClause::From, "set country = 'Bar'")
      .from("addresses")
      .as_string();
    let expected_query = "set country = 'Bar' FROM addresses";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_from_clause() {
    let query = sql::Update::new()
      .from("users")
      .raw_after(sql::UpdateClause::From, "where login = $1")
      .as_string();
    let expected_query = "FROM users where login = $1";

    assert_eq!(query, expected_query);
  }
}
