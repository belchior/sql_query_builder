#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod builder_features {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn drop_index_builder_should_be_displayable() {
    let drop_index = sql::DropIndex::new().drop_index("users_name_idx");

    println!("{}", drop_index);

    let query = drop_index.as_string();
    let expected_query = "DROP INDEX users_name_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn drop_index_builder_should_be_debuggable() {
    let drop_index = sql::DropIndex::new().drop_index("users_name_idx");

    println!("{:?}", drop_index);

    let expected_query = "DROP INDEX users_name_idx";
    let query = drop_index.as_string();

    assert_eq!(expected_query, query);
  }

  #[cfg(not(feature = "postgresql"))]
  #[test]
  fn drop_index_builder_should_be_able_to_conditionally_add_clauses() {
    let mut drop_index = sql::DropIndex::new().drop_index("users_name_idx");

    if true {
      drop_index = drop_index.drop_index("users_name_idx");
    }

    let query = drop_index.as_string();
    let expected_query = "DROP INDEX users_name_idx";

    assert_eq!(expected_query, query);
  }

  #[cfg(feature = "postgresql")]
  #[test]
  fn drop_index_builder_should_be_able_to_conditionally_add_clauses() {
    let mut drop_index = sql::DropIndex::new().drop_index("users_name_idx");

    if true {
      drop_index = drop_index.drop_index("users_age_idx");
    }

    let expected_query = "DROP INDEX users_name_idx, users_age_idx";

    assert_eq!(expected_query, drop_index.as_string());
  }

  #[cfg(not(feature = "postgresql"))]
  #[test]
  fn drop_index_builder_should_be_cloneable() {
    let drop_users = sql::DropIndex::new().drop_index("users_name_idx");
    let drop_users_and_users_name_idx = drop_users.clone().drop_index("users_name_idx");

    let expected_drop_users = "DROP INDEX users_name_idx";
    let expected_drop_users_and_users_name_idx = "DROP INDEX users_name_idx";

    assert_eq!(expected_drop_users, drop_users.as_string());
    assert_eq!(
      expected_drop_users_and_users_name_idx,
      drop_users_and_users_name_idx.as_string()
    );
  }

  #[cfg(feature = "postgresql")]
  #[test]
  fn drop_index_builder_should_be_cloneable() {
    let drop_index_name = sql::DropIndex::new().drop_index("users_name_idx");
    let drop_index_name_and_age = drop_index_name.clone().drop_index("users_age_idx");

    let expected_drop_index_name = "DROP INDEX users_name_idx";
    assert_eq!(expected_drop_index_name, drop_index_name.as_string());

    let expected_drop_index_name_and_age = "DROP INDEX users_name_idx, users_age_idx";
    assert_eq!(expected_drop_index_name_and_age, drop_index_name_and_age.as_string());
  }

  #[test]
  fn drop_index_builder_should_be_composable() {
    fn add_comment(select: sql::DropIndex) -> sql::DropIndex {
      select.raw("/* drop command */")
    }

    fn drop_users_name_idx(select: sql::DropIndex) -> sql::DropIndex {
      select.drop_index("users_name_idx")
    }

    fn as_string(select: sql::DropIndex) -> String {
      select.as_string()
    }

    let query = Some(sql::DropIndex::new())
      .map(add_comment)
      .map(drop_users_name_idx)
      .map(as_string)
      .unwrap();

    let expected_query = "/* drop command */ DROP INDEX users_name_idx";

    assert_eq!(expected_query, query);
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod builder_methods {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_new_should_initialize_as_empty_string() {
    let query = sql::DropIndex::new().as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_as_string_should_convert_the_current_state_into_string() {
    let query = sql::DropIndex::new().as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_debug_should_print_at_console_in_a_human_readable_format() {
    let query = sql::DropIndex::new()
      .drop_index_if_exists("users_name_idx")
      .debug()
      .as_string();

    let expected_query = "DROP INDEX IF EXISTS users_name_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_print_should_print_in_one_line_the_current_state_of_builder() {
    let query = sql::DropIndex::new()
      .drop_index_if_exists("users_name_idx")
      .print()
      .as_string();

    let expected_query = "DROP INDEX IF EXISTS users_name_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_add_raw_sql() {
    let query = sql::DropIndex::new().raw("drop index users_name_idx").as_string();

    let expected_query = "drop index users_name_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_accumulate_values_on_consecutive_calls() {
    let query = sql::DropIndex::new()
      .raw("drop index")
      .raw("users_name_idx")
      .as_string();

    let expected_query = "drop index users_name_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_be_the_first_to_be_concatenated() {
    let query = sql::DropIndex::new()
      .raw("/* drop index command */")
      .drop_index("users_name_idx")
      .as_string();

    let expected_query = "/* drop index command */ DROP INDEX users_name_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::DropIndex::new()
      .raw("drop index users_name_idx")
      .raw("drop index users_name_idx")
      .as_string();

    let expected_query = "drop index users_name_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_trim_space_of_the_argument() {
    let query = sql::DropIndex::new()
      .drop_index("users_name_idx")
      .raw_after(sql::DropIndexParams::DropIndex, "   /* end drop index */   ")
      .as_string();
    let expected_query = "DROP INDEX users_name_idx /* end drop index */";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_trim_space_of_the_argument() {
    let query = sql::DropIndex::new()
      .raw_before(sql::DropIndexParams::DropIndex, "  /* drop index command */  ")
      .drop_index("users_name_idx")
      .as_string();
    let expected_query = "/* drop index command */ DROP INDEX users_name_idx";

    assert_eq!(expected_query, query);
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod method_drop_index {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_drop_index_should_add_the_drop_index_signature() {
    let query = sql::DropIndex::new().drop_index("films_title_idx").as_string();
    let expected_query = "DROP INDEX films_title_idx";

    assert_eq!(expected_query, query);
  }

  #[cfg(not(feature = "postgresql"))]
  #[test]
  fn method_drop_index_should_overrides_previous_value_on_consecutive_calls() {
    let query = sql::DropIndex::new()
      .drop_index("films_title_idx")
      .drop_index("films_published_at_idx")
      .as_string();

    let expected_query = "DROP INDEX films_published_at_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_drop_index_should_trim_space_of_the_argument() {
    let query = sql::DropIndex::new().drop_index("   films_title_idx   ").as_string();
    let expected_query = "DROP INDEX films_title_idx";

    assert_eq!(expected_query, query);
  }

  #[cfg(not(feature = "postgresql"))]
  #[test]
  fn method_drop_index_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::DropIndex::new()
      .drop_index("films_title_idx")
      .drop_index("films_title_idx")
      .as_string();
    let expected_query = "DROP INDEX films_title_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_method_drop_index() {
    let query = sql::DropIndex::new()
      .raw_before(sql::DropIndexParams::DropIndex, "/* drop command */")
      .drop_index("films_title_idx")
      .as_string();
    let expected_query = "/* drop command */ DROP INDEX films_title_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_method_drop_index() {
    let query = sql::DropIndex::new()
      .drop_index("films_title_idx")
      .raw_after(sql::DropIndexParams::DropIndex, "/* end drop index */")
      .as_string();
    let expected_query = "DROP INDEX films_title_idx /* end drop index */";

    assert_eq!(expected_query, query);
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod method_drop_index_if_exists {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_drop_index_if_exists_should_add_the_drop_index_signature() {
    let query = sql::DropIndex::new()
      .drop_index_if_exists("films_title_idx")
      .as_string();
    let expected_query = "DROP INDEX IF EXISTS films_title_idx";

    assert_eq!(expected_query, query);
  }

  #[cfg(not(feature = "postgresql"))]
  #[test]
  fn method_drop_index_if_exists_should_overrides_previous_value_on_consecutive_calls() {
    let query = sql::DropIndex::new()
      .drop_index_if_exists("films_title_idx")
      .drop_index_if_exists("films_published_at_idx")
      .as_string();

    let expected_query = "DROP INDEX IF EXISTS films_published_at_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_drop_index_if_exists_should_trim_space_of_the_argument() {
    let query = sql::DropIndex::new()
      .drop_index_if_exists("   films_title_idx   ")
      .as_string();
    let expected_query = "DROP INDEX IF EXISTS films_title_idx";

    assert_eq!(expected_query, query);
  }

  #[cfg(not(feature = "postgresql"))]
  #[test]
  fn method_drop_index_if_exists_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::DropIndex::new()
      .drop_index_if_exists("films_title_idx")
      .drop_index_if_exists("films_title_idx")
      .as_string();
    let expected_query = "DROP INDEX IF EXISTS films_title_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_method_drop_index_if_exists() {
    let query = sql::DropIndex::new()
      .raw_before(sql::DropIndexParams::DropIndex, "/* drop command */")
      .drop_index_if_exists("films_title_idx")
      .as_string();
    let expected_query = "/* drop command */ DROP INDEX IF EXISTS films_title_idx";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_method_drop_index_if_exists() {
    let query = sql::DropIndex::new()
      .drop_index_if_exists("films_title_idx")
      .raw_after(sql::DropIndexParams::DropIndex, "/* end drop index */")
      .as_string();
    let expected_query = "DROP INDEX IF EXISTS films_title_idx /* end drop index */";

    assert_eq!(expected_query, query);
  }
}

#[cfg(feature = "postgresql")]
mod postgres_feature_flag {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_drop_index_should_accumulate_values_on_consecutive_calls() {
    let query = sql::DropIndex::new()
      .drop_index("films_title_idx")
      .drop_index("series")
      .as_string();

    let expected_query = "DROP INDEX films_title_idx, series";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_drop_index_if_exists_should_accumulate_values_on_consecutive_calls() {
    let query = sql::DropIndex::new()
      .drop_index_if_exists("films_title_idx")
      .drop_index_if_exists("series")
      .as_string();

    let expected_query = "DROP INDEX IF EXISTS films_title_idx, series";

    assert_eq!(expected_query, query);
  }
}
