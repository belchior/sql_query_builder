#[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
mod full_api {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[cfg(feature = "postgresql")]
  #[test]
  fn postgres_with_all_methods() {
    let query = sql::CreateTable::new()
      // at least one of methods
      .create_table("users")
      .create_table_if_not_exists("users")
      // optional methods
      .column("id serial, login varchar(100) not null")
      .primary_key("(id)")
      .foreign_key("(address_id) references addresses(id)")
      .constraint("login users_login_key unique(login)")
      .as_string();

    let expected_query = "\
      CREATE TABLE IF NOT EXISTS users (\
        id serial, login varchar(100) not null, \
        PRIMARY KEY(id), \
        CONSTRAINT login users_login_key unique(login), \
        FOREIGN KEY(address_id) references addresses(id)\
      )\
    ";

    assert_eq!(expected_query, query);
  }

  #[cfg(feature = "sqlite")]
  #[test]
  fn sqlite_with_all_methods() {
    let query = sql::CreateTable::new()
      // at least one of methods
      .create_table("users")
      .create_table_if_not_exists("users")
      // required methods
      .column("id integer, login varchar(100) not null")
      // optional methods
      .primary_key("(id)")
      .foreign_key("(address_id) references addresses(id)")
      .constraint("login users_login_key unique(login)")
      .as_string();

    let expected_query = "\
      CREATE TABLE IF NOT EXISTS users (\
        id integer, login varchar(100) not null, \
        PRIMARY KEY(id), \
        CONSTRAINT login users_login_key unique(login), \
        FOREIGN KEY(address_id) references addresses(id)\
      )\
    ";

    assert_eq!(expected_query, query);
  }

  #[cfg(feature = "mysql")]
  #[test]
  fn mysql_with_all_methods() {
    let query = sql::CreateTable::new()
      // at least one of methods
      .create_table("users")
      .create_table_if_not_exists("users")
      // optional methods
      .column("id int not null auto_increment, login varchar(100) not null")
      .primary_key("(id)")
      .foreign_key("(address_id) references addresses(id)")
      .constraint("login users_login_key unique(login)")
      .as_string();

    let expected_query = "\
      CREATE TABLE IF NOT EXISTS users (\
        id int not null auto_increment, login varchar(100) not null, \
        PRIMARY KEY(id), \
        CONSTRAINT login users_login_key unique(login), \
        FOREIGN KEY(address_id) references addresses(id)\
      )\
    ";

    assert_eq!(expected_query, query);
  }
}

mod builder_features {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn create_table_builder_should_be_displayable() {
    let create_table = sql::CreateTable::new().create_table("orders").column("id int not null");

    println!("{}", create_table);

    let query = create_table.as_string();
    let expected_query = "CREATE TABLE orders (id int not null)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn create_table_builder_should_be_debuggable() {
    let create_table = sql::CreateTable::new().create_table("orders").column("id int not null");

    println!("{:?}", create_table);

    let expected_query = "CREATE TABLE orders (id int not null)";
    let query = create_table.as_string();

    assert_eq!(expected_query, query);
  }

  #[test]
  fn create_table_builder_should_be_cloneable() {
    let basic_table = sql::CreateTable::new()
      .column("id int")
      .column("created_at timestamp")
      .column("updated_at timestamp");

    let users_table = basic_table
      .clone()
      .create_table("users")
      .column("login varchar(40)")
      .as_string();

    let expected_users_table = "\
      CREATE TABLE users (\
        id int, \
        created_at timestamp, \
        updated_at timestamp, \
        login varchar(40)\
      )\
    ";

    let orders_table = basic_table
      .clone()
      .create_table("orders")
      .column("name varchar(200)")
      .as_string();

    let expected_orders_table = "\
      CREATE TABLE orders (\
        id int, \
        created_at timestamp, \
        updated_at timestamp, \
        name varchar(200)\
      )\
    ";

    assert_eq!(expected_users_table, users_table);
    assert_eq!(expected_orders_table, orders_table);
  }

