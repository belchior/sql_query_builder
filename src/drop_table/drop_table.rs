use crate::{
  behavior::{push_unique, Concat, TransactionQuery},
  fmt,
  structure::{DropTable, DropTableParams},
};

impl TransactionQuery for DropTable {}

impl DropTable {
  /// Gets the current state of the [DropTable] and returns it as string
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::DropTable::new()
  ///   .drop_table("users")
  ///   .as_string();
  ///
  /// # let expected = "DROP TABLE users";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// DROP TABLE users
  /// ```
  pub fn as_string(&self) -> String {
    let fmts = fmt::one_line();
    self.concat(&fmts)
  }

  /// Defines a drop table parameter, this method overrides the previous value
  ///
  /// ### Example 1
  ///
  ///```
  /// # #[cfg(not(feature = "postgresql"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::DropTable::new()
  ///   .drop_table("users")
  ///   .drop_table("orders")
  ///   .as_string();
  ///
  /// # let expected = "DROP TABLE orders";
  /// # assert_eq!(expected, query);
  ///  # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// DROP TABLE orders
  /// ```
  ///
  /// ### Example 2 `crate features postgresql only`
  ///
  /// Multiples call will concatenates all values
  ///
  ///```
  /// # #[cfg(feature = "postgresql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::DropTable::new()
  ///   .drop_table("users")
  ///   .drop_table("orders")
  ///   .as_string();
  ///
  /// # let expected = "DROP TABLE users, orders";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// DROP TABLE users, orders
  /// ```
  pub fn drop_table(mut self, table_name: &str) -> Self {
    push_unique(&mut self._drop_table, table_name.trim().to_string());
    self
  }

  /// Defines a drop table parameter with the `if exists` modifier, this method overrides the previous value
  ///
  /// ### Example 1
  ///
  /// ```
  /// # #[cfg(not(feature = "postgresql"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::DropTable::new()
  ///   .drop_table("users")
  ///   .drop_table_if_exists("orders")
  ///   .to_string();
  ///
  /// # let expected = "DROP TABLE IF EXISTS orders";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// DROP TABLE IF EXISTS orders
  /// ```
  ///
  /// ### Example 2 `crate features postgresql only`
  ///
  /// Multiples call will concatenates all values
  ///
  /// ```
  /// # #[cfg(feature = "postgresql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::DropTable::new()
  ///   .drop_table("users")
  ///   .drop_table_if_exists("orders")
  ///   .to_string();
  ///
  /// # let expected = "DROP TABLE IF EXISTS users, orders";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// DROP TABLE IF EXISTS users, orders
  /// ```
  pub fn drop_table_if_exists(mut self, table_name: &str) -> Self {
    push_unique(&mut self._drop_table, table_name.trim().to_string());
    self._if_exists = true;
    self
  }

  /// Prints the current state of the [DropTable] to the standard output in a more ease to read version.
  /// This method is useful to debug complex queries or just print the generated SQL while you type
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::DropTable::new()
  ///   .drop_table("users")
  ///   .debug()
  ///   .as_string();
  /// ```
  ///
  /// Prints to the standard output
  ///
  /// ```sql
  /// -- ------------------------------------------------------------------------------
  /// DROP TABLE users
  /// -- ------------------------------------------------------------------------------
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::multiline();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// Creates instance of the [DropTable] command
  pub fn new() -> Self {
    Self::default()
  }

  /// Prints the current state of the [DropTable] to the standard output similar to debug method,
  /// the difference is that this method prints in one line.
  pub fn print(self) -> Self {
    let fmts = fmt::one_line();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// Adds at the beginning a raw SQL query. Is useful to create a more complex drop table command.
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let drop_table_query = sql::DropTable::new()
  ///   .raw("/* drop command */")
  ///   .drop_table("users_temp")
  ///   .as_string();
  ///
  /// # let expected = "/* drop command */ DROP TABLE users_temp";
  /// # assert_eq!(expected, drop_table_query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// /* drop command */ DROP TABLE users_temp
  /// ```
  pub fn raw(mut self, raw_sql: &str) -> Self {
    push_unique(&mut self._raw, raw_sql.trim().to_string());
    self
  }

  /// Adds a raw SQL query after a specified parameter.
  ///
  /// The `DropTableParams::DropTable` works both to `.drop_table` and `.drop_table_if_exist` methods
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::DropTable::new()
  ///   .drop_table("users")
  ///   .raw_after(sql::DropTableParams::DropTable, "CASCADE")
  ///   .as_string();
  ///
  /// # let expected = "DROP TABLE users CASCADE";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// DROP TABLE users CASCADE
  /// ```
  pub fn raw_after(mut self, param: DropTableParams, raw_sql: &str) -> Self {
    self._raw_after.push((param, raw_sql.trim().to_string()));
    self
  }

  /// Adds a raw SQL query before a specified parameter.
  ///
  /// The `DropTableParams::DropTable` works both to `.drop_table` and `.drop_table_if_exist` methods
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let raw = "CREATE TABLE users_temp;";
  ///
  /// let query = sql::DropTable::new()
  ///   .raw_before(sql::DropTableParams::DropTable, raw)
  ///   .drop_table("users_temp")
  ///   .as_string();
  ///
  /// # let expected = "CREATE TABLE users_temp; DROP TABLE users_temp";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// CREATE TABLE users_temp; DROP TABLE users_temp
  /// ```
  pub fn raw_before(mut self, param: DropTableParams, raw_sql: &str) -> Self {
    self._raw_before.push((param, raw_sql.trim().to_string()));
    self
  }
}

impl std::fmt::Display for DropTable {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl std::fmt::Debug for DropTable {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let fmts = fmt::multiline();
    write!(f, "{}", fmt::format(self.concat(&fmts), &fmts))
  }
}
