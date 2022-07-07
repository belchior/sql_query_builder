#[cfg(feature = "postgresql")]
mod delete_builder_returning_clause {
  use pretty_assertions::assert_eq;
  use sql_query_builder::{DeleteBuilder, DeleteClause};

  #[test]
  fn method_returning_should_add_the_returning_clause() {
    let query = DeleteBuilder::new().returning("*").as_string();
    let expected_query = "RETURNING *";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_returning_should_accumulate_values_on_consecutive_calls() {
    let query = DeleteBuilder::new().returning("login").returning("name").as_string();
    let expected_query = "RETURNING login, name";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_returning_should_not_accumulate_arguments_with_the_same_content() {
    let query = DeleteBuilder::new().returning("id").returning("id").as_string();
    let expected_query = "RETURNING id";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_returning_should_trim_space_of_the_argument() {
    let query = DeleteBuilder::new().returning("  login  ").as_string();
    let expected_query = "RETURNING login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_returning_should_be_after_where_clause() {
    let query = DeleteBuilder::new()
      .returning("id")
      .where_clause("name = $1")
      .as_string();
    let expected_query = "WHERE name = $1 RETURNING id";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_returning_clause() {
    let query = DeleteBuilder::new()
      .raw_before(DeleteClause::Returning, "delete from users")
      .returning("login")
      .as_string();
    let expected_query = "delete from users RETURNING login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_returning_clause() {
    let query = DeleteBuilder::new()
      .returning("id")
      .raw_after(DeleteClause::Returning, ", login, name")
      .as_string();
    let expected_query = "RETURNING id , login, name";

    assert_eq!(query, expected_query);
  }
}

#[cfg(feature = "postgresql")]
mod insert_builder_returning_clause {
  use pretty_assertions::assert_eq;
  use sql_query_builder::{InsertBuilder, InsertClause};

  #[test]
  fn method_returning_should_add_the_returning_clause() {
    let query = InsertBuilder::new().returning("*").as_string();
    let expected_query = "RETURNING *";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_returning_should_accumulate_values_on_consecutive_calls() {
    let query = InsertBuilder::new().returning("login").returning("name").as_string();
    let expected_query = "RETURNING login, name";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_returning_should_not_accumulate_arguments_with_the_same_content() {
    let query = InsertBuilder::new().returning("id").returning("id").as_string();
    let expected_query = "RETURNING id";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_returning_should_trim_space_of_the_argument() {
    let query = InsertBuilder::new().returning("  login  ").as_string();
    let expected_query = "RETURNING login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_returning_should_be_after_values_clause() {
    let query = InsertBuilder::new()
      .insert_into("(login, name)")
      .returning("login")
      .values("('foo', 'Foo')")
      .as_string();
    let expected_query = "INSERT INTO (login, name) VALUES ('foo', 'Foo') RETURNING login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_returning_clause() {
    let query = InsertBuilder::new()
      .raw_before(InsertClause::Returning, "values ('foo')")
      .returning("login")
      .as_string();
    let expected_query = "values ('foo') RETURNING login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_returning_clause() {
    let query = InsertBuilder::new()
      .returning("id")
      .raw_after(InsertClause::Returning, ", login, name")
      .as_string();
    let expected_query = "RETURNING id , login, name";

    assert_eq!(query, expected_query);
  }
}

#[cfg(feature = "postgresql")]
mod select_builder_with_clause {
  use pretty_assertions::assert_eq;
  use sql_query_builder::{SelectBuilder, SelectClause};

  #[test]
  fn method_with_should_add_the_with_clause() {
    let select_users = SelectBuilder::new().select("login").from("users");
    let query = SelectBuilder::new().with("user_list", select_users).as_string();
    let expected_query = "WITH user_list AS (SELECT login FROM users)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_with_should_accept_inline_argument() {
    let query = SelectBuilder::new()
      .with("user_list", SelectBuilder::new().select("login").from("users"))
      .as_string();
    let expected_query = "WITH user_list AS (SELECT login FROM users)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_with_should_accumulate_values_on_consecutive_calls() {
    let select_users = SelectBuilder::new().select("id, login").from("users");
    let select_users_id = SelectBuilder::new().select("id").from("user_list");
    let query = SelectBuilder::new()
      .with("user_list", select_users)
      .with("user_ids", select_users_id)
      .as_string();
    let expected_query = "\
      WITH user_list AS (SELECT id, login FROM users), user_ids AS (SELECT id FROM user_list)\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_with_should_trim_space_of_the_argument() {
    let query = SelectBuilder::new()
      .with("  date  ", SelectBuilder::new().select("current_date"))
      .as_string();
    let expected_query = "WITH date AS (SELECT current_date)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_with_should_be_after_raw() {
    let select_base = SelectBuilder::new()
      .raw("select 123 as id union")
      .with("user_list", SelectBuilder::new().select("*").from("users"))
      .select("id");
    let query = select_base.as_string();
    let expected_query = "\
    select 123 as id union \
    WITH user_list AS (SELECT * FROM users) \
    SELECT id\
  ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_with_clause() {
    let query = SelectBuilder::new()
      .raw_before(SelectClause::With, "/* the users orders */")
      .with("orders_list", SelectBuilder::new().select("*").from("orders"))
      .as_string();
    let expected_query = "/* the users orders */ WITH orders_list AS (SELECT * FROM orders)";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_with_clause() {
    let query = SelectBuilder::new()
      .with("address_list", SelectBuilder::new().select("*").from("address"))
      .raw_after(SelectClause::With, "select name, login")
      .as_string();
    let expected_query = "WITH address_list AS (SELECT * FROM address) select name, login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_select_should_be_after_with_clause() {
    let select_users = SelectBuilder::new().select("*").from("users");
    let select_base = SelectBuilder::new().with("user_list", select_users).select("id");
    let query = select_base.as_string();
    let expected_query = "\
      WITH user_list AS (SELECT * FROM users) \
      SELECT id\
    ";

    assert_eq!(query, expected_query);
  }
}

#[cfg(feature = "postgresql")]
mod update_builder_returning_clause {
  use pretty_assertions::assert_eq;
  use sql_query_builder::{UpdateBuilder, UpdateClause};

  #[test]
  fn method_returning_should_add_the_returning_clause() {
    let query = UpdateBuilder::new().returning("*").as_string();
    let expected_query = "RETURNING *";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_returning_should_accumulate_values_on_consecutive_calls() {
    let query = UpdateBuilder::new().returning("login").returning("name").as_string();
    let expected_query = "RETURNING login, name";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_returning_should_not_accumulate_arguments_with_the_same_content() {
    let query = UpdateBuilder::new().returning("id").returning("id").as_string();
    let expected_query = "RETURNING id";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_returning_should_trim_space_of_the_argument() {
    let query = UpdateBuilder::new().returning("  login  ").as_string();
    let expected_query = "RETURNING login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_returning_should_be_after_where_clause() {
    let query = UpdateBuilder::new()
      .returning("id")
      .where_clause("name = $1")
      .as_string();
    let expected_query = "WHERE name = $1 RETURNING id";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_returning_clause() {
    let query = UpdateBuilder::new()
      .raw_before(UpdateClause::Returning, "where login = $1")
      .returning("login")
      .as_string();
    let expected_query = "where login = $1 RETURNING login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_returning_clause() {
    let query = UpdateBuilder::new()
      .returning("id")
      .raw_after(UpdateClause::Returning, ", login, name")
      .as_string();
    let expected_query = "RETURNING id , login, name";

    assert_eq!(query, expected_query);
  }
}