  #[test]
  fn create_table_builder_should_be_able_to_conditionally_add_clauses() {
    let mut create_table = sql::CreateTable::new().create_table("orders");

    if true {
      create_table = create_table.column("id int");
    }

    let query = create_table.as_string();
    let expected_query = "CREATE TABLE orders (id int)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn create_table_builder_should_be_composable() {
    fn create_table(select: sql::CreateTable) -> sql::CreateTable {
      select.create_table("users")
    }

    fn columns(select: sql::CreateTable) -> sql::CreateTable {
      select.column("id int").column("login varchar(40)")
    }

    fn constraint(select: sql::CreateTable) -> sql::CreateTable {
      select
        .constraint("users_id_key primary key(id)")
        .constraint("users_login_key unique(login)")
    }

    fn as_string(select: sql::CreateTable) -> String {
      select.as_string()
    }

    let query = Some(sql::CreateTable::new())
      .map(create_table)
      .map(columns)
      .map(constraint)
      .map(as_string)
      .unwrap();

    let expected_query = "\
      CREATE TABLE users (\
        id int, \
        login varchar(40), \
        CONSTRAINT users_id_key primary key(id), \
        CONSTRAINT users_login_key unique(login)\
      )\
    ";

    assert_eq!(expected_query, query);
  }
}

