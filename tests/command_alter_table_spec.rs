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

    assert_eq!(expected_basic_alter_table, basic_alter_table.to_string());
    assert_eq!(expected_adds_login_column, adds_login_column.to_string());
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
      alter_table.add("column id serial").add("column login varchar(40)")
    }

    fn add_constraints(alter_table: sql::AlterTable) -> sql::AlterTable {
      alter_table
        .add("constraint users_id_key primary key(id)")
        .add("constraint users_login_key unique(login)")
    }

    fn as_string(alter_table: sql::AlterTable) -> String {
      alter_table.as_string()
    }

    let query = Some(sql::AlterTable::new())
      .map(alter_table)
      .map(add_columns)
      .map(add_constraints)
      .map(as_string)
      .unwrap();

    let expected_query = "\
      ALTER TABLE users \
        ADD column id serial, \
        ADD column login varchar(40), \
        ADD constraint users_id_key primary key(id), \
        ADD constraint users_login_key unique(login)\
    ";

    assert_eq!(expected_query, query);
  }

  #[test]
  #[cfg(any(feature = "postgresql"))]
  fn ordered_actions_should_respect_the_orders_of_the_call() {
    let query = sql::AlterTable::new()
      // start respecting the order of the calls
      .add("COLUMN login varchar(40) not null")
      .alter("COLUMN login TYPE varchar(80)")
      .rename("COLUMN login TO user_login")
      .drop("COLUMN user_login")
      // end respecting the order of the calls
      .rename_to("films")
      .alter_table("users")
      .as_string();

    let expected_query = "\
      ALTER TABLE users \
      RENAME TO films, \
      ADD COLUMN login varchar(40) not null, \
      ALTER COLUMN login TYPE varchar(80), \
      RENAME COLUMN login TO user_login, \
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
      .add("column id serial not null")
      .add("column login varchar(40) not null")
      .add("constraint users_id_key primary key(id)")
      .add("constraint users_login_key unique(login)")
      .debug()
      .as_string();

    let expected_query = "\
      ALTER TABLE users \
        ADD column id serial not null, \
        ADD column login varchar(40) not null, \
        ADD constraint users_id_key primary key(id), \
        ADD constraint users_login_key unique(login)\
    ";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_print_should_print_in_one_line_the_current_state_of_builder() {
    let query = sql::AlterTable::new()
      .alter_table("users")
      .add("column id serial not null")
      .add("column login varchar(40) not null")
      .add("constraint users_id_key primary key(id)")
      .add("constraint users_login_key unique(login)")
      .print()
      .as_string();

    let expected_query = "\
      ALTER TABLE users \
        ADD column id serial not null, \
        ADD column login varchar(40) not null, \
        ADD constraint users_id_key primary key(id), \
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
      .raw_after(sql::AlterTableAction::AlterTable, "add column id serial not null  ")
      .as_string();
    let expected_query = "ALTER TABLE users add column id serial not null";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_trim_space_of_the_argument() {
    let query = sql::AlterTable::new()
      .raw_before(sql::AlterTableAction::AlterTable, "/* alter table users */")
      .alter_table("users")
      .as_string();
    let expected_query = "/* alter table users */ ALTER TABLE users";

    assert_eq!(expected_query, query);
  }
}

mod method_add {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_add_should_define_a_add_action() {
    let query = sql::AlterTable::new()
      .add("COLUMN login varchar(40) not null")
      .as_string();
    let expected_query = "ADD COLUMN login varchar(40) not null";

    assert_eq!(expected_query, query);
  }

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

  #[test]
  fn method_add_should_not_accumulate_actions_with_the_same_content() {
    let query = sql::AlterTable::new()
      .add("COLUMN login varchar(40) not null")
      .add("COLUMN login varchar(40) not null")
      .as_string();
    let expected_query = "ADD COLUMN login varchar(40) not null";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_add_should_trim_space_of_the_argument() {
    let query = sql::AlterTable::new().add("   COLUMN login  ").as_string();
    let expected_query = "ADD COLUMN login";

    assert_eq!(expected_query, query);
  }
}

#[cfg(any(feature = "postgresql"))]
mod method_alter {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_alter_should_define_a_alter_action() {
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

  #[test]
  fn method_drop_should_not_accumulate_actions_with_the_same_content() {
    let query = sql::AlterTable::new()
      .drop("COLUMN login")
      .drop("COLUMN login")
      .as_string();
    let expected_query = "DROP COLUMN login";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_drop_should_trim_space_of_the_argument() {
    let query = sql::AlterTable::new().drop("   COLUMN login  ").as_string();
    let expected_query = "DROP COLUMN login";

    assert_eq!(expected_query, query);
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
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
  fn method_rename_should_accumulate_rename_actions_on_consecutive_calls() {
    let query = sql::AlterTable::new()
      .rename("COLUMN login TO user_login")
      .rename("COLUMN created TO created_at")
      .as_string();

    let expected_query = "\
      RENAME COLUMN login TO user_login, \
      RENAME COLUMN created TO created_at\
    ";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_rename_should_preserve_the_order_of_the_actions_on_consecutive_calls() {
    let query = sql::AlterTable::new()
      .rename("CONSTRAINT age TO birthdate")
      .rename("COLUMN age TO birthdate")
      .as_string();

    let expected_query = "\
      RENAME CONSTRAINT age TO birthdate, \
      RENAME COLUMN age TO birthdate\
    ";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_rename_should_not_accumulate_actions_with_the_same_content() {
    let query = sql::AlterTable::new()
      .rename("COLUMN login TO user_login")
      .rename("COLUMN login TO user_login")
      .as_string();
    let expected_query = "RENAME COLUMN login TO user_login";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_rename_should_trim_space_of_the_argument() {
    let query = sql::AlterTable::new()
      .rename("   COLUMN login TO user_login  ")
      .as_string();
    let expected_query = "RENAME COLUMN login TO user_login";

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
