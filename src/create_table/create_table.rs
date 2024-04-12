use crate::{
  behavior::{push_unique, Concat, TransactionQuery},
  fmt,
  structure::{CreateTable, CreateTableParams},
};

impl TransactionQuery for CreateTable {}

impl CreateTable {
  /// Gets the current state of the [CreateTable] and returns it as string
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::CreateTable::new()
  ///   .create_table("users")
  ///   .column("name varchar(100) not null")
  ///   .as_string();
  ///
  /// # let expected = "\
  /// #   CREATE TABLE users (\
  /// #     name varchar(100) not null\
  /// #   )\
  /// # ";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// CREATE TABLE users (
  ///   name varchar(100) not null
  /// )
  /// ```
  pub fn as_string(&self) -> String {
    let fmts = fmt::one_line();
    self.concat(&fmts)
  }

  /// Define a column to be passed as arguments to the create table command, multiples call will concatenates all column parameters
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::CreateTable::new()
  ///   .column("id serial not null primary key")
  ///   .column("name varchar(100) not null")
  ///   .as_string();
  ///
  /// # let expected = "(\
  /// #   id serial not null primary key, \
  /// #   name varchar(100) not null\
  /// # )";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// (
  ///   id serial not null primary key,
  ///   name varchar(100) not null
  /// )
  /// ```
  pub fn column(mut self, column: &str) -> Self {
    push_unique(&mut self._column, column.trim().to_string());
    self
  }

  /// Defines a table constraint, multiples call will concatenates all constraints
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::CreateTable::new()
  ///   .constraint("users_id_key PRIMARY KEY(id)")
  ///   .constraint("users_login_key UNIQUE(login)")
  ///   .as_string();
  ///
  /// # let expected = "(\
  /// #  CONSTRAINT users_id_key PRIMARY KEY(id), \
  /// #  CONSTRAINT users_login_key UNIQUE(login)\
  /// # )";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// (
  ///   CONSTRAINT users_id_key PRIMARY KEY(id),
  ///   CONSTRAINT users_login_key UNIQUE(login)
  /// )
  /// ```
  pub fn constraint(mut self, column: &str) -> Self {
    push_unique(&mut self._constraint, column.trim().to_string());
    self
  }

  /// Defines a create table signature. Multiples calls will overrides the previous value
  ///
  /// ### Example
  ///
  ///```
  /// # use sql_query_builder as sql;
  /// let create_table = sql::CreateTable::new()
  ///   .create_table("users")
  ///   .create_table("orders");
  ///
  /// # let expected = "CREATE TABLE orders";
  /// # assert_eq!(expected, create_table.to_string());
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// CREATE TABLE orders
  /// ```
  pub fn create_table(mut self, table_name: &str) -> Self {
    self._create_table = table_name.trim().to_string();
    self
  }

  /// Defines a create table signature with the modifer `if not exists`. Multiples calls will overrides the previous value
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let create_table = sql::CreateTable::new()
  ///   .create_table("users")
  ///   .create_table_if_not_exists("orders");
  ///
  /// # let expected = "CREATE TABLE IF NOT EXISTS orders";
  /// # assert_eq!(expected, create_table.to_string());
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// CREATE TABLE IF NOT EXISTS orders
  /// ```
  pub fn create_table_if_not_exists(mut self, table_name: &str) -> Self {
    self._create_table = format!("IF NOT EXISTS {}", table_name.trim());
    self
  }

