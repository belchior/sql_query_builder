mod builder_features {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn drop_table_builder_should_be_displayable() {
    let drop_table = sql::DropTable::new().drop_table("orders");

    println!("{}", drop_table);

    let query = drop_table.as_string();
    let expected_query = "DROP TABLE orders";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn drop_table_builder_should_be_debuggable() {
    let drop_table = sql::DropTable::new().drop_table("orders");

    println!("{:?}", drop_table);

    let expected_query = "DROP TABLE orders";
    let query = drop_table.as_string();

    assert_eq!(expected_query, query);
  }

  #[cfg(not(feature = "postgresql"))]
  #[test]
  fn drop_table_builder_should_be_able_to_conditionally_add_clauses() {
    let mut drop_table = sql::DropTable::new().drop_table("orders");

    if true {
      drop_table = drop_table.drop_table("users");
    }

    let query = drop_table.as_string();
    let expected_query = "DROP TABLE users";

    assert_eq!(expected_query, query);
  }

  #[cfg(feature = "postgresql")]
  #[test]
  fn drop_table_builder_should_be_able_to_conditionally_add_clauses() {
    let mut drop_table = sql::DropTable::new().drop_table("orders");

    if true {
      drop_table = drop_table.drop_table("users");
    }

    let query = drop_table.as_string();
    let expected_query = "DROP TABLE orders, users";

    assert_eq!(expected_query, query);
  }

  #[cfg(not(feature = "postgresql"))]
  #[test]
  fn drop_table_builder_should_be_cloneable() {
    let drop_users = sql::DropTable::new().drop_table("users");
    let drop_users_and_orders = drop_users.clone().drop_table("orders");

    let expected_drop_users = "DROP TABLE users";
    let expected_drop_users_and_orders = "DROP TABLE orders";

    assert_eq!(expected_drop_users, drop_users.as_string());
    assert_eq!(expected_drop_users_and_orders, drop_users_and_orders.as_string());
  }

  #[cfg(feature = "postgresql")]
  #[test]
  fn drop_table_builder_should_be_cloneable() {
    let drop_users = sql::DropTable::new().drop_table("users");
    let drop_users_and_orders = drop_users.clone().drop_table("orders");

    let expected_drop_users = "DROP TABLE users";
    let expected_drop_users_and_orders = "DROP TABLE users, orders";

    assert_eq!(expected_drop_users, drop_users.as_string());
    assert_eq!(expected_drop_users_and_orders, drop_users_and_orders.as_string());
  }

  #[test]
  fn drop_table_builder_should_be_composable() {
    fn add_comment(select: sql::DropTable) -> sql::DropTable {
      select.raw("/* drop command */")
    }

    fn drop_orders(select: sql::DropTable) -> sql::DropTable {
      select.drop_table("orders")
    }

    fn as_string(select: sql::DropTable) -> String {
      select.as_string()
    }

    let query = Some(sql::DropTable::new())
      .map(add_comment)
      .map(drop_orders)
      .map(as_string)
      .unwrap();

    let expected_query = "/* drop command */ DROP TABLE orders";

    assert_eq!(expected_query, query);
  }
}

mod builder_methods {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_new_should_initialize_as_empty_string() {
    let query = sql::DropTable::new().as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_as_string_should_convert_the_current_state_into_string() {
    let query = sql::DropTable::new().as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_debug_should_print_at_console_in_a_human_readable_format() {
    let query = sql::DropTable::new().drop_table_if_exists("users").debug().as_string();

    let expected_query = "DROP TABLE IF EXISTS users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_print_should_print_in_one_line_the_current_state_of_builder() {
    let query = sql::DropTable::new().drop_table_if_exists("users").print().as_string();

    let expected_query = "DROP TABLE IF EXISTS users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_add_raw_sql() {
    let query = sql::DropTable::new().raw("drop table users cascade").as_string();

    let expected_query = "drop table users cascade";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_accumulate_values_on_consecutive_calls() {
    let query = sql::DropTable::new().raw("drop table users").raw("cascade").as_string();

    let expected_query = "drop table users cascade";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_be_the_first_to_be_concatenated() {
    let query = sql::DropTable::new()
      .raw("/* drop table command */")
      .drop_table("users")
      .as_string();

    let expected_query = "/* drop table command */ DROP TABLE users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::DropTable::new()
      .raw("drop table users")
      .raw("drop table users")
      .as_string();

    let expected_query = "drop table users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_trim_space_of_the_argument() {
    let query = sql::DropTable::new()
      .drop_table("users")
      .raw_after(sql::DropTableParams::DropTable, "   CASCADE   ")
      .as_string();
    let expected_query = "DROP TABLE users CASCADE";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_trim_space_of_the_argument() {
    let query = sql::DropTable::new()
      .raw_before(sql::DropTableParams::DropTable, "  /* drop table command */  ")
      .drop_table("users")
      .as_string();
    let expected_query = "/* drop table command */ DROP TABLE users";

    assert_eq!(expected_query, query);
  }
}

mod method_drop_table {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_drop_table_should_add_the_drop_table_signature() {
    let query = sql::DropTable::new().drop_table("films").as_string();
    let expected_query = "DROP TABLE films";

    assert_eq!(expected_query, query);
  }

  #[cfg(not(feature = "postgresql"))]
  #[test]
  fn method_drop_table_should_overrides_previous_value_on_consecutive_calls() {
    let query = sql::DropTable::new()
      .drop_table("films")
      .drop_table("series")
      .as_string();

    let expected_query = "DROP TABLE series";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_drop_table_should_trim_space_of_the_argument() {
    let query = sql::DropTable::new().drop_table("   films   ").as_string();
    let expected_query = "DROP TABLE films";

    assert_eq!(expected_query, query);
  }

  #[cfg(not(feature = "postgresql"))]
  #[test]
  fn method_drop_table_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::DropTable::new()
      .drop_table("films")
      .drop_table("films")
      .as_string();
    let expected_query = "DROP TABLE films";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_method_drop_table() {
    let query = sql::DropTable::new()
      .raw_before(sql::DropTableParams::DropTable, "/* drop command */")
      .drop_table("films")
      .as_string();
    let expected_query = "/* drop command */ DROP TABLE films";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_method_drop_table() {
    let query = sql::DropTable::new()
      .drop_table("films")
      .raw_after(sql::DropTableParams::DropTable, "RESTRICT")
      .as_string();
    let expected_query = "DROP TABLE films RESTRICT";

    assert_eq!(expected_query, query);
  }
}

mod method_drop_table_if_exists {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_drop_table_if_exists_should_add_the_drop_table_signature() {
    let query = sql::DropTable::new().drop_table_if_exists("films").as_string();
    let expected_query = "DROP TABLE IF EXISTS films";

    assert_eq!(expected_query, query);
  }

  #[cfg(not(feature = "postgresql"))]
  #[test]
  fn method_drop_table_if_exists_should_overrides_previous_value_on_consecutive_calls() {
    let query = sql::DropTable::new()
      .drop_table_if_exists("films")
      .drop_table_if_exists("series")
      .as_string();

    let expected_query = "DROP TABLE IF EXISTS series";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_drop_table_if_exists_should_trim_space_of_the_argument() {
    let query = sql::DropTable::new().drop_table_if_exists("   films   ").as_string();
    let expected_query = "DROP TABLE IF EXISTS films";

    assert_eq!(expected_query, query);
  }

  #[cfg(not(feature = "postgresql"))]
  #[test]
  fn method_drop_table_if_exists_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::DropTable::new()
      .drop_table_if_exists("films")
      .drop_table_if_exists("films")
      .as_string();
    let expected_query = "DROP TABLE IF EXISTS films";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_method_drop_table_if_exists() {
    let query = sql::DropTable::new()
      .raw_before(sql::DropTableParams::DropTable, "/* drop command */")
      .drop_table_if_exists("films")
      .as_string();
    let expected_query = "/* drop command */ DROP TABLE IF EXISTS films";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_method_drop_table_if_exists() {
    let query = sql::DropTable::new()
      .drop_table_if_exists("films")
      .raw_after(sql::DropTableParams::DropTable, "RESTRICT")
      .as_string();
    let expected_query = "DROP TABLE IF EXISTS films RESTRICT";

    assert_eq!(expected_query, query);
  }
}

#[cfg(feature = "postgresql")]
mod postgres_feature_flag {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_drop_table_should_accumulate_values_on_consecutive_calls() {
    let query = sql::DropTable::new()
      .drop_table("films")
      .drop_table("series")
      .as_string();

    let expected_query = "DROP TABLE films, series";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_drop_table_if_exists_should_accumulate_values_on_consecutive_calls() {
    let query = sql::DropTable::new()
      .drop_table_if_exists("films")
      .drop_table_if_exists("series")
      .as_string();

    let expected_query = "DROP TABLE IF EXISTS films, series";

    assert_eq!(expected_query, query);
  }
}
