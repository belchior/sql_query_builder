mod where_clause {
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  mod create_index_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_where_clause_should_add_the_where_clause() {
      let query = sql::CreateIndex::new().where_clause("created_at >= $1").as_string();
      let expected_query = "WHERE created_at >= $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_clause_should_omit_the_operation_when_was_the_first_clause() {
      let query = sql::CreateIndex::new().where_clause("status = 'active'").as_string();
      let expected_query = "WHERE status = 'active'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_clause_should_accumulate_values_on_consecutive_calls_using_the_and_operator() {
      let query = sql::CreateIndex::new()
        .where_clause("created_at >= $1")
        .where_clause("status = 'active'")
        .as_string();

      let expected_query = "\
        WHERE \
          created_at >= $1 \
          AND status = 'active'\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_clause_should_not_accumulate_values_when_expression_is_empty() {
      let query = sql::CreateIndex::new()
        .where_clause("")
        .where_clause("status = 'active'")
        .where_clause("")
        .as_string();

      let expected_query = "WHERE status = 'active'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_clause_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::CreateIndex::new()
        .where_clause("status = 'active'")
        .where_clause("status = 'active'")
        .as_string();
      let expected_query = "WHERE status = 'active'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_clause_should_trim_space_of_the_argument() {
      let query = sql::CreateIndex::new()
        .where_clause("  status = 'active'  ")
        .as_string();
      let expected_query = "WHERE status = 'active'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_where_clause() {
      let query = sql::CreateIndex::new()
        .raw_before(sql::CreateIndexParams::Where, "/* uncommon parameter */")
        .where_clause("status = 'active'")
        .as_string();
      let expected_query = "/* uncommon parameter */ WHERE status = 'active'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_where_clause() {
      let query = sql::CreateIndex::new()
        .where_clause("created_at >= $1")
        .raw_after(sql::CreateIndexParams::Where, "and created_at < $2")
        .as_string();
      let expected_query = "WHERE created_at >= $1 and created_at < $2";

      assert_eq!(query, expected_query);
    }
  }

  mod delete_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_where_clause_should_add_the_where_clause() {
      let query = sql::Delete::new().where_clause("id = $1").as_string();
      let expected_query = "WHERE id = $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_clause_should_omit_the_operation_when_was_the_first_clause() {
      let query = sql::Delete::new().where_clause("login = 'foo'").as_string();
      let expected_query = "WHERE login = 'foo'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_clause_should_accumulate_values_on_consecutive_calls_using_the_and_operator() {
      let query = sql::Delete::new()
        .where_clause("id = $1")
        .where_clause("status = 'pending'")
        .as_string();

      let expected_query = "\
        WHERE \
          id = $1 \
          AND status = 'pending'\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_clause_should_not_accumulate_values_when_expression_is_empty() {
      let query = sql::Delete::new()
        .where_clause("")
        .where_clause("status = 'pending'")
        .where_clause("")
        .as_string();

      let expected_query = "WHERE status = 'pending'";

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
    fn method_where_clause_should_trim_space_of_the_argument() {
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
    fn method_where_clause_should_add_the_where_clause() {
      let query = sql::Select::new()
        .where_clause("created_at::date = current_date")
        .as_string();
      let expected_query = "WHERE created_at::date = current_date";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_clause_should_omit_the_operation_when_was_the_first_clause() {
      let query = sql::Select::new().where_clause("login = 'foo'").as_string();
      let expected_query = "WHERE login = 'foo'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_clause_should_accumulate_values_on_consecutive_calls_using_the_and_operator() {
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
    fn method_where_clause_should_not_accumulate_values_when_expression_is_empty() {
      let query = sql::Select::new()
        .where_clause("")
        .where_clause("created_at::date <= current_date")
        .where_clause("")
        .as_string();

      let expected_query = "WHERE created_at::date <= current_date";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_clause_should_trim_space_of_the_argument() {
      let query = sql::Select::new().where_clause("  id = $1  ").as_string();
      let expected_query = "WHERE id = $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_clause_should_not_accumulate_arguments_with_the_same_content() {
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
        .where_clause("user.login = $1")
        .inner_join("addresses ON users.login = addresses.login")
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
    fn method_where_clause_should_add_the_where_clause() {
      let query = sql::Update::new().where_clause("id = $1").as_string();
      let expected_query = "WHERE id = $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_clause_should_accumulate_values_on_consecutive_calls_using_the_and_operator() {
      let query = sql::Update::new()
        .where_clause("id = $1")
        .where_clause("status = 'pending'")
        .as_string();

      let expected_query = "\
        WHERE \
          id = $1 \
          AND status = 'pending'\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_clause_should_not_accumulate_values_when_expression_is_empty() {
      let query = sql::Update::new()
        .where_clause("")
        .where_clause("status = 'pending'")
        .where_clause("")
        .as_string();

      let expected_query = "WHERE status = 'pending'";

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

    #[cfg(any(feature = "postgresql", feature = "sqlite"))]
    #[test]
    fn clause_where_should_be_after_from_clause() {
      let query = sql::Update::new().where_or("user.login = $1").from("users").as_string();
      let expected_query = "FROM users WHERE user.login = $1";

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
    fn method_where_clause_should_trim_space_of_the_argument() {
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

mod where_and {
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  mod create_index_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_where_and_should_be_an_alias_of_where_clause() {
      let query = sql::CreateIndex::new()
        .where_and("active = 'true'")
        .where_and("created_at >= $1")
        .as_string();
      let expected_query = "WHERE active = 'true' AND created_at >= $1";

      assert_eq!(query, expected_query);
    }
  }

  mod delete_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_where_and_should_be_an_alias_of_where_clause() {
      let query = sql::Delete::new().where_and("login = $1").as_string();
      let expected_query = "WHERE login = $1";

      assert_eq!(query, expected_query);
    }
  }

  mod select_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_where_and_should_be_an_alias_of_where_clause() {
      let query = sql::Select::new().where_and("login = $1").as_string();
      let expected_query = "WHERE login = $1";

      assert_eq!(query, expected_query);
    }
  }

  mod update_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_where_and_should_be_an_alias_of_where_clause() {
      let query = sql::Update::new().where_and("login = $1").as_string();
      let expected_query = "WHERE login = $1";

      assert_eq!(query, expected_query);
    }
  }
}

mod where_or {
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  mod create_index_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_where_or_should_add_the_where_clause() {
      let query = sql::CreateIndex::new().where_or("created_at >= $1").as_string();
      let expected_query = "WHERE created_at >= $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_or_should_omit_the_operation_when_was_the_first_clause() {
      let query = sql::CreateIndex::new().where_or("created_at >= $1").as_string();
      let expected_query = "WHERE created_at >= $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_or_should_accumulate_values_on_consecutive_calls_using_the_or_operator() {
      let query = sql::CreateIndex::new()
        .where_or("created_at >= $1")
        .where_or("status = 'active'")
        .as_string();

      let expected_query = "\
        WHERE \
          created_at >= $1 \
          OR status = 'active'\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_or_should_not_accumulate_values_when_expression_is_empty() {
      let query = sql::CreateIndex::new()
        .where_or("")
        .where_or("status = 'active'")
        .where_or("")
        .as_string();

      let expected_query = "WHERE status = 'active'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_or_should_trim_space_of_the_argument() {
      let query = sql::CreateIndex::new().where_or("  status = 'active'  ").as_string();
      let expected_query = "WHERE status = 'active'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_or_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::CreateIndex::new()
        .where_or("status = 'active'")
        .where_or("status = 'active'")
        .as_string();
      let expected_query = "WHERE status = 'active'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_where_clause() {
      let query = sql::CreateIndex::new()
        .raw_before(sql::CreateIndexParams::Where, "/* uncommon parameter */")
        .where_or("status = 'active'")
        .as_string();
      let expected_query = "/* uncommon parameter */ WHERE status = 'active'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_where_clause() {
      let query = sql::CreateIndex::new()
        .where_or("status = 'active'")
        .raw_after(sql::CreateIndexParams::Where, "/* uncommon parameter */")
        .as_string();
      let expected_query = "WHERE status = 'active' /* uncommon parameter */";

      assert_eq!(query, expected_query);
    }
  }

  mod delete_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_where_or_should_add_the_where_clause() {
      let query = sql::Delete::new()
        .where_or("created_at::date = current_date")
        .as_string();
      let expected_query = "WHERE created_at::date = current_date";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_or_should_omit_the_operation_when_was_the_first_clause() {
      let query = sql::Delete::new().where_or("login = 'foo'").as_string();
      let expected_query = "WHERE login = 'foo'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_or_should_accumulate_values_on_consecutive_calls_using_the_or_operator() {
      let query = sql::Delete::new()
        .where_or("login = 'foo'")
        .where_or("login = 'bar'")
        .as_string();

      let expected_query = "\
        WHERE \
          login = 'foo' \
          OR login = 'bar'\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_or_should_not_accumulate_values_when_expression_is_empty() {
      let query = sql::Delete::new()
        .where_or("")
        .where_or("login = 'bar'")
        .where_or("")
        .as_string();

      let expected_query = "WHERE login = 'bar'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_or_should_trim_space_of_the_argument() {
      let query = sql::Delete::new().where_or("  id = $1  ").as_string();
      let expected_query = "WHERE id = $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_or_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Delete::new()
        .where_or("active = true")
        .where_or("active = true")
        .as_string();
      let expected_query = "WHERE active = true";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_where_should_be_after_delete_from_clause() {
      let query = sql::Delete::new()
        .where_or("user.login = $1")
        .delete_from("users")
        .as_string();
      let expected_query = "DELETE FROM users WHERE user.login = $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_where_clause() {
      let query = sql::Delete::new()
        .raw_before(sql::DeleteClause::Where, "delete from orders")
        .where_or("created_at::date = current_date")
        .as_string();
      let expected_query = "delete from orders WHERE created_at::date = current_date";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_where_clause() {
      let query = sql::Delete::new()
        .where_or("created_at::date = current_date")
        .raw_after(sql::DeleteClause::Where, "/* end of the delete clause */")
        .as_string();
      let expected_query = "WHERE created_at::date = current_date /* end of the delete clause */";

      assert_eq!(query, expected_query);
    }
  }

  mod select_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_where_or_should_add_the_where_clause() {
      let query = sql::Select::new()
        .where_or("created_at::date = current_date")
        .as_string();
      let expected_query = "WHERE created_at::date = current_date";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_or_should_omit_the_operation_when_was_the_first_clause() {
      let query = sql::Select::new().where_or("login = 'foo'").as_string();
      let expected_query = "WHERE login = 'foo'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_or_should_accumulate_values_on_consecutive_calls_using_the_or_operator() {
      let query = sql::Select::new()
        .where_or("login = 'foo'")
        .where_or("login = 'bar'")
        .as_string();

      let expected_query = "\
        WHERE \
          login = 'foo' \
          OR login = 'bar'\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_or_should_not_accumulate_values_when_expression_is_empty() {
      let query = sql::Select::new()
        .where_or("")
        .where_or("login = 'bar'")
        .where_or("")
        .as_string();

      let expected_query = "WHERE login = 'bar'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_or_should_trim_space_of_the_argument() {
      let query = sql::Select::new().where_or("  id = $1  ").as_string();
      let expected_query = "WHERE id = $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_or_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Select::new()
        .where_or("active = true")
        .where_or("active = true")
        .as_string();
      let expected_query = "WHERE active = true";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_where_should_be_after_any_of_the_joins_clauses() {
      let query = sql::Select::new()
        .where_or("user.login = $1")
        .inner_join("addresses ON users.login = addresses.login")
        .as_string();
      let expected_query = "INNER JOIN addresses ON users.login = addresses.login WHERE user.login = $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_where_clause() {
      let query = sql::Select::new()
        .raw_before(sql::SelectClause::Where, "from orders")
        .where_or("created_at::date = current_date")
        .as_string();
      let expected_query = "from orders WHERE created_at::date = current_date";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_where_clause() {
      let query = sql::Select::new()
        .where_or("created_at::date = current_date")
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
    fn method_where_or_should_add_the_where_clause() {
      let query = sql::Update::new()
        .where_or("created_at::date = current_date")
        .as_string();
      let expected_query = "WHERE created_at::date = current_date";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_or_should_omit_the_operation_when_was_the_first_clause() {
      let query = sql::Update::new().where_or("login = 'foo'").as_string();
      let expected_query = "WHERE login = 'foo'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_or_should_accumulate_values_on_consecutive_calls_using_the_or_operator() {
      let query = sql::Update::new()
        .where_or("login = 'foo'")
        .where_or("login = 'bar'")
        .as_string();

      let expected_query = "\
        WHERE \
          login = 'foo' \
          OR login = 'bar'\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_or_should_not_accumulate_values_when_expression_is_empty() {
      let query = sql::Update::new()
        .where_or("")
        .where_or("login = 'bar'")
        .where_or("")
        .as_string();

      let expected_query = "WHERE login = 'bar'";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_or_should_trim_space_of_the_argument() {
      let query = sql::Update::new().where_or("  id = $1  ").as_string();
      let expected_query = "WHERE id = $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_where_or_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Update::new()
        .where_or("active = true")
        .where_or("active = true")
        .as_string();
      let expected_query = "WHERE active = true";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_where_should_be_after_set_clause() {
      let query = sql::Update::new()
        .where_or("user.login = $1")
        .set("users.name = 'Foo'")
        .as_string();
      let expected_query = "SET users.name = 'Foo' WHERE user.login = $1";

      assert_eq!(query, expected_query);
    }

    #[cfg(any(feature = "postgresql", feature = "sqlite"))]
    #[test]
    fn clause_where_should_be_after_from_clause() {
      let query = sql::Update::new().where_or("user.login = $1").from("users").as_string();
      let expected_query = "FROM users WHERE user.login = $1";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_where_clause() {
      let query = sql::Update::new()
        .raw_before(sql::UpdateClause::Where, "SET users.name = 'Foo'")
        .where_or("created_at::date = current_date")
        .as_string();
      let expected_query = "SET users.name = 'Foo' WHERE created_at::date = current_date";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_where_clause() {
      let query = sql::Update::new()
        .where_or("created_at::date = current_date")
        .raw_after(sql::UpdateClause::Where, "/* end of the update clause */")
        .as_string();
      let expected_query = "WHERE created_at::date = current_date /* end of the update clause */";

      assert_eq!(query, expected_query);
    }
  }
}