  /// Prints the current state of the [CreateTable] to the standard output in a more ease to read version.
  /// This method is useful to debug complex queries or just print the generated SQL while you type
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::CreateTable::new()
  ///   .create_table("users")
  ///   .column("name varchar(100) not null")
  ///   .column("login varchar(40) not null")
  ///   .constraint("users_login_key unique(login)")
  ///   .debug()
  ///   .as_string();
  /// ```
  ///
  /// Prints to the standard output
  ///
  /// ```sql
  /// -- ------------------------------------------------------------------------------
  /// CREATE TABLE users (
  ///   name varchar(100) not null,
  ///   login varchar(40) not null,
  ///   CONSTRAINT users_login_key unique(login)
  /// )
  /// -- ------------------------------------------------------------------------------
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::multiline();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// Defines a foreign key constraint, multiples call will concatenates all foreign keys
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::CreateTable::new()
  ///   .foreign_key("(user_id) refereces users")
  ///   .foreign_key("(address_id) refereces address(id)")
  ///   .as_string();
  ///
  /// # let expected = "(\
  /// #   FOREIGN KEY(user_id) refereces users, \
  /// #   FOREIGN KEY(address_id) refereces address(id)\
  /// # )";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// (
  ///   FOREIGN KEY(user_id) refereces users,
  ///   FOREIGN KEY(address_id) refereces address (id)
  /// )
  /// ```
  pub fn foreign_key(mut self, column: &str) -> Self {
    push_unique(&mut self._foreign_key, column.trim().to_string());
    self
  }

  /// Creates instance of the CreateTable command
  pub fn new() -> Self {
    Self::default()
  }

  /// Defines a primary key constraint. Multiples calls will overrides the previous value
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::CreateTable::new()
  ///   .create_table("users")
  ///   .primary_key("id")
  ///   .as_string();
  ///
  /// # let expected = "\
  /// #   CREATE TABLE users (\
  /// #     PRIMARY KEY(id)\
  /// #   )\
  /// # ";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// CREATE TABLE users (
  ///   PRIMARY KEY(id)
  /// )
  /// ```
  pub fn primary_key(mut self, column: &str) -> Self {
    self._primary_key = column.trim().to_string();
    self
  }

  /// Prints the current state of the [CreateTable] to the standard output similar to debug method,
  /// the difference is that this method prints in one line.
  pub fn print(self) -> Self {
    let fmts = fmt::one_line();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// Adds at the beginning a raw SQL query. Is useful to create a more complex create table signature like the example below.
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let create_table_query = sql::CreateTable::new()
  ///   .raw("CREATE LOCAL TEMP TABLE IF NOT EXISTS users_temp")
  ///   .column("login VARCHAR(40) NOT NULL")
  ///   .as_string();
  ///
  /// # let expected = "\
  /// #   CREATE LOCAL TEMP TABLE IF NOT EXISTS users_temp (\
  /// #     login VARCHAR(40) NOT NULL\
  /// #   )\
  /// # ";
  /// # assert_eq!(expected, create_table_query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// CREATE LOCAL TEMP TABLE IF NOT EXISTS users_temp (
  ///   login VARCHAR(40) NOT NULL
  /// )
  /// ```
  pub fn raw(mut self, raw_sql: &str) -> Self {
    push_unique(&mut self._raw, raw_sql.trim().to_string());
    self
  }

  /// Adds a raw SQL query after a specified parameter.
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let raw = "(name varchar(100) not null)";
  ///
  /// let query = sql::CreateTable::new()
  ///   .create_table("users")
  ///   .raw_after(sql::CreateTableParams::CreateTable, raw)
  ///   .as_string();
  ///
  /// # let expected = "CREATE TABLE users (name varchar(100) not null)";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// CREATE TABLE users (name varchar(100) not null)
  /// ```
  pub fn raw_after(mut self, param: CreateTableParams, raw_sql: &str) -> Self {
    self._raw_after.push((param, raw_sql.trim().to_string()));
    self
  }

  /// Adds a raw SQL query before a specified parameter.
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let raw = "name varchar(100) not null, ";
  ///
  /// let query = sql::CreateTable::new()
  ///   .raw_before(sql::CreateTableParams::Column, raw)
  ///   .column("login varchar(40) not null")
  ///   .as_string();
  ///
  /// # let expected = "(name varchar(100) not null, login varchar(40) not null)";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// (name varchar(100) not null, login varchar(40) not null)
  /// ```
  pub fn raw_before(mut self, param: CreateTableParams, raw_sql: &str) -> Self {
    self._raw_before.push((param, raw_sql.trim().to_string()));
    self
  }
}

impl std::fmt::Display for CreateTable {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl std::fmt::Debug for CreateTable {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let fmts = fmt::multiline();
    write!(f, "{}", fmt::format(self.concat(&fmts), &fmts))
  }
}