mod builder_methods {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_new_should_initialize_as_empty_string() {
    let query = sql::CreateTable::new().as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_as_string_should_convert_the_current_state_into_string() {
    let query = sql::CreateTable::new().as_string();
    let expected_query = "";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_debug_should_print_at_console_in_a_human_readable_format() {
    let query = sql::CreateTable::new()
      .create_table_if_not_exists("users")
      .column("id int not null")
      .column("login varchar(40) not null")
      .constraint("users_id_key primary key(id)")
      .constraint("users_login_key unique(login)")
      .debug()
      .as_string();

    let expected_query = "\
      CREATE TABLE IF NOT EXISTS users (\
        id int not null, \
        login varchar(40) not null, \
        CONSTRAINT users_id_key primary key(id), \
        CONSTRAINT users_login_key unique(login)\
      )\
    ";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_print_should_print_in_one_line_the_current_state_of_builder() {
    let query = sql::CreateTable::new()
      .create_table_if_not_exists("users")
      .column("id int not null")
      .column("login varchar(40) not null")
      .constraint("users_id_key primary key(id)")
      .constraint("users_login_key unique(login)")
      .print()
      .as_string();

    let expected_query = "\
      CREATE TABLE IF NOT EXISTS users (\
        id int not null, \
        login varchar(40) not null, \
        CONSTRAINT users_id_key primary key(id), \
        CONSTRAINT users_login_key unique(login)\
      )\
    ";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_add_raw_sql() {
    let query = sql::CreateTable::new()
      .raw("create table local temp users ()")
      .as_string();

    let expected_query = "create table local temp users ()";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_accumulate_values_on_consecutive_calls() {
    let query = sql::CreateTable::new()
      .raw("create table local temp users (")
      .raw("id int)")
      .as_string();

    let expected_query = "create table local temp users ( id int)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::CreateTable::new()
      .raw("")
      .raw("create table local temp users (")
      .raw("id int)")
      .raw("")
      .as_string();

    let expected_query = "create table local temp users ( id int)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_be_the_first_to_be_concatenated() {
    let query = sql::CreateTable::new()
      .raw("create table local temp users")
      .column("created_at timestamp")
      .as_string();

    let expected_query = "create table local temp users (created_at timestamp)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::CreateTable::new()
      .raw("create table local temp users")
      .raw("create table local temp users")
      .as_string();

    let expected_query = "create table local temp users";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_trim_space_of_the_argument() {
    let query = sql::CreateTable::new()
      .raw_after(sql::CreateTableParams::Column, "  id int not null  ")
      .as_string();
    let expected_query = "(id int not null)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_trim_space_of_the_argument() {
    let query = sql::CreateTable::new()
      .raw_before(sql::CreateTableParams::Column, "  id int not null  ")
      .as_string();
    let expected_query = "(id int not null)";

    assert_eq!(expected_query, query);
  }
}

mod method_column {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_column_should_define_a_column_parameter() {
    let query = sql::CreateTable::new().column("login varchar(40) not null").as_string();
    let expected_query = "(login varchar(40) not null)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_column_should_accumulate_parameters_on_consecutive_calls() {
    let query = sql::CreateTable::new()
      .column("login varchar(40) not null")
      .column("created_at timestamp not null")
      .as_string();

    let expected_query = "(login varchar(40) not null, created_at timestamp not null)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_column_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::CreateTable::new()
      .column("")
      .column("login varchar(40) not null")
      .column("")
      .as_string();

    let expected_query = "(login varchar(40) not null)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_column_should_not_accumulate_parameters_with_the_same_content() {
    let query = sql::CreateTable::new()
      .column("login varchar(40) not null")
      .column("login varchar(40) not null")
      .as_string();
    let expected_query = "(login varchar(40) not null)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_column_should_trim_space_of_the_argument() {
    let query = sql::CreateTable::new().column("  login  ").as_string();
    let expected_query = "(login)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_column_parameter() {
    let raw = ", id int not null";
    let query = sql::CreateTable::new()
      .column("name varchar(100)")
      .raw_after(sql::CreateTableParams::Column, raw)
      .as_string();

    let expected_query = "(name varchar(100), id int not null)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_column_parameter() {
    let raw = "id int not null, ";
    let query = sql::CreateTable::new()
      .raw_before(sql::CreateTableParams::Column, raw)
      .column("name varchar(100)")
      .as_string();

    let expected_query = "(id int not null, name varchar(100))";

    assert_eq!(expected_query, query);
  }
}

mod method_constraint {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_constraint_should_define_a_table_constraint_parameter() {
    let query = sql::CreateTable::new()
      .constraint("login users_login_key unique(login)")
      .as_string();

    let expected_query = "(CONSTRAINT login users_login_key unique(login))";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_constraint_should_accumulate_parameters_on_consecutive_calls() {
    let query = sql::CreateTable::new()
      .constraint("id users_id_key primary key(id)")
      .constraint("login users_login_key unique(login)")
      .as_string();

    let expected_query = "(\
        CONSTRAINT id users_id_key primary key(id), \
        CONSTRAINT login users_login_key unique(login)\
      )";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_constraint_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::CreateTable::new()
      .constraint("")
      .constraint("login users_login_key unique(login)")
      .constraint("")
      .as_string();

    let expected_query = "(CONSTRAINT login users_login_key unique(login))";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_constraint_should_not_accumulate_parameters_with_the_same_content() {
    let query = sql::CreateTable::new()
      .constraint("id users_id_key primary key(id)")
      .constraint("id users_id_key primary key(id)")
      .as_string();
    let expected_query = "(CONSTRAINT id users_id_key primary key(id))";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_constraint_should_trim_space_of_the_argument() {
    let query = sql::CreateTable::new().constraint("  id  ").as_string();
    let expected_query = "(CONSTRAINT id)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_table_constraint_parameter() {
    let raw = ", id int not null";
    let query = sql::CreateTable::new()
      .constraint("id users_id_key primary key(id)")
      .raw_after(sql::CreateTableParams::Constraint, raw)
      .as_string();

    let expected_query = "(CONSTRAINT id users_id_key primary key(id), id int not null)";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_table_constraint_parameter() {
    let raw = "login varchar(40) not null, ";
    let query = sql::CreateTable::new()
      .raw_before(sql::CreateTableParams::Constraint, raw)
      .constraint("login users_login_key unique(login)")
      .as_string();

    let expected_query = "(login varchar(40) not null, CONSTRAINT login users_login_key unique(login))";

    assert_eq!(expected_query, query);
  }
}

mod method_create_table {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_create_table_should_add_the_create_table_signature() {
    let query = sql::CreateTable::new().create_table("films").as_string();
    let expected_query = "CREATE TABLE films";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_create_table_should_override_the_current_value() {
    let query = sql::CreateTable::new()
      .create_table("films")
      .create_table("series")
      .as_string();

    let expected_query = "CREATE TABLE series";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_create_table_should_trim_space_of_the_argument() {
    let query = sql::CreateTable::new().create_table("   films   ").as_string();
    let expected_query = "CREATE TABLE films";

    assert_eq!(expected_query, query);
  }
}

mod method_create_table_if_not_exists {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_create_table_should_add_the_create_table_signature() {
    let query = sql::CreateTable::new().create_table_if_not_exists("films").as_string();
    let expected_query = "CREATE TABLE IF NOT EXISTS films";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_create_table_if_not_exists_should_override_the_current_value() {
    let query = sql::CreateTable::new()
      .create_table_if_not_exists("films")
      .create_table_if_not_exists("series")
      .as_string();

    let expected_query = "CREATE TABLE IF NOT EXISTS series";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_create_table_if_not_exists_should_trim_space_of_the_argument() {
    let query = sql::CreateTable::new()
      .create_table_if_not_exists("   films   ")
      .as_string();
    let expected_query = "CREATE TABLE IF NOT EXISTS films";

    assert_eq!(expected_query, query);
  }
}

mod method_foreign_key {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_foreign_key_should_define_a_foreign_key_table_constraint() {
    let query = sql::CreateTable::new()
      .foreign_key("(user_id) REFERENCES users(id)")
      .as_string();

    let expected_query = "(FOREIGN KEY(user_id) REFERENCES users(id))";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_foreign_key_should_accumatulte_values_on_consecutive_calls() {
    let query = sql::CreateTable::new()
      .foreign_key("(users_id) REFERENCES users(id)")
      .foreign_key("(users_login) REFERENCES users(login)")
      .as_string();

    let expected_query = "(\
      FOREIGN KEY(users_id) REFERENCES users(id), \
      FOREIGN KEY(users_login) REFERENCES users(login)\
    )";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_foreign_key_should_not_accumulate_values_when_expression_is_empty() {
    let query = sql::CreateTable::new()
      .foreign_key("")
      .foreign_key("(users_login) REFERENCES users(login)")
      .foreign_key("")
      .as_string();

    let expected_query = "(FOREIGN KEY(users_login) REFERENCES users(login))";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_foreign_key_should_not_accumulate_constraints_with_the_same_content() {
    let query = sql::CreateTable::new()
      .foreign_key("(users_id) REFERENCES users(id)")
      .foreign_key("(users_id) REFERENCES users(id)")
      .as_string();

    let expected_query = "(FOREIGN KEY(users_id) REFERENCES users(id))";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_foreign_key_should_trim_space_of_the_argument() {
    let query = sql::CreateTable::new()
      .foreign_key("  (users_id) REFERENCES users(id)  ")
      .as_string();
    let expected_query = "(FOREIGN KEY(users_id) REFERENCES users(id))";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_foreign_key_constraint() {
    let raw = ", foreign key(orders_login) references orders(login)";
    let query = sql::CreateTable::new()
      .foreign_key("(orders_id) REFERENCES orders(id)")
      .raw_after(sql::CreateTableParams::ForeignKey, raw)
      .as_string();

    let expected_query = "(\
      FOREIGN KEY(orders_id) REFERENCES orders(id), \
      foreign key(orders_login) references orders(login)\
    )";

    assert_eq!(expected_query, dbg!(query));
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_foreign_key_constraint() {
    let raw = "foreign key(orders_id) references orders(id), ";
    let query = sql::CreateTable::new()
      .raw_before(sql::CreateTableParams::ForeignKey, raw)
      .foreign_key("(orders_login) REFERENCES orders(login)")
      .as_string();

    let expected_query = "(\
      foreign key(orders_id) references orders(id), \
      FOREIGN KEY(orders_login) REFERENCES orders(login)\
    )";

    assert_eq!(expected_query, query);
  }
}

mod method_primary_key {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_primary_key_should_define_a_primary_key_table_constraint() {
    let query = sql::CreateTable::new()
      .primary_key("(login) INCLUDE (name)")
      .as_string();

    let expected_query = "(PRIMARY KEY(login) INCLUDE (name))";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_primary_key_should_add_parens_when_not_defined() {
    let query = sql::CreateTable::new().primary_key("login, name").as_string();

    let expected_query = "(PRIMARY KEY(login, name))";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_primary_key_should_overrides_the_current_value_on_consecutive_calls() {
    let query = sql::CreateTable::new()
      .primary_key("id")
      .primary_key("login")
      .as_string();

    let expected_query = "(PRIMARY KEY(login))";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_primary_key_should_not_accumulate_constraints_with_the_same_content() {
    let query = sql::CreateTable::new().primary_key("id").primary_key("id").as_string();

    let expected_query = "(PRIMARY KEY(id))";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_primary_key_should_trim_space_of_the_argument() {
    let query = sql::CreateTable::new().primary_key("  login  ").as_string();
    let expected_query = "(PRIMARY KEY(login))";

    assert_eq!(expected_query, query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_primary_key_constraint() {
    let raw = ", id int not null";
    let query = sql::CreateTable::new()
      .primary_key("id")
      .raw_after(sql::CreateTableParams::PrimaryKey, raw)
      .as_string();

    let expected_query = "(PRIMARY KEY(id), id int not null)";

    assert_eq!(expected_query, dbg!(query));
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_primary_key_constraint() {
    let raw = "login varchar(40) not null, ";
    let query = sql::CreateTable::new()
      .raw_before(sql::CreateTableParams::PrimaryKey, raw)
      .primary_key("login")
      .as_string();

    let expected_query = "(login varchar(40) not null, PRIMARY KEY(login))";

    assert_eq!(expected_query, query);
  }
}
