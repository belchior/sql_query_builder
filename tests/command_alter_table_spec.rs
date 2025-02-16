mod builder_features {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn alter_table_builder_should_be_displayable() {
    let alter_table = sql::AlterTable::new()
      .alter_table("orders")
      .add("COLUMN id serial not null");

    println!("{}", alter_table);

    let query = alter_table.as_string();
    let expected_query = "ALTER TABLE orders ADD COLUMN id serial not null";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn alter_table_builder_should_be_debuggable() {
    let alter_table = sql::AlterTable::new()
      .alter_table("orders")
      .add("column id serial not null");

    println!("{:?}", alter_table);

    let expected_query = "ALTER TABLE orders ADD column id serial not null";
    let query = alter_table.as_string();

    assert_eq!(expected_query, query);
  }

  #[test]
  fn alter_table_builder_should_be_cloneable() {
    let basic_alter_table = sql::AlterTable::new().alter_table("users");
    let adds_login_column = basic_alter_table.clone().add("column login varchar(40)");

    let expected_basic_alter_table = "ALTER TABLE users";
    let expected_adds_login_column = "ALTER TABLE users ADD column login varchar(40)";

    assert_eq!(expected_basic_alter_table, basic_alter_table.as_string());
    assert_eq!(expected_adds_login_column, adds_login_column.as_string());
  }

  #[test]
  fn alter_table_builder_should_be_able_to_conditionally_add_actions() {
    let mut alter_table = sql::AlterTable::new().alter_table("orders");

    if true {
      alter_table = alter_table.add("column id serial");
    }

    let query = alter_table.as_string();
    let expected_query = "ALTER TABLE orders ADD column id serial";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn alter_table_builder_should_be_composable() {
    fn alter_table(alter_table: sql::AlterTable) -> sql::AlterTable {
      alter_table.alter_table("users")
    }

    fn add_columns(alter_table: sql::AlterTable) -> sql::AlterTable {
      alter_table.add("column id serial")
    }

    fn as_string(alter_table: sql::AlterTable) -> String {
      alter_table.as_string()
    }

    let query = Some(sql::AlterTable::new())
      .map(alter_table)
      .map(add_columns)
      .map(as_string)
      .unwrap();

    let expected_query = "ALTER TABLE users ADD column id serial";

    assert_eq!(expected_query, query);
  }

  #[test]
  #[cfg(any(feature = "postgresql", feature = "mysql"))]
  fn ordered_actions_should_respect_the_orders_of_the_call() {
    let query = sql::AlterTable::new()
      // start respecting the order of the calls
      .add("COLUMN login varchar(40) not null")
      .alter("COLUMN login TYPE varchar(80)")
      .drop("COLUMN user_login")
      // end respecting the order of the calls
      .alter_table("users")
      .as_string();

    let expected_query = "\
      ALTER TABLE users \
      ADD COLUMN login varchar(40) not null, \
      ALTER COLUMN login TYPE varchar(80), \
      DROP COLUMN user_login\
    ";

    assert_eq!(expected_query, query);
  }
}

mod builder_methods {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_new_should_initialize_as_empty_string() {
    let query = sql::AlterTable::new().as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_as_string_should_convert_the_current_state_into_string() {
    let query = sql::AlterTable::new().as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_debug_should_print_at_console_in_a_human_readable_format() {
    let query = sql::AlterTable::new()
      .alter_table("users")
      .add("constraint users_login_key unique(login)")
      .debug()
      .as_string();

    let expected_query = "\
      ALTER TABLE users \
        ADD constraint users_login_key unique(login)\
    ";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_print_should_print_in_one_line_the_current_state_of_builder() {
    let query = sql::AlterTable::new()
      .alter_table("users")
      .add("constraint users_login_key unique(login)")
      .print()
      .as_string();

    let expected_query = "\
      ALTER TABLE users \
        ADD constraint users_login_key unique(login)\
    ";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_add_raw_sql_on_top_of_the_output() {
    let query = sql::AlterTable::new()
      .raw("alter table users")
      .drop("column created_at")
      .as_string();

    let expected_query = "alter table users DROP column created_at";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_accumulate_values_on_consecutive_calls() {
    let query = sql::AlterTable::new()
      .raw("alter table users")
      .raw("add column id serial")
      .as_string();

    let expected_query = "alter table users add column id serial";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::AlterTable::new()
      .raw("")
      .raw("add column id serial")
      .raw("")
      .as_string();

    let expected_query = "add column id serial";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_be_the_first_to_be_concatenated() {
    let query = sql::AlterTable::new()
      .raw("alter table users")
      .add("column created_at timestamp")
      .as_string();

    let expected_query = "alter table users ADD column created_at timestamp";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::AlterTable::new()
      .raw("alter table users")
      .raw("alter table users")
      .as_string();

    let expected_query = "alter table users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_trim_space_of_the_argument() {
    let query = sql::AlterTable::new()
      .alter_table("users")
      .raw_after(sql::AlterTableAction::AlterTable, "  add column id serial not null  ")
      .as_string();
    let expected_query = "ALTER TABLE users add column id serial not null";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_trim_space_of_the_argument() {
    let query = sql::AlterTable::new()
      .raw_before(sql::AlterTableAction::AlterTable, "  /* alter table users */  ")
      .alter_table("users")
      .as_string();
    let expected_query = "/* alter table users */ ALTER TABLE users";

    assert_eq!(expected_query, query);
  }
}

mod method_alter_table {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_alter_table_should_add_the_alter_table_signature() {
    let query = sql::AlterTable::new().alter_table("films").as_string();
    let expected_query = "ALTER TABLE films";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_alter_table_should_override_the_current_value() {
    let query = sql::AlterTable::new()
      .alter_table("films")
      .alter_table("tv_shows")
      .as_string();

    let expected_query = "ALTER TABLE tv_shows";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_alter_table_should_trim_space_of_the_argument() {
    let query = sql::AlterTable::new().alter_table("   films   ").as_string();
    let expected_query = "ALTER TABLE films";

    assert_eq!(expected_query, query);
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
mod method_rename {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_rename_should_define_a_rename_action() {
    let query = sql::AlterTable::new().rename("COLUMN login TO user_login").as_string();
    let expected_query = "RENAME COLUMN login TO user_login";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_rename_should_trim_space_of_the_argument() {
    let query = sql::AlterTable::new()
      .rename("   COLUMN login TO user_login   ")
      .as_string();
    let expected_query = "RENAME COLUMN login TO user_login";

    assert_eq!(expected_query, query);
  }

  #[cfg(not(feature = "mysql"))]
  #[test]
  fn method_rename_should_override_on_consecutive_calls() {
    let query = sql::AlterTable::new()
      .rename("COLUMN login TO user_login")
      .rename("COLUMN created TO created_at")
      .as_string();

    let expected_query = "RENAME COLUMN created TO created_at";

    assert_eq!(expected_query, query);
  }

  #[cfg(feature = "mysql")]
  #[test]
  fn method_rename_should_accumulate_rename_actions_on_consecutive_calls() {
    let query = sql::AlterTable::new()
      .rename("TO users_old")
      .rename("COLUMN name TO full_name")
      .as_string();

    let expected_query = "\
      RENAME TO users_old, \
      RENAME COLUMN name TO full_name\
    ";

    assert_eq!(expected_query, query);
  }

  #[cfg(feature = "mysql")]
  #[test]
  fn method_rename_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::AlterTable::new()
      .rename("")
      .rename("COLUMN name TO full_name")
      .rename("")
      .as_string();

    let expected_query = "RENAME COLUMN name TO full_name";

    assert_eq!(expected_query, query);
  }

  #[cfg(feature = "mysql")]
  #[test]
  fn method_rename_should_preserve_the_order_of_the_actions_on_consecutive_calls() {
    let query = sql::AlterTable::new()
      .rename("TO users_one")
      .rename("TO users_two")
      .as_string();

    let expected_query = "\
      RENAME TO users_one, \
      RENAME TO users_two\
    ";

    assert_eq!(expected_query, query);
  }

  #[cfg(feature = "mysql")]
  #[test]
  fn method_rename_should_not_accumulate_actions_with_the_same_content() {
    let query = sql::AlterTable::new()
      .rename("COLUMN street TO street_name")
      .rename("COLUMN street TO street_name")
      .as_string();
    let expected_query = "RENAME COLUMN street TO street_name";

    assert_eq!(expected_query, query);
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod method_rename_to {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_rename_to_should_add_the_rename_to_action() {
    let query = sql::AlterTable::new().rename_to("films").as_string();
    let expected_query = "RENAME TO films";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_rename_to_should_override_the_current_value() {
    let query = sql::AlterTable::new()
      .rename_to("films")
      .rename_to("tv_shows")
      .as_string();

    let expected_query = "RENAME TO tv_shows";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_rename_to_should_trim_space_of_the_argument() {
    let query = sql::AlterTable::new().rename_to("   films   ").as_string();
    let expected_query = "RENAME TO films";

    assert_eq!(expected_query, query);
  }
}

mod method_add {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_add_should_define_the_add_action() {
    let query = sql::AlterTable::new()
      .add("COLUMN login varchar(40) not null")
      .as_string();
    let expected_query = "ADD COLUMN login varchar(40) not null";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_add_should_trim_space_of_the_argument() {
    let query = sql::AlterTable::new().add("   COLUMN login   ").as_string();
    let expected_query = "ADD COLUMN login";

    assert_eq!(expected_query, query);
  }

  #[cfg(not(any(feature = "postgresql", feature = "mysql")))]
  #[test]
  fn method_add_should_override_the_current_value() {
    let query = sql::AlterTable::new()
      .add("COLUMN login varchar(40) not null")
      .add("COLUMN created_at timestamp not null")
      .as_string();

    let expected_query = "ADD COLUMN created_at timestamp not null";

    assert_eq!(expected_query, query);
  }

  #[cfg(not(any(feature = "postgresql", feature = "mysql")))]
  #[test]
  fn method_raw_before_should_add_raw_sql_before_add_action() {
    let query = sql::AlterTable::new()
      .raw_before(sql::AlterTableAction::Add, "alter table users")
      .add("COLUMN login")
      .as_string();
    let expected_query = "alter table users ADD COLUMN login";

    assert_eq!(expected_query, query);
  }

  #[cfg(not(any(feature = "postgresql", feature = "mysql")))]
  #[test]
  fn method_raw_after_should_add_raw_sql_after_add_action() {
    let query = sql::AlterTable::new()
      .add("COLUMN login")
      .raw_after(sql::AlterTableAction::Add, "/* uncommon paramenter */")
      .as_string();
    let expected_query = "ADD COLUMN login /* uncommon paramenter */";

    assert_eq!(expected_query, query);
  }

  #[cfg(any(feature = "postgresql", feature = "mysql"))]
  #[test]
  fn method_add_should_accumulate_add_actions_on_consecutive_calls() {
    let query = sql::AlterTable::new()
      .add("COLUMN login varchar(40) not null")
      .add("COLUMN created_at timestamp not null")
      .as_string();

    let expected_query = "\
      ADD COLUMN login varchar(40) not null, \
      ADD COLUMN created_at timestamp not null\
    ";

    assert_eq!(expected_query, query);
  }

  #[cfg(any(feature = "postgresql", feature = "mysql"))]
  #[test]
  fn method_add_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::AlterTable::new()
      .add("")
      .add("COLUMN created_at timestamp not null")
      .add("")
      .as_string();

    let expected_query = "ADD COLUMN created_at timestamp not null";

    assert_eq!(expected_query, query);
  }

  #[cfg(any(feature = "postgresql", feature = "mysql"))]
  #[test]
  fn method_add_should_preserve_the_order_of_the_actions_on_consecutive_calls() {
    let query = sql::AlterTable::new()
      .add("CONSTRAINT age check(age >= 0)")
      .add("COLUMN age int not null")
      .as_string();

    let expected_query = "\
      ADD CONSTRAINT age check(age >= 0), \
      ADD COLUMN age int not null\
    ";

    assert_eq!(expected_query, query);
  }

  #[cfg(any(feature = "postgresql", feature = "mysql"))]
  #[test]
  fn method_add_should_not_accumulate_actions_with_the_same_content() {
    let query = sql::AlterTable::new()
      .add("COLUMN login varchar(40) not null")
      .add("COLUMN login varchar(40) not null")
      .as_string();
    let expected_query = "ADD COLUMN login varchar(40) not null";

    assert_eq!(expected_query, query);
  }
}

mod method_drop {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_drop_should_define_a_drop_action() {
    let query = sql::AlterTable::new().drop("COLUMN login").as_string();
    let expected_query = "DROP COLUMN login";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_drop_should_trim_space_of_the_argument() {
    let query = sql::AlterTable::new().drop("   COLUMN login  ").as_string();
    let expected_query = "DROP COLUMN login";

    assert_eq!(expected_query, query);
  }

  #[cfg(not(any(feature = "postgresql", feature = "mysql")))]
  #[test]
  fn method_drop_should_override_the_current_value() {
    let query = sql::AlterTable::new()
      .drop("COLUMN login")
      .drop("COLUMN created_at")
      .as_string();

    let expected_query = "DROP COLUMN created_at";

    assert_eq!(expected_query, query);
  }

  #[cfg(not(any(feature = "postgresql", feature = "mysql")))]
  #[test]
  fn method_raw_before_should_add_raw_sql_before_drop_action() {
    let query = sql::AlterTable::new()
      .raw_before(sql::AlterTableAction::Drop, "alter table users")
      .drop("COLUMN login")
      .as_string();
    let expected_query = "alter table users DROP COLUMN login";

    assert_eq!(expected_query, query);
  }

  #[cfg(not(any(feature = "postgresql", feature = "mysql")))]
  #[test]
  fn method_raw_after_should_add_raw_sql_after_drop_action() {
    let query = sql::AlterTable::new()
      .drop("COLUMN login")
      .raw_after(sql::AlterTableAction::Drop, "/* uncommon paramenter */")
      .as_string();
    let expected_query = "DROP COLUMN login /* uncommon paramenter */";

    assert_eq!(expected_query, query);
  }

  #[cfg(any(feature = "postgresql", feature = "mysql"))]
  #[test]
  fn method_drop_should_accumulate_drop_actions_on_consecutive_calls() {
    let query = sql::AlterTable::new()
      .drop("COLUMN login")
      .drop("COLUMN created_at")
      .as_string();

    let expected_query = "\
      DROP COLUMN login, \
      DROP COLUMN created_at\
    ";

    assert_eq!(expected_query, query);
  }

  #[cfg(any(feature = "postgresql", feature = "mysql"))]
  #[test]
  fn method_drop_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::AlterTable::new()
      .drop("")
      .drop("COLUMN login")
      .drop("")
      .as_string();

    let expected_query = "DROP COLUMN login";

    assert_eq!(expected_query, query);
  }

  #[cfg(any(feature = "postgresql", feature = "mysql"))]
  #[test]
  fn method_drop_should_preserve_the_order_of_the_actions_on_consecutive_calls() {
    let query = sql::AlterTable::new()
      .drop("CONSTRAINT age")
      .drop("COLUMN age")
      .as_string();

    let expected_query = "\
      DROP CONSTRAINT age, \
      DROP COLUMN age\
    ";

    assert_eq!(expected_query, query);
  }

  #[cfg(any(feature = "postgresql", feature = "mysql"))]
  #[test]
  fn method_drop_should_not_accumulate_actions_with_the_same_content() {
    let query = sql::AlterTable::new()
      .drop("COLUMN login")
      .drop("COLUMN login")
      .as_string();
    let expected_query = "DROP COLUMN login";

    assert_eq!(expected_query, query);
  }
}

#[cfg(any(feature = "postgresql", feature = "mysql"))]
mod method_alter {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_alter_should_define_the_alter_action() {
    let query = sql::AlterTable::new()
      .alter("COLUMN login TYPE varchar(80)")
      .as_string();
    let expected_query = "ALTER COLUMN login TYPE varchar(80)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_alter_should_accumulate_alter_actions_on_consecutive_calls() {
    let query = sql::AlterTable::new()
      .alter("COLUMN login SET not null")
      .alter("COLUMN created_at SET DEFAULT now()")
      .as_string();

    let expected_query = "\
      ALTER COLUMN login SET not null, \
      ALTER COLUMN created_at SET DEFAULT now()\
    ";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_alter_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::AlterTable::new()
      .alter("")
      .alter("COLUMN created_at SET DEFAULT now()")
      .alter("")
      .as_string();

    let expected_query = "ALTER COLUMN created_at SET DEFAULT now()";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_alter_should_preserve_the_order_of_the_actions_on_consecutive_calls() {
    let query = sql::AlterTable::new()
      .alter("CONSTRAINT user_id SET not null")
      .alter("COLUMN age SET check(age >= 0)")
      .as_string();

    let expected_query = "\
      ALTER CONSTRAINT user_id SET not null, \
      ALTER COLUMN age SET check(age >= 0)\
    ";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_alter_should_not_accumulate_actions_with_the_same_content() {
    let query = sql::AlterTable::new()
      .alter("COLUMN street DROP not null")
      .alter("COLUMN street DROP not null")
      .as_string();
    let expected_query = "ALTER COLUMN street DROP not null";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_alter_should_trim_space_of_the_argument() {
    let query = sql::AlterTable::new()
      .alter("   COLUMN street DROP not null  ")
      .as_string();
    let expected_query = "ALTER COLUMN street DROP not null";

    assert_eq!(expected_query, query);
  }
}
