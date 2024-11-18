mod raw_methods {
  #[cfg(feature = "mysql")]
  mod delete_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_raw_after_should_add_raw_sql_after_join_clause() {
      let query = sql::Delete::new()
        .inner_join("orders ON orders.user_id = user.id")
        .raw_after(sql::DeleteClause::Join, "WHERE user.id = $1")
        .as_string();
      let expected_query = "INNER JOIN orders ON orders.user_id = user.id WHERE user.id = $1";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_join_clause() {
      let query = sql::Delete::new()
        .raw_before(sql::DeleteClause::Join, "FROM users")
        .inner_join("orders ON orders.user_id = user.id")
        .as_string();
      let expected_query = "FROM users INNER JOIN orders ON orders.user_id = user.id";

      assert_eq!(expected_query, query);
    }
  }

  mod select_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_raw_after_should_add_raw_sql_after_join_clause() {
      let query = sql::Select::new()
        .inner_join("addresses ON users.login = addresses.login")
        .raw_after(sql::SelectClause::Join, "where id = $1")
        .as_string();
      let expected_query = "INNER JOIN addresses ON users.login = addresses.login where id = $1";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_join_clause() {
      let query = sql::Select::new()
        .raw_before(sql::SelectClause::Join, "from orders")
        .inner_join("addresses ON addresses.user_login = orders.user_login")
        .as_string();
      let expected_query = "from orders INNER JOIN addresses ON addresses.user_login = orders.user_login";

      assert_eq!(expected_query, query);
    }
  }

  #[cfg(feature = "sqlite")]
  mod update_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_raw_after_should_add_raw_sql_after_join_clause() {
      let query = sql::Update::new()
        .inner_join("orders ON orders.user_id = user.id")
        .raw_after(sql::UpdateClause::Join, "WHERE user.id = $1")
        .as_string();
      let expected_query = "INNER JOIN orders ON orders.user_id = user.id WHERE user.id = $1";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_join_clause() {
      let query = sql::Update::new()
        .raw_before(sql::UpdateClause::Join, "FROM users")
        .inner_join("orders ON orders.user_id = user.id")
        .as_string();
      let expected_query = "FROM users INNER JOIN orders ON orders.user_id = user.id";

      assert_eq!(expected_query, query);
    }
  }
}

