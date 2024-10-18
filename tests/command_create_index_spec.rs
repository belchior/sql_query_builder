#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod full_api {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[cfg(feature = "postgresql")]
  #[test]
  fn postgres_with_all_methods() {
    let query = sql::CreateIndex::new()
      // at least one of methods
      .create_index("users_name_idx")
      .create_index_if_not_exists("users_name_idx")
      .unique()
      .concurrently()
      // required methods
      .on("users")
      .column("name")
      // optional methods
      .only()
      .using("btree")
      .include("last_name")
      .where_clause("created_at >= $1")
      .where_and("created_at < $2")
      .where_or("status = 'active'")
      .as_string();

    let expected_query = "\
      CREATE UNIQUE INDEX \
      CONCURRENTLY \
      IF NOT EXISTS users_name_idx \
      ON ONLY users \
      USING btree \
      (name) \
      INCLUDE (last_name) \
      WHERE \
        created_at >= $1 \
        AND created_at < $2 \
        OR status = 'active'\
    ";

    assert_eq!(expected_query, query);
  }

  #[cfg(feature = "sqlite")]
  #[test]
  fn sqlite_with_all_methods() {
    let query = sql::CreateIndex::new()
      // at least one of methods
      .create_index("users_name_idx")
      .create_index_if_not_exists("users_name_idx")
      // required methods
      .on("users")
      .column("name")
      // optional methods
      .unique()
      .where_clause("created_at >= $1")
      .where_and("created_at < $2")
      .where_or("status = 'active'")
      .as_string();

    let expected_query = "\
      CREATE UNIQUE INDEX \
      IF NOT EXISTS users_name_idx \
      ON users \
      (name) \
      WHERE \
        created_at >= $1 \
        AND created_at < $2 \
        OR status = 'active'\
    ";

    assert_eq!(expected_query, query);
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod builder_features {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn create_index_builder_should_be_displayable() {
    let create_index = sql::CreateIndex::new().create_index("orders_product_name_idx");

    println!("{create_index}");

    let query = create_index.as_string();
    let expected_query = "CREATE INDEX orders_product_name_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn create_index_builder_should_be_debuggable() {
    let create_index = sql::CreateIndex::new().create_index("orders_product_name_idx");

    println!("{create_index:?}");

    let expected_query = "CREATE INDEX orders_product_name_idx";
    let query = create_index.as_string();

    assert_eq!(expected_query, query);
  }

  #[test]
  fn create_index_builder_should_be_cloneable() {
    let login_index = sql::CreateIndex::new().create_index("users_login_idx");

    let product_name_index = login_index
      .clone()
      .create_index_if_not_exists("orders_product_name_idx");

    let expected_login_index = "CREATE INDEX users_login_idx";
    let expected_product_name_index = "CREATE INDEX IF NOT EXISTS orders_product_name_idx";

    assert_eq!(expected_login_index, login_index.as_string());
    assert_eq!(expected_product_name_index, product_name_index.as_string());
  }

  #[test]
  fn create_index_builder_should_be_able_to_conditionally_add_clauses() {
    let mut create_index = sql::CreateIndex::new().create_index("orders_product_name_idx");

    if true {
      create_index = create_index.create_index_if_not_exists("users_login_idx");
    }

    let query = create_index.as_string();
    let expected_query = "CREATE INDEX IF NOT EXISTS users_login_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn create_index_builder_should_be_composable() {
    fn create_index(select: sql::CreateIndex) -> sql::CreateIndex {
      select.create_index("orders_product_name_idx")
    }

    fn create_index_if_not_exists(select: sql::CreateIndex) -> sql::CreateIndex {
      select.create_index_if_not_exists("users_login_idx")
    }

    fn as_string(select: sql::CreateIndex) -> String {
      select.as_string()
    }

    let query = Some(sql::CreateIndex::new())
      .map(create_index)
      .map(create_index_if_not_exists)
      .map(as_string)
      .unwrap();

    let expected_query = "CREATE INDEX IF NOT EXISTS users_login_idx";

    assert_eq!(expected_query, query);
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod builder_methods {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_new_should_initialize_as_empty_string() {
    let query = sql::CreateIndex::new().as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_as_string_should_convert_the_current_state_into_string() {
    let query = sql::CreateIndex::new().as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_debug_should_print_at_console_in_a_human_readable_format() {
    let query = sql::CreateIndex::new()
      .create_index_if_not_exists("users_login_idx")
      .on("users")
      .column("login")
      .debug()
      .as_string();

    let expected_query = "CREATE INDEX IF NOT EXISTS users_login_idx ON users (login)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_print_should_print_in_one_line_the_current_state_of_builder() {
    let query = sql::CreateIndex::new()
      .create_index_if_not_exists("users_login_idx")
      .on("users")
      .column("login")
      .print()
      .as_string();

    let expected_query = "CREATE INDEX IF NOT EXISTS users_login_idx ON users (login)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_add_raw_sql() {
    let query = sql::CreateIndex::new().raw("create index on users (name)").as_string();

    let expected_query = "create index on users (name)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_accumulate_values_on_consecutive_calls() {
    let query = sql::CreateIndex::new()
      .raw("create index")
      .raw("on users")
      .raw("(name)")
      .as_string();

    let expected_query = "create index on users (name)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::CreateIndex::new()
      .raw("")
      .raw("on users")
      .raw("(name)")
      .raw("")
      .as_string();

    let expected_query = "on users (name)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_be_the_first_to_be_concatenated() {
    let query = sql::CreateIndex::new()
      .raw("/* create index command */")
      .create_index("users_login_idx")
      .as_string();

    let expected_query = "/* create index command */ CREATE INDEX users_login_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::CreateIndex::new()
      .raw("create index")
      .raw("create index")
      .as_string();

    let expected_query = "create index";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_trim_space_of_the_argument() {
    let query = sql::CreateIndex::new()
      .raw_after(sql::CreateIndexParams::CreateIndex, "  /* command */  ")
      .as_string();
    let expected_query = "/* command */";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_trim_space_of_the_argument() {
    let query = sql::CreateIndex::new()
      .raw_before(sql::CreateIndexParams::CreateIndex, "  on users (name)  ")
      .as_string();
    let expected_query = "on users (name)";

    assert_eq!(expected_query, query);
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod method_column {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_column_should_define_the_column_of_the_table() {
    let query = sql::CreateIndex::new().column("login").as_string();
    let expected_query = "(login)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_column_should_not_add_the_column_part_when_has_no_column_name_at_list() {
    let query = sql::CreateIndex::new().column("").column("").as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_column_should_not_add_column_when_the_column_name_is_empty() {
    let query = sql::CreateIndex::new()
      .column("")
      .column("login")
      .column("")
      .as_string();
    let expected_query = "(login)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_column_should_accumulate_column_names_on_consecutive_calls() {
    let query = sql::CreateIndex::new().column("login").column("name").as_string();

    let expected_query = "(login, name)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_column_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::CreateIndex::new()
      .column("")
      .column("login")
      .column("")
      .as_string();

    let expected_query = "(login)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_column_should_not_accumulate_parameters_with_the_same_content() {
    let query = sql::CreateIndex::new().column("login").column("login").as_string();
    let expected_query = "(login)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_column_should_trim_space_of_the_argument() {
    let query = sql::CreateIndex::new().column("  login  ").as_string();
    let expected_query = "(login)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_column_parameter() {
    let query = sql::CreateIndex::new()
      .column("name")
      .raw_after(sql::CreateIndexParams::Column, "/* end command */")
      .as_string();

    let expected_query = "(name) /* end command */";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_column_parameter() {
    let query = sql::CreateIndex::new()
      .raw_before(sql::CreateIndexParams::Column, "on users")
      .column("name")
      .as_string();

    let expected_query = "on users (name)";

    assert_eq!(expected_query, query);
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod method_create_index {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_create_index_should_define_a_create_index_parameter() {
    let query = sql::CreateIndex::new().create_index("users_login_idx").as_string();
    let expected_query = "CREATE INDEX users_login_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_create_index_should_overrides_the_current_value_on_consecutive_calls() {
    let query = sql::CreateIndex::new()
      .create_index("users_login_idx")
      .create_index("orders_product_name_idx")
      .as_string();

    let expected_query = "CREATE INDEX orders_product_name_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_create_index_should_not_accumulate_parameters_with_the_same_content() {
    let query = sql::CreateIndex::new()
      .create_index("orders_product_name_idx")
      .create_index("orders_product_name_idx")
      .as_string();
    let expected_query = "CREATE INDEX orders_product_name_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_create_index_should_trim_space_of_the_argument() {
    let query = sql::CreateIndex::new().create_index("  users_login_idx  ").as_string();
    let expected_query = "CREATE INDEX users_login_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_create_index_parameter() {
    let query = sql::CreateIndex::new()
      .create_index("users_login_idx")
      .raw_after(sql::CreateIndexParams::CreateIndex, "/* end command */")
      .as_string();

    let expected_query = "CREATE INDEX users_login_idx /* end command */";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_create_index_parameter() {
    let query = sql::CreateIndex::new()
      .raw_before(sql::CreateIndexParams::CreateIndex, "/* start command */")
      .create_index("users_name_idx")
      .as_string();

    let expected_query = "/* start command */ CREATE INDEX users_name_idx";

    assert_eq!(expected_query, query);
  }

  #[cfg(feature = "postgresql")]
  #[test]
  fn method_create_index_should_define_the_parameter_without_specify_the_name_of_the_index() {
    let query = sql::CreateIndex::new().create_index("").as_string();
    let expected_query = "CREATE INDEX";

    assert_eq!(expected_query, query);
  }

  #[cfg(feature = "sqlite")]
  #[test]
  fn method_create_index_should_define_the_parameter_only_with_name_of_the_index() {
    let query = sql::CreateIndex::new().create_index("").as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod method_create_index_if_not_exists {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_create_index_if_not_exists_should_define_a_create_index_parameter() {
    let query = sql::CreateIndex::new()
      .create_index_if_not_exists("users_login_idx")
      .as_string();
    let expected_query = "CREATE INDEX IF NOT EXISTS users_login_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_create_index_if_not_exists_should_overrides_the_current_value_on_consecutive_calls() {
    let query = sql::CreateIndex::new()
      .create_index("users_name_idx")
      .create_index_if_not_exists("users_login_idx")
      .create_index_if_not_exists("orders_product_name_idx")
      .as_string();

    let expected_query = "CREATE INDEX IF NOT EXISTS orders_product_name_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_create_index_if_not_exists_should_not_accumulate_parameters_with_the_same_content() {
    let query = sql::CreateIndex::new()
      .create_index_if_not_exists("orders_product_name_idx")
      .create_index_if_not_exists("orders_product_name_idx")
      .as_string();
    let expected_query = "CREATE INDEX IF NOT EXISTS orders_product_name_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_create_index_if_not_exists_should_trim_space_of_the_argument() {
    let query = sql::CreateIndex::new()
      .create_index_if_not_exists("  users_login_idx  ")
      .as_string();
    let expected_query = "CREATE INDEX IF NOT EXISTS users_login_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_create_index_if_not_exists_parameter() {
    let query = sql::CreateIndex::new()
      .create_index_if_not_exists("users_login_idx")
      .raw_after(sql::CreateIndexParams::CreateIndex, "/* end command */")
      .as_string();

    let expected_query = "CREATE INDEX IF NOT EXISTS users_login_idx /* end command */";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_create_index_if_not_exists_parameter() {
    let query = sql::CreateIndex::new()
      .raw_before(sql::CreateIndexParams::CreateIndex, "/* start command */")
      .create_index_if_not_exists("users_name_idx")
      .as_string();

    let expected_query = "/* start command */ CREATE INDEX IF NOT EXISTS users_name_idx";

    assert_eq!(expected_query, query);
  }

  #[cfg(feature = "postgresql")]
  #[test]
  fn method_create_index_if_not_exists_should_define_the_parameter_only_with_the_name_of_the_index_postgres() {
    let query = sql::CreateIndex::new().create_index_if_not_exists("").as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[cfg(feature = "sqlite")]
  #[test]
  fn method_create_index_if_not_exists_should_define_the_parameter_only_with_the_name_of_the_index_sqlite() {
    let query = sql::CreateIndex::new().create_index_if_not_exists("").as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod method_on {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_on_should_define_a_on_parameter() {
    let query = sql::CreateIndex::new().on("users").as_string();
    let expected_query = "ON users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_on_should_overrides_the_current_value_on_consecutive_calls() {
    let query = sql::CreateIndex::new().on("users").on("orders").as_string();

    let expected_query = "ON orders";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_on_should_not_accumulate_parameters_with_the_same_content() {
    let query = sql::CreateIndex::new().on("users").on("users").as_string();
    let expected_query = "ON users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_on_should_trim_space_of_the_argument() {
    let query = sql::CreateIndex::new().on("  users  ").as_string();
    let expected_query = "ON users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_on_parameter() {
    let query = sql::CreateIndex::new()
      .on("users")
      .raw_after(sql::CreateIndexParams::On, "(login)")
      .as_string();

    let expected_query = "ON users (login)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_on_parameter() {
    let query = sql::CreateIndex::new()
      .raw_before(sql::CreateIndexParams::On, "create index users_name_idx")
      .on("users")
      .as_string();

    let expected_query = "create index users_name_idx ON users";

    assert_eq!(expected_query, query);
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod method_unique {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_unique_should_define_a_create_index_parameter_with_the_modifier_unique() {
    let query = sql::CreateIndex::new()
      .create_index("users_login_idx")
      .unique()
      .as_string();
    let expected_query = "CREATE UNIQUE INDEX users_login_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_unique_should_define_a_create_index_if_not_exists_parameter_with_the_modifier_unique() {
    let query = sql::CreateIndex::new()
      .create_index_if_not_exists("users_login_idx")
      .unique()
      .as_string();
    let expected_query = "CREATE UNIQUE INDEX IF NOT EXISTS users_login_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_unique_should_have_any_effect_in_the_current_state_on_consecutive_calls() {
    let query = sql::CreateIndex::new()
      .create_index("users_login_idx")
      .unique()
      .unique()
      .as_string();

    let expected_query = "CREATE UNIQUE INDEX users_login_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_unique_parameter() {
    let query = sql::CreateIndex::new()
      .create_index("users_name_idx")
      .unique()
      .raw_after(sql::CreateIndexParams::Unique, "/* uncommon parameter */")
      .as_string();

    let expected_query = "CREATE UNIQUE /* uncommon parameter */ INDEX users_name_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_not_add_raw_sql_when_the_method_unique_was_not_called() {
    let query = sql::CreateIndex::new()
      .create_index("users_name_idx")
      .raw_after(sql::CreateIndexParams::Unique, "/* uncommon parameter */")
      .as_string();

    let expected_query = "CREATE INDEX users_name_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_unique_parameter() {
    let query = sql::CreateIndex::new()
      .create_index("users_name_idx")
      .raw_before(sql::CreateIndexParams::Unique, "/* uncommon parameter */")
      .unique()
      .as_string();

    let expected_query = "CREATE /* uncommon parameter */ UNIQUE INDEX users_name_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_not_add_raw_sql_when_the_method_unique_was_not_called() {
    let query = sql::CreateIndex::new()
      .create_index("users_name_idx")
      .raw_before(sql::CreateIndexParams::Unique, "/* uncommon parameter */")
      .as_string();

    let expected_query = "CREATE INDEX users_name_idx";

    assert_eq!(expected_query, query);
  }

  #[cfg(feature = "postgresql")]
  #[test]
  fn method_unique_should_define_a_create_index_parameter_even_when_the_method_create_index_was_not_called() {
    let query = sql::CreateIndex::new().unique().as_string();
    let expected_query = "CREATE UNIQUE INDEX";

    assert_eq!(expected_query, query);
  }

  #[cfg(feature = "sqlite")]
  #[test]
  fn method_unique_should_not_define_a_create_index_parameter_when_the_method_create_index_was_not_called() {
    let query = sql::CreateIndex::new().unique().as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }
}

#[cfg(feature = "postgresql")]
mod method_concurrently {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_concurrently_should_define_a_create_index_parameter_with_the_modifier_concurrently() {
    let query = sql::CreateIndex::new()
      .create_index("users_login_idx")
      .concurrently()
      .as_string();
    let expected_query = "CREATE INDEX CONCURRENTLY users_login_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_concurrently_should_define_a_create_index_if_not_exists_parameter_with_the_modifier_concurrently() {
    let query = sql::CreateIndex::new()
      .create_index_if_not_exists("users_login_idx")
      .concurrently()
      .as_string();
    let expected_query = "CREATE INDEX CONCURRENTLY IF NOT EXISTS users_login_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_concurrently_should_have_any_effect_in_the_current_state_on_consecutive_calls() {
    let query = sql::CreateIndex::new()
      .create_index("users_login_idx")
      .concurrently()
      .concurrently()
      .as_string();

    let expected_query = "CREATE INDEX CONCURRENTLY users_login_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_concurrently_parameter() {
    let query = sql::CreateIndex::new()
      .create_index("users_name_idx")
      .concurrently()
      .raw_after(sql::CreateIndexParams::Concurrently, "/* uncommon parameter */")
      .as_string();

    let expected_query = "CREATE INDEX CONCURRENTLY /* uncommon parameter */ users_name_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_not_add_raw_sql_when_the_method_concurrently_was_not_called() {
    let query = sql::CreateIndex::new()
      .create_index("users_name_idx")
      .raw_after(sql::CreateIndexParams::Concurrently, "/* uncommon parameter */")
      .as_string();

    let expected_query = "CREATE INDEX users_name_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_concurrently_parameter() {
    let query = sql::CreateIndex::new()
      .create_index("users_name_idx")
      .raw_before(sql::CreateIndexParams::Concurrently, "/* uncommon parameter */")
      .concurrently()
      .as_string();

    let expected_query = "CREATE INDEX /* uncommon parameter */ CONCURRENTLY users_name_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_not_add_raw_sql_when_the_method_concurrently_was_not_called() {
    let query = sql::CreateIndex::new()
      .create_index("users_name_idx")
      .raw_before(sql::CreateIndexParams::Concurrently, "/* uncommon parameter */")
      .as_string();

    let expected_query = "CREATE INDEX users_name_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_concurrently_should_define_a_create_index_parameter_even_when_the_method_create_index_was_not_called() {
    let query = sql::CreateIndex::new().concurrently().as_string();
    let expected_query = "CREATE INDEX CONCURRENTLY";

    assert_eq!(expected_query, query);
  }
}

#[cfg(feature = "postgresql")]
mod method_include {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_include_should_define_the_include_parameter() {
    let query = sql::CreateIndex::new().include("login").as_string();
    let expected_query = "INCLUDE (login)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_include_should_not_add_column_when_the_column_name_is_empty() {
    let query = sql::CreateIndex::new()
      .include("")
      .include("login")
      .include("")
      .as_string();
    let expected_query = "INCLUDE (login)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_include_should_not_defined_the_paramenter_without_column_names() {
    let query = sql::CreateIndex::new().include("").as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_include_should_accumulate_columns_on_consecutive_calls() {
    let query = sql::CreateIndex::new().include("login").include("name").as_string();

    let expected_query = "INCLUDE (login, name)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_include_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::CreateIndex::new()
      .include("")
      .include("login")
      .include("")
      .as_string();

    let expected_query = "INCLUDE (login)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_include_should_not_accumulate_columns_with_the_same_content() {
    let query = sql::CreateIndex::new().include("login").include("login").as_string();
    let expected_query = "INCLUDE (login)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_include_should_trim_space_of_the_argument() {
    let query = sql::CreateIndex::new().include("  login  ").as_string();
    let expected_query = "INCLUDE (login)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_include_parameter() {
    let query = sql::CreateIndex::new()
      .include("name")
      .raw_after(sql::CreateIndexParams::Include, "/* uncommon parameter */")
      .as_string();

    let expected_query = "INCLUDE (name) /* uncommon parameter */";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_include_parameter() {
    let query = sql::CreateIndex::new()
      .raw_before(sql::CreateIndexParams::Include, "/* uncommon parameter */")
      .include("name")
      .as_string();

    let expected_query = "/* uncommon parameter */ INCLUDE (name)";

    assert_eq!(expected_query, query);
  }
}

#[cfg(feature = "postgresql")]
mod method_only {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_only_should_define_a_on_parameter_with_the_modifier_only() {
    let query = sql::CreateIndex::new().on("users").only().as_string();
    let expected_query = "ON ONLY users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_only_should_have_any_effect_in_the_current_state_on_consecutive_calls() {
    let query = sql::CreateIndex::new().on("users").only().only().as_string();

    let expected_query = "ON ONLY users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_only_should_be_defined_in_presence_of_the_method_on() {
    let query = sql::CreateIndex::new().only().as_string();

    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_only_parameter() {
    let query = sql::CreateIndex::new()
      .on("users")
      .only()
      .raw_after(sql::CreateIndexParams::Only, "/* uncommon parameter */")
      .as_string();

    let expected_query = "ON ONLY /* uncommon parameter */ users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_not_add_raw_sql_when_the_method_only_was_not_called() {
    let query = sql::CreateIndex::new()
      .on("users")
      .raw_after(sql::CreateIndexParams::Only, "/* uncommon parameter */")
      .as_string();

    let expected_query = "ON users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_only_parameter() {
    let query = sql::CreateIndex::new()
      .on("users")
      .raw_before(sql::CreateIndexParams::Only, "/* uncommon parameter */")
      .only()
      .as_string();

    let expected_query = "ON /* uncommon parameter */ ONLY users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_not_add_raw_sql_when_the_method_only_was_not_called() {
    let query = sql::CreateIndex::new()
      .on("users")
      .raw_before(sql::CreateIndexParams::Only, "/* uncommon parameter */")
      .as_string();

    let expected_query = "ON users";

    assert_eq!(expected_query, query);
  }
}

#[cfg(feature = "postgresql")]
mod method_using {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_using_should_define_a_using_parameter() {
    let query = sql::CreateIndex::new().using("btree").as_string();
    let expected_query = "USING btree";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_using_should_overrides_the_current_value_on_consecutive_calls() {
    let query = sql::CreateIndex::new().using("btree").using("gist").as_string();
    let expected_query = "USING gist";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_using_should_not_accumulate_parameters_with_the_same_content() {
    let query = sql::CreateIndex::new().using("gist").using("gist").as_string();
    let expected_query = "USING gist";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_using_should_trim_space_of_the_argument() {
    let query = sql::CreateIndex::new().using("  btree  ").as_string();
    let expected_query = "USING btree";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_using_parameter() {
    let query = sql::CreateIndex::new()
      .using("btree")
      .raw_after(sql::CreateIndexParams::Using, "/* uncommon parameter */")
      .as_string();

    let expected_query = "USING btree /* uncommon parameter */";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_using_parameter() {
    let query = sql::CreateIndex::new()
      .raw_before(sql::CreateIndexParams::Using, "/* uncommon parameter */")
      .using("gist")
      .as_string();

    let expected_query = "/* uncommon parameter */ USING gist";

    assert_eq!(expected_query, query);
  }
}
