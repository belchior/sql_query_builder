mod builder_features {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn select_builder_should_be_displayable() {
    let select = sql::Select::new().select("id, login").from("users");

    println!("{}", select);

    let query = select.as_string();
    let expected_query = "SELECT id, login FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn select_builder_should_be_debuggable() {
    let select = sql::Select::new().select("*").from("orders").where_clause("id = $1");

    println!("{:?}", select);

    let expected_query = "SELECT * FROM orders WHERE id = $1";
    let query = select.as_string();

    assert_eq!(query, expected_query);
  }

  #[test]
  fn select_builder_should_be_cloneable() {
    let select_zipcode = sql::Select::new()
      .raw("/* test raw */")
      .select("zipcode")
      .from("addresses")
      .raw_before(sql::SelectClause::Where, "/* test raw_before */")
      .where_clause("login = $1")
      .raw_after(sql::SelectClause::Where, "/* test raw_after */");

    let select_city = select_zipcode.clone().select("city");

    let query_zipcode = select_zipcode.as_string();
    let query_city = select_city.as_string();

    let expected_query_zipcode = "\
      /* test raw */ \
      SELECT zipcode \
      FROM addresses \
      /* test raw_before */ \
      WHERE login = $1 \
      /* test raw_after */\
    ";
    let expected_query_city = "\
      /* test raw */ \
      SELECT zipcode, city \
      FROM addresses \
      /* test raw_before */ \
      WHERE login = $1 \
      /* test raw_after */\
    ";

    assert_eq!(query_zipcode, expected_query_zipcode);
    assert_eq!(query_city, expected_query_city);
  }

  #[test]
  fn select_builder_should_be_able_to_conditionally_add_clauses() {
    let mut select = sql::Select::new().select("zipcode").from("addresses");

    if true {
      select = select.where_clause("login = $1");
    }

    let query = select.as_string();
    let expected_query = "SELECT zipcode FROM addresses WHERE login = $1";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn select_builder_should_be_composable() {
    fn project(select: sql::Select) -> sql::Select {
      select
        .select("u.id, u.name as user_name, u.login")
        .select("a.name as addresses_name")
        .select("o.name as product_name")
    }

    fn joins(select: sql::Select) -> sql::Select {
      select
        .from("users u")
        .inner_join("addresses a ON a.user_login = u.login")
        .inner_join("orders o ON o.user_login = u.login")
    }

    fn conditions(select: sql::Select) -> sql::Select {
      select.where_clause("u.login = $1").where_clause("o.id = $2")
    }

    fn as_string(select: sql::Select) -> String {
      select.as_string()
    }

    let query = Some(sql::Select::new())
      .map(project)
      .map(joins)
      .map(conditions)
      .map(as_string)
      .unwrap();

    let expected_query = "\
      SELECT u.id, u.name as user_name, u.login, a.name as addresses_name, o.name as product_name \
      FROM users u \
      INNER JOIN addresses a ON a.user_login = u.login \
      INNER JOIN orders o ON o.user_login = u.login \
      WHERE u.login = $1 AND o.id = $2\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn all_standard_clauses_concatenated_in_order() {
    let query = sql::Select::new()
      .raw("/* all clauses in order */")
      .select("*")
      .from("users")
      .inner_join("orders ON users.login = orders.login")
      .where_clause("user.login = $1")
      .group_by("login")
      .having("active = true")
      .order_by("created_at desc")
      .as_string();

    let expected_query = "\
      /* all clauses in order */ \
      SELECT * \
      FROM users \
      INNER JOIN orders ON users.login = orders.login \
      WHERE user.login = $1 \
      GROUP BY login \
      HAVING active = true \
      ORDER BY created_at desc\
    ";

    assert_eq!(query, expected_query);
  }
}

mod builder_methods {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_new_should_initialize_as_empty_string() {
    let query = sql::Select::new().as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_as_string_should_convert_the_current_state_into_string() {
    let query = sql::Select::new().as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_debug_should_print_at_console_in_a_human_readable_format() {
    let query = sql::Select::new()
      .raw("/* all clauses in order */")
      .select("id as user_id")
      .from("user_list")
      .inner_join("orders ON users.login = orders.login")
      .where_clause("user.created_at::date >= $1")
      .where_clause("user.login not in ($2)")
      .group_by("login")
      .having("active = true")
      .order_by("login asc, created_at desc")
      .debug()
      .as_string();

    let expected_query = "\
      /* all clauses in order */ \
      SELECT id as user_id \
      FROM user_list \
      INNER JOIN orders ON users.login = orders.login \
      WHERE user.created_at::date >= $1 \
      AND user.login not in ($2) \
      GROUP BY login \
      HAVING active = true \
      ORDER BY login asc, created_at desc\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_print_should_print_in_one_line_the_current_state_of_builder() {
    let query = sql::Select::new().select("1 + 2").print().as_string();
    let expected_query = "SELECT 1 + 2";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_add_raw_sql() {
    let query = sql::Select::new().raw("select id from users").as_string();
    let expected_query = "select id from users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Select::new().raw("select id").raw("from users").as_string();
    let expected_query = "select id from users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_be_the_first_to_be_concatenated() {
    let query = sql::Select::new().raw("select *").from("users").as_string();
    let expected_query = "select * FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_trim_space_of_the_argument() {
    let query = sql::Select::new().raw("  update users  ").as_string();
    let expected_query = "update users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Select::new()
      .raw("select login, name")
      .raw("select login, name")
      .as_string();
    let expected_query = "select login, name";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_trim_space_of_the_argument() {
    let query = sql::Select::new()
      .raw_after(sql::SelectClause::Select, "  from orders  ")
      .as_string();
    let expected_query = "from orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_trim_space_of_the_argument() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::Where, "  from addresses  ")
      .as_string();
    let expected_query = "from addresses";

    assert_eq!(query, expected_query);
  }
}