mod cross_join_clause {
  #[cfg(feature = "mysql")]
  mod delete_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_cross_join_should_add_the_cross_join_clause() {
      let query = sql::Delete::new().cross_join("addresses").as_string();
      let expected_query = "CROSS JOIN addresses";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_cross_join_should_accumulate_values_on_consecutive_calls() {
      let query = sql::Delete::new()
        .cross_join("addresses")
        .cross_join("orders")
        .as_string();
      let expected_query = "\
        CROSS JOIN addresses \
        CROSS JOIN orders\
      ";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_cross_join_should_not_accumulate_values_when_table_name_is_empty() {
      let query = sql::Delete::new()
        .cross_join("")
        .cross_join("orders")
        .cross_join("")
        .as_string();
      let expected_query = "CROSS JOIN orders";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_cross_join_by_should_trim_space_of_the_argument() {
      let query = sql::Delete::new().cross_join("  orders  ").as_string();
      let expected_query = "CROSS JOIN orders";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_cross_join_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Delete::new()
        .cross_join("addresses")
        .cross_join("addresses")
        .as_string();
      let expected_query = "CROSS JOIN addresses";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn clause_cross_join_should_be_after_from_clause() {
      let query = sql::Delete::new().from("users").cross_join("addresses").as_string();
      let expected_query = "FROM users CROSS JOIN addresses";

      assert_eq!(expected_query, query);
    }
  }

  mod select_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_cross_join_should_add_the_cross_join_clause() {
      let query = sql::Select::new().cross_join("addresses").as_string();
      let expected_query = "CROSS JOIN addresses";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_cross_join_should_accumulate_values_on_consecutive_calls() {
      let query = sql::Select::new()
        .cross_join("addresses")
        .cross_join("orders")
        .as_string();
      let expected_query = "\
        CROSS JOIN addresses \
        CROSS JOIN orders\
      ";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_cross_join_should_not_accumulate_values_when_table_name_is_empty() {
      let query = sql::Select::new()
        .cross_join("")
        .cross_join("orders")
        .cross_join("")
        .as_string();
      let expected_query = "CROSS JOIN orders";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_cross_join_by_should_trim_space_of_the_argument() {
      let query = sql::Select::new().cross_join("  orders  ").as_string();
      let expected_query = "CROSS JOIN orders";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_cross_join_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Select::new()
        .cross_join("addresses")
        .cross_join("addresses")
        .as_string();
      let expected_query = "CROSS JOIN addresses";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn clause_cross_join_should_be_after_from_clause() {
      let query = sql::Select::new().from("users").cross_join("addresses").as_string();
      let expected_query = "FROM users CROSS JOIN addresses";

      assert_eq!(expected_query, query);
    }
  }

  #[cfg(feature = "sqlite")]
  mod update_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_cross_join_should_add_the_cross_join_clause() {
      let query = sql::Update::new().cross_join("addresses").as_string();
      let expected_query = "CROSS JOIN addresses";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_cross_join_should_accumulate_values_on_consecutive_calls() {
      let query = sql::Update::new()
        .cross_join("addresses")
        .cross_join("orders")
        .as_string();
      let expected_query = "\
        CROSS JOIN addresses \
        CROSS JOIN orders\
      ";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_cross_join_should_not_accumulate_values_when_table_name_is_empty() {
      let query = sql::Update::new()
        .cross_join("")
        .cross_join("orders")
        .cross_join("")
        .as_string();
      let expected_query = "CROSS JOIN orders";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_cross_join_by_should_trim_space_of_the_argument() {
      let query = sql::Update::new().cross_join("  orders  ").as_string();
      let expected_query = "CROSS JOIN orders";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_cross_join_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Update::new()
        .cross_join("addresses")
        .cross_join("addresses")
        .as_string();
      let expected_query = "CROSS JOIN addresses";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn clause_cross_join_should_be_after_from_clause() {
      let query = sql::Update::new().from("users").cross_join("addresses").as_string();
      let expected_query = "FROM users CROSS JOIN addresses";

      assert_eq!(expected_query, query);
    }
  }
}

mod inner_join_clause {
  #[cfg(feature = "mysql")]
  mod delete_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_inner_join_should_add_the_inner_join_clause() {
      let query = sql::Delete::new()
        .inner_join("addresses ON users.login = addresses.login")
        .as_string();
      let expected_query = "INNER JOIN addresses ON users.login = addresses.login";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_inner_join_should_accumulate_values_on_consecutive_calls() {
      let query = sql::Delete::new()
        .inner_join("addresses ON users.login = addresses.login")
        .inner_join("orders ON users.login = orders.login")
        .as_string();
      let expected_query = "\
        INNER JOIN addresses ON users.login = addresses.login \
        INNER JOIN orders ON users.login = orders.login\
      ";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_inner_join_should_not_accumulate_values_when_table_expression_is_empty() {
      let query = sql::Delete::new()
        .inner_join("")
        .inner_join("orders ON users.login = orders.login")
        .inner_join("")
        .as_string();
      let expected_query = "INNER JOIN orders ON users.login = orders.login";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_inner_join_by_should_trim_space_of_the_argument() {
      let query = sql::Delete::new().inner_join("  orders  ").as_string();
      let expected_query = "INNER JOIN orders";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_inner_join_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Delete::new()
        .inner_join("addresses")
        .inner_join("addresses")
        .as_string();
      let expected_query = "INNER JOIN addresses";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn clause_inner_join_should_be_after_from_clause() {
      let query = sql::Delete::new()
        .from("users")
        .inner_join("addresses ON users.login = addresses.login")
        .as_string();
      let expected_query = "FROM users INNER JOIN addresses ON users.login = addresses.login";

      assert_eq!(expected_query, query);
    }
  }

  mod select_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_inner_join_should_add_the_inner_join_clause() {
      let query = sql::Select::new()
        .inner_join("addresses ON users.login = addresses.login")
        .as_string();
      let expected_query = "INNER JOIN addresses ON users.login = addresses.login";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_inner_join_should_accumulate_values_on_consecutive_calls() {
      let query = sql::Select::new()
        .inner_join("addresses ON users.login = addresses.login")
        .inner_join("orders ON users.login = orders.login")
        .as_string();
      let expected_query = "\
        INNER JOIN addresses ON users.login = addresses.login \
        INNER JOIN orders ON users.login = orders.login\
      ";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_inner_join_should_not_accumulate_values_when_table_expression_is_empty() {
      let query = sql::Select::new()
        .inner_join("")
        .inner_join("orders ON users.login = orders.login")
        .inner_join("")
        .as_string();
      let expected_query = "INNER JOIN orders ON users.login = orders.login";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_inner_join_by_should_trim_space_of_the_argument() {
      let query = sql::Select::new().inner_join("  orders  ").as_string();
      let expected_query = "INNER JOIN orders";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_inner_join_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Select::new()
        .inner_join("addresses")
        .inner_join("addresses")
        .as_string();
      let expected_query = "INNER JOIN addresses";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn clause_inner_join_should_be_after_from_clause() {
      let query = sql::Select::new()
        .from("users")
        .inner_join("addresses ON users.login = addresses.login")
        .as_string();
      let expected_query = "FROM users INNER JOIN addresses ON users.login = addresses.login";

      assert_eq!(expected_query, query);
    }
  }

  #[cfg(feature = "sqlite")]
  mod update_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_inner_join_should_add_the_inner_join_clause() {
      let query = sql::Update::new()
        .inner_join("addresses ON users.login = addresses.login")
        .as_string();
      let expected_query = "INNER JOIN addresses ON users.login = addresses.login";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_inner_join_should_accumulate_values_on_consecutive_calls() {
      let query = sql::Update::new()
        .inner_join("addresses ON users.login = addresses.login")
        .inner_join("orders ON users.login = orders.login")
        .as_string();
      let expected_query = "\
        INNER JOIN addresses ON users.login = addresses.login \
        INNER JOIN orders ON users.login = orders.login\
      ";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_inner_join_should_not_accumulate_values_when_table_expression_is_empty() {
      let query = sql::Update::new()
        .inner_join("")
        .inner_join("orders ON users.login = orders.login")
        .inner_join("")
        .as_string();
      let expected_query = "INNER JOIN orders ON users.login = orders.login";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_inner_join_by_should_trim_space_of_the_argument() {
      let query = sql::Update::new().inner_join("  orders  ").as_string();
      let expected_query = "INNER JOIN orders";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_inner_join_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Update::new()
        .inner_join("addresses")
        .inner_join("addresses")
        .as_string();
      let expected_query = "INNER JOIN addresses";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn clause_inner_join_should_be_after_from_clause() {
      let query = sql::Update::new()
        .from("users")
        .inner_join("addresses ON users.login = addresses.login")
        .as_string();
      let expected_query = "FROM users INNER JOIN addresses ON users.login = addresses.login";

      assert_eq!(expected_query, query);
    }
  }
}

mod left_join_clause {
  #[cfg(feature = "mysql")]
  mod delete_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_left_join_should_add_the_left_join_clause() {
      let query = sql::Delete::new()
        .left_join("addresses ON users.login = addresses.login")
        .as_string();
      let expected_query = "LEFT JOIN addresses ON users.login = addresses.login";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_left_join_should_accumulate_values_on_consecutive_calls() {
      let query = sql::Delete::new()
        .left_join("addresses ON users.login = addresses.login")
        .left_join("orders ON users.login = orders.login")
        .as_string();
      let expected_query = "\
        LEFT JOIN addresses ON users.login = addresses.login \
        LEFT JOIN orders ON users.login = orders.login\
      ";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_left_join_should_not_accumulate_values_when_table_expression_is_empty() {
      let query = sql::Delete::new()
        .left_join("")
        .left_join("orders ON users.login = orders.login")
        .left_join("")
        .as_string();
      let expected_query = "LEFT JOIN orders ON users.login = orders.login";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_left_join_by_should_trim_space_of_the_argument() {
      let query = sql::Delete::new().left_join("  orders  ").as_string();
      let expected_query = "LEFT JOIN orders";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_left_join_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Delete::new()
        .left_join("addresses")
        .left_join("addresses")
        .as_string();
      let expected_query = "LEFT JOIN addresses";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn clause_left_join_should_be_after_from_clause() {
      let query = sql::Delete::new()
        .from("users")
        .left_join("addresses ON users.login = addresses.login")
        .as_string();
      let expected_query = "FROM users LEFT JOIN addresses ON users.login = addresses.login";

      assert_eq!(expected_query, query);
    }
  }

  mod select_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_left_join_should_add_the_left_join_clause() {
      let query = sql::Select::new()
        .left_join("addresses ON users.login = addresses.login")
        .as_string();
      let expected_query = "LEFT JOIN addresses ON users.login = addresses.login";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_left_join_should_accumulate_values_on_consecutive_calls() {
      let query = sql::Select::new()
        .left_join("addresses ON users.login = addresses.login")
        .left_join("orders ON users.login = orders.login")
        .as_string();
      let expected_query = "\
        LEFT JOIN addresses ON users.login = addresses.login \
        LEFT JOIN orders ON users.login = orders.login\
      ";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_left_join_should_not_accumulate_values_when_table_expression_is_empty() {
      let query = sql::Select::new()
        .left_join("")
        .left_join("orders ON users.login = orders.login")
        .left_join("")
        .as_string();
      let expected_query = "LEFT JOIN orders ON users.login = orders.login";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_left_join_by_should_trim_space_of_the_argument() {
      let query = sql::Select::new().left_join("  orders  ").as_string();
      let expected_query = "LEFT JOIN orders";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_left_join_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Select::new()
        .left_join("addresses")
        .left_join("addresses")
        .as_string();
      let expected_query = "LEFT JOIN addresses";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn clause_left_join_should_be_after_from_clause() {
      let query = sql::Select::new()
        .from("users")
        .left_join("addresses ON users.login = addresses.login")
        .as_string();
      let expected_query = "FROM users LEFT JOIN addresses ON users.login = addresses.login";

      assert_eq!(expected_query, query);
    }
  }

  #[cfg(feature = "sqlite")]
  mod update_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_left_join_should_add_the_left_join_clause() {
      let query = sql::Update::new()
        .left_join("addresses ON users.login = addresses.login")
        .as_string();
      let expected_query = "LEFT JOIN addresses ON users.login = addresses.login";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_left_join_should_accumulate_values_on_consecutive_calls() {
      let query = sql::Update::new()
        .left_join("addresses ON users.login = addresses.login")
        .left_join("orders ON users.login = orders.login")
        .as_string();
      let expected_query = "\
        LEFT JOIN addresses ON users.login = addresses.login \
        LEFT JOIN orders ON users.login = orders.login\
      ";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_left_join_should_not_accumulate_values_when_table_expression_is_empty() {
      let query = sql::Update::new()
        .left_join("")
        .left_join("orders ON users.login = orders.login")
        .left_join("")
        .as_string();
      let expected_query = "LEFT JOIN orders ON users.login = orders.login";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_left_join_by_should_trim_space_of_the_argument() {
      let query = sql::Update::new().left_join("  orders  ").as_string();
      let expected_query = "LEFT JOIN orders";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_left_join_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Update::new()
        .left_join("addresses")
        .left_join("addresses")
        .as_string();
      let expected_query = "LEFT JOIN addresses";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn clause_left_join_should_be_after_from_clause() {
      let query = sql::Update::new()
        .from("users")
        .left_join("addresses ON users.login = addresses.login")
        .as_string();
      let expected_query = "FROM users LEFT JOIN addresses ON users.login = addresses.login";

      assert_eq!(expected_query, query);
    }
  }
}

mod right_join_clause {
  #[cfg(feature = "mysql")]
  mod delete_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_right_join_should_add_the_right_join_clause() {
      let query = sql::Delete::new()
        .right_join("addresses ON users.login = addresses.login")
        .as_string();
      let expected_query = "RIGHT JOIN addresses ON users.login = addresses.login";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_right_join_should_accumulate_values_on_consecutive_calls() {
      let query = sql::Delete::new()
        .right_join("addresses ON users.login = addresses.login")
        .right_join("orders ON users.login = orders.login")
        .as_string();
      let expected_query = "\
        RIGHT JOIN addresses ON users.login = addresses.login \
        RIGHT JOIN orders ON users.login = orders.login\
      ";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_right_join_should_not_accumulate_values_when_table_expression_is_empty() {
      let query = sql::Delete::new()
        .right_join("")
        .right_join("orders ON users.login = orders.login")
        .right_join("")
        .as_string();
      let expected_query = "RIGHT JOIN orders ON users.login = orders.login";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_right_join_by_should_trim_space_of_the_argument() {
      let query = sql::Delete::new().right_join("  orders  ").as_string();
      let expected_query = "RIGHT JOIN orders";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_right_join_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Delete::new()
        .right_join("addresses")
        .right_join("addresses")
        .as_string();
      let expected_query = "RIGHT JOIN addresses";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn clause_right_join_should_be_after_from_clause() {
      let query = sql::Delete::new()
        .from("users")
        .right_join("addresses ON users.login = addresses.login")
        .as_string();
      let expected_query = "FROM users RIGHT JOIN addresses ON users.login = addresses.login";

      assert_eq!(expected_query, query);
    }
  }

  mod select_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_right_join_should_add_the_right_join_clause() {
      let query = sql::Select::new()
        .right_join("addresses ON users.login = addresses.login")
        .as_string();
      let expected_query = "RIGHT JOIN addresses ON users.login = addresses.login";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_right_join_should_accumulate_values_on_consecutive_calls() {
      let query = sql::Select::new()
        .right_join("addresses ON users.login = addresses.login")
        .right_join("orders ON users.login = orders.login")
        .as_string();
      let expected_query = "\
        RIGHT JOIN addresses ON users.login = addresses.login \
        RIGHT JOIN orders ON users.login = orders.login\
      ";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_right_join_should_not_accumulate_values_when_table_expression_is_empty() {
      let query = sql::Select::new()
        .right_join("")
        .right_join("orders ON users.login = orders.login")
        .right_join("")
        .as_string();
      let expected_query = "RIGHT JOIN orders ON users.login = orders.login";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_right_join_by_should_trim_space_of_the_argument() {
      let query = sql::Select::new().right_join("  orders  ").as_string();
      let expected_query = "RIGHT JOIN orders";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_right_join_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Select::new()
        .right_join("addresses")
        .right_join("addresses")
        .as_string();
      let expected_query = "RIGHT JOIN addresses";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn clause_right_join_should_be_after_from_clause() {
      let query = sql::Select::new()
        .from("users")
        .right_join("addresses ON users.login = addresses.login")
        .as_string();
      let expected_query = "FROM users RIGHT JOIN addresses ON users.login = addresses.login";

      assert_eq!(expected_query, query);
    }
  }

  #[cfg(feature = "sqlite")]
  mod update_command {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_right_join_should_add_the_right_join_clause() {
      let query = sql::Update::new()
        .right_join("addresses ON users.login = addresses.login")
        .as_string();
      let expected_query = "RIGHT JOIN addresses ON users.login = addresses.login";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_right_join_should_accumulate_values_on_consecutive_calls() {
      let query = sql::Update::new()
        .right_join("addresses ON users.login = addresses.login")
        .right_join("orders ON users.login = orders.login")
        .as_string();
      let expected_query = "\
        RIGHT JOIN addresses ON users.login = addresses.login \
        RIGHT JOIN orders ON users.login = orders.login\
      ";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_right_join_should_not_accumulate_values_when_table_expression_is_empty() {
      let query = sql::Update::new()
        .right_join("")
        .right_join("orders ON users.login = orders.login")
        .right_join("")
        .as_string();
      let expected_query = "RIGHT JOIN orders ON users.login = orders.login";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_right_join_by_should_trim_space_of_the_argument() {
      let query = sql::Update::new().right_join("  orders  ").as_string();
      let expected_query = "RIGHT JOIN orders";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn method_right_join_should_not_accumulate_arguments_with_the_same_content() {
      let query = sql::Update::new()
        .right_join("addresses")
        .right_join("addresses")
        .as_string();
      let expected_query = "RIGHT JOIN addresses";

      assert_eq!(expected_query, query);
    }

    #[test]
    fn clause_right_join_should_be_after_from_clause() {
      let query = sql::Update::new()
        .from("users")
        .right_join("addresses ON users.login = addresses.login")
        .as_string();
      let expected_query = "FROM users RIGHT JOIN addresses ON users.login = addresses.login";

      assert_eq!(expected_query, query);
    }
  }
}
