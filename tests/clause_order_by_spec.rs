mod select_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_order_by_should_add_the_order_by_clause() {
    let query = sql::Select::new().order_by("id asc").as_string();
    let expected_query = "ORDER BY id asc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_order_by_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Select::new()
      .order_by("login asc")
      .order_by("created_at desc")
      .as_string();
    let expected_query = "ORDER BY login asc, created_at desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_order_by_should_not_accumulate_values_when_column_name_is_empty() {
    let query = sql::Select::new()
      .order_by("")
      .order_by("created_at desc")
      .order_by("")
      .as_string();
    let expected_query = "ORDER BY created_at desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_order_by_should_trim_space_of_the_argument() {
    let query = sql::Select::new().order_by("  id desc  ").as_string();
    let expected_query = "ORDER BY id desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_order_by_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Select::new().order_by("id desc").order_by("id desc").as_string();
    let expected_query = "ORDER BY id desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_order_by_should_be_after_having_clause() {
    let query = sql::Select::new()
      .having("active = true")
      .order_by("created_at desc")
      .as_string();
    let expected_query = "HAVING active = true ORDER BY created_at desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_order_by_should_be_after_window_clause() {
    let query = sql::Select::new().window("foo").order_by("created_at desc").as_string();
    let expected_query = "WINDOW foo ORDER BY created_at desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_order_by_clause() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::OrderBy, "where orders.user_login = $1")
      .order_by("id desc")
      .as_string();
    let expected_query = "where orders.user_login = $1 ORDER BY id desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_order_by_clause() {
    let query = sql::Select::new()
      .order_by("id desc")
      .raw_after(sql::SelectClause::OrderBy, "limit 20")
      .as_string();
    let expected_query = "ORDER BY id desc limit 20";

    assert_eq!(expected_query, query);
  }
}

#[cfg(feature = "mysql")]
mod delete_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_order_by_should_add_the_order_by_clause() {
    let query = sql::Delete::new().order_by("id asc").as_string();
    let expected_query = "ORDER BY id asc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_order_by_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Delete::new()
      .order_by("login asc")
      .order_by("created_at desc")
      .as_string();
    let expected_query = "ORDER BY login asc, created_at desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_order_by_should_not_accumulate_values_when_column_name_is_empty() {
    let query = sql::Delete::new()
      .order_by("")
      .order_by("created_at desc")
      .order_by("")
      .as_string();
    let expected_query = "ORDER BY created_at desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_order_by_should_trim_space_of_the_argument() {
    let query = sql::Delete::new().order_by("  id desc  ").as_string();
    let expected_query = "ORDER BY id desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_order_by_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Delete::new().order_by("id desc").order_by("id desc").as_string();
    let expected_query = "ORDER BY id desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_order_by_should_be_after_where_clause() {
    let query = sql::Delete::new()
      .where_clause("active = true")
      .order_by("created_at desc")
      .as_string();
    let expected_query = "WHERE active = true ORDER BY created_at desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_order_by_should_be_after_delete_from_clause() {
    let query = sql::Delete::new()
      .delete_from("foo")
      .order_by("created_at desc")
      .as_string();
    let expected_query = "DELETE FROM foo ORDER BY created_at desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_order_by_clause() {
    let query = sql::Delete::new()
      .raw_before(sql::DeleteClause::OrderBy, "where user_login = $1")
      .order_by("id desc")
      .as_string();
    let expected_query = "where user_login = $1 ORDER BY id desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_order_by_clause() {
    let query = sql::Delete::new()
      .order_by("id desc")
      .raw_after(sql::DeleteClause::OrderBy, "limit 20")
      .as_string();
    let expected_query = "ORDER BY id desc limit 20";

    assert_eq!(expected_query, query);
  }
}

#[cfg(feature = "mysql")]
mod update_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_order_by_should_add_the_order_by_clause() {
    let query = sql::Update::new().order_by("id asc").as_string();
    let expected_query = "ORDER BY id asc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_order_by_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Update::new()
      .order_by("login asc")
      .order_by("created_at desc")
      .as_string();
    let expected_query = "ORDER BY login asc, created_at desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_order_by_should_not_accumulate_values_when_column_name_is_empty() {
    let query = sql::Update::new()
      .order_by("")
      .order_by("created_at desc")
      .order_by("")
      .as_string();
    let expected_query = "ORDER BY created_at desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_order_by_should_trim_space_of_the_argument() {
    let query = sql::Update::new().order_by("  id desc  ").as_string();
    let expected_query = "ORDER BY id desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_order_by_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Update::new().order_by("id desc").order_by("id desc").as_string();
    let expected_query = "ORDER BY id desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_order_by_should_be_after_where_clause() {
    let query = sql::Update::new()
      .where_clause("active = true")
      .order_by("created_at desc")
      .as_string();
    let expected_query = "WHERE active = true ORDER BY created_at desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn clause_order_by_should_be_after_update_clause() {
    let query = sql::Update::new().update("foo").order_by("created_at desc").as_string();
    let expected_query = "UPDATE foo ORDER BY created_at desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_order_by_clause() {
    let query = sql::Update::new()
      .raw_before(sql::UpdateClause::OrderBy, "where user_login = $1")
      .order_by("id desc")
      .as_string();
    let expected_query = "where user_login = $1 ORDER BY id desc";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_order_by_clause() {
    let query = sql::Update::new()
      .order_by("id desc")
      .raw_after(sql::UpdateClause::OrderBy, "limit 20")
      .as_string();
    let expected_query = "ORDER BY id desc limit 20";

    assert_eq!(expected_query, query);
  }
}
