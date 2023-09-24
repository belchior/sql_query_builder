mod where_clause {
  mod delete_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_where_should_add_the_where_clause() {
      let query = sql::Delete::new().where_clause("id = $1").as_string();
      let expected_query = "WHERE id = $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_should_accumulate_values_on_consecutive_calls() {
      let query = sql::Delete::new()
        .where_clause("id = $1")
        .where_clause("status = 'pending'")
        .as_string();
      let expected_query = "WHERE id = $1 AND status = 'pending'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_clause_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Delete::new()
        .where_clause("id = $1")
        .where_clause("id = $1")
        .as_string();
      let expected_query = "WHERE id = $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_should_trim_space_of_the_argument() {
      let query = sql::Delete::new().where_clause("  id = $1  ").as_string();
      let expected_query = "WHERE id = $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_where_should_be_after_delete_from_clause() {
      let query = sql::Delete::new()
        .where_clause("name = $1")
        .delete_from("users")
        .as_string();
      let expected_query = "DELETE FROM users WHERE name = $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_where_clause() {
      let query = sql::Delete::new()
        .raw_before(sql::DeleteClause::Where, "delete from users")
        .where_clause("login = $1")
        .as_string();
      let expected_query = "delete from users WHERE login = $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_where_clause() {
      let query = sql::Delete::new()
        .where_clause("created_at::date >= $1")
        .raw_after(sql::DeleteClause::Where, "and created_at::date < $2")
        .as_string();
      let expected_query = "WHERE created_at::date >= $1 and created_at::date < $2";

      assert_eq!(query, expected_query);
    }
  }

  mod select_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_where_should_add_the_where_clause() {
      let query = sql::Select::new()
        .where_clause("created_at::date = current_date")
        .as_string();
      let expected_query = "WHERE created_at::date = current_date";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_should_accumulate_values_on_consecutive_calls() {
      let query = sql::Select::new()
        .where_clause("created_at::date > current_date - INTERVAL '2 days'")
        .where_clause("created_at::date <= current_date")
        .as_string();
      let expected_query = "\
        WHERE \
          created_at::date > current_date - INTERVAL '2 days' \
          AND created_at::date <= current_date\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_by_should_trim_space_of_the_argument() {
      let query = sql::Select::new().where_clause("  id = $1  ").as_string();
      let expected_query = "WHERE id = $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Select::new()
        .where_clause("active = true")
        .where_clause("active = true")
        .as_string();
      let expected_query = "WHERE active = true";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_where_should_be_after_any_of_the_joins_clauses() {
      let query = sql::Select::new()
        .inner_join("addresses ON users.login = addresses.login")
        .where_clause("user.login = $1")
        .as_string();
      let expected_query = "INNER JOIN addresses ON users.login = addresses.login WHERE user.login = $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_where_clause() {
      let query = sql::Select::new()
        .raw_before(sql::SelectClause::Where, "from orders")
        .where_clause("created_at::date = current_date")
        .as_string();
      let expected_query = "from orders WHERE created_at::date = current_date";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_where_clause() {
      let query = sql::Select::new()
        .where_clause("created_at::date = current_date")
        .raw_after(sql::SelectClause::Where, "limit 10")
        .as_string();
      let expected_query = "WHERE created_at::date = current_date limit 10";

      assert_eq!(query, expected_query);
    }
  }

  mod update_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_where_should_add_the_where_clause() {
      let query = sql::Update::new().where_clause("id = $1").as_string();
      let expected_query = "WHERE id = $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_should_accumulate_values_on_consecutive_calls() {
      let query = sql::Update::new()
        .where_clause("id = $1")
        .where_clause("status = 'pending'")
        .as_string();
      let expected_query = "WHERE id = $1 AND status = 'pending'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_where_should_be_after_set_clause() {
      let query = sql::Update::new()
        .set("name = $1")
        .where_clause("login = $2")
        .as_string();
      let expected_query = "SET name = $1 WHERE login = $2";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_clause_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Update::new()
        .where_clause("id = $1")
        .where_clause("id = $1")
        .as_string();
      let expected_query = "WHERE id = $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_should_trim_space_of_the_argument() {
      let query = sql::Update::new().where_clause("  id = $1  ").as_string();
      let expected_query = "WHERE id = $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_where_clause() {
      let query = sql::Update::new()
        .raw_before(sql::UpdateClause::Where, "set name = $1")
        .where_clause("login = $2")
        .as_string();
      let expected_query = "set name = $1 WHERE login = $2";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_where_clause() {
      let query = sql::Update::new()
        .where_clause("created_at::date >= $1")
        .raw_after(sql::UpdateClause::Where, "and created_at::date < $2")
        .as_string();
      let expected_query = "WHERE created_at::date >= $1 and created_at::date < $2";

      assert_eq!(query, expected_query);
    }
  }
}

mod and_clause {
  mod delete_clause {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_and_should_be_an_alias_to_where_clause() {
      let query = sql::Delete::new().and("login = 'foo'").as_string();
      let expected_query = "WHERE login = 'foo'";

      assert_eq!(query, expected_query);
    }
  }

  mod select_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_and_should_be_an_alias_to_where_clause() {
      let query = sql::Select::new().and("login = 'foo'").as_string();
      let expected_query = "WHERE login = 'foo'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_and_should_accumulate_values_on_consecutive_calls() {
      let query = sql::Select::new().and("login = 'foo'").and("active = true").as_string();
      let expected_query = "WHERE login = 'foo' AND active = true";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_and_should_trim_space_of_the_argument() {
      let query = sql::Select::new().and("  name = 'Bar'  ").as_string();
      let expected_query = "WHERE name = 'Bar'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_and_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Select::new()
        .and("status = 'success'")
        .and("status = 'success'")
        .as_string();
      let expected_query = "WHERE status = 'success'";

      assert_eq!(query, expected_query);
    }
  }

  mod update_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_and_should_be_an_alias_to_where_clause() {
      let query = sql::Update::new().and("login = 'foo'").as_string();
      let expected_query = "WHERE login = 'foo'";

      assert_eq!(query, expected_query);
    }
  }
}
