use crate::{
  behavior::TransactionQuery,
  concat::Concat,
  fmt,
  structure::{DropIndex, DropIndexParams},
  utils::push_unique,
};

impl TransactionQuery for DropIndex {}

impl DropIndex {
  /// Gets the current state of the [DropIndex] and returns it as string
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::DropIndex::new()
  ///   .drop_index("users_name_idx")
  ///   .as_string();
  ///
  /// # let expected = "DROP INDEX users_name_idx";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// DROP INDEX users_name_idx
  /// ```
  pub fn as_string(&self) -> String {
    let fmts = fmt::one_line();
    self.concat(&fmts)
  }

  /// Defines a drop index parameter, this method overrides the previous value
  ///
  /// ### Example 1
  ///
  ///```
  /// # #[cfg(not(feature = "postgresql"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::DropIndex::new()
  ///   .drop_index("users_name_idx")
  ///   .drop_index("orders_product_name_idx")
  ///   .as_string();
  ///
  /// # let expected = "DROP INDEX orders_product_name_idx";
  /// # assert_eq!(expected, query);
  ///  # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// DROP INDEX orders_product_name_idx
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
  /// let query = sql::DropIndex::new()
  ///   .drop_index("users_name_idx")
  ///   .drop_index("orders_product_name_idx")
  ///   .as_string();
  ///
  /// # let expected = "DROP INDEX users_name_idx, orders_product_name_idx";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// DROP INDEX users_name_idx, orders_product_name_idx
  /// ```
  pub fn drop_index(mut self, index_name: &str) -> Self {
    push_unique(&mut self._drop_index, index_name.trim().to_string());
    self
  }

  /// Defines a drop index parameter with the `if exists` modifier, this method overrides the previous value
  ///
  /// ### Example 1
  ///
  /// ```
  /// # #[cfg(not(feature = "postgresql"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::DropIndex::new()
  ///   .drop_index("users_name_idx")
  ///   .drop_index_if_exists("orders_product_name_idx")
  ///   .to_string();
  ///
  /// # let expected = "DROP INDEX IF EXISTS orders_product_name_idx";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// DROP INDEX IF EXISTS orders_product_name_idx
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
  /// let query = sql::DropIndex::new()
  ///   .drop_index("users_name_idx")
  ///   .drop_index_if_exists("orders_product_name_idx")
  ///   .to_string();
  ///
  /// # let expected = "DROP INDEX IF EXISTS users_name_idx, orders_product_name_idx";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// DROP INDEX IF EXISTS users_name_idx, orders_product_name_idx
  /// ```
  pub fn drop_index_if_exists(mut self, index_name: &str) -> Self {
    push_unique(&mut self._drop_index, index_name.trim().to_string());
    self._if_exists = true;
    self
  }

  /// Prints the current state of the [DropIndex] to the standard output in a more ease to read version.
  /// This method is useful to debug complex queries or just print the generated SQL while you type
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::DropIndex::new()
  ///   .drop_index("users_name_idx")
  ///   .debug()
  ///   .as_string();
  /// ```
  ///
  /// Prints to the standard output
  ///
  /// ```sql
  /// -- ------------------------------------------------------------------------------
  /// DROP INDEX users_name_idx
  /// -- ------------------------------------------------------------------------------
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::multiline();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// Creates instance of the [DropIndex] command
  pub fn new() -> Self {
    Self::default()
  }

  /// Prints the current state of the [DropIndex] to the standard output similar to debug method,
  /// the difference is that this method prints in one line.
  pub fn print(self) -> Self {
    let fmts = fmt::one_line();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// Adds at the beginning a raw SQL query. Is useful to create a more complex drop index command.
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let drop_index_query = sql::DropIndex::new()
  ///   .raw("/* drop index command */")
  ///   .drop_index("users_name_idx")
  ///   .as_string();
  ///
  /// # let expected = "/* drop index command */ DROP INDEX users_name_idx";
  /// # assert_eq!(expected, drop_index_query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// /* drop index command */ DROP INDEX users_name_idx
  /// ```
  pub fn raw(mut self, raw_sql: &str) -> Self {
    push_unique(&mut self._raw, raw_sql.trim().to_string());
    self
  }

  /// Adds a raw SQL query after a specified parameter.
  ///
  /// The `DropIndexParams::DropIndex` works both to `.drop_index` and `.drop_index_if_exist` methods
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::DropIndex::new()
  ///   .drop_index("users_name_idx")
  ///   .raw_after(sql::DropIndexParams::DropIndex, "/* end drop index */")
  ///   .as_string();
  ///
  /// # let expected = "DROP INDEX users_name_idx /* end drop index */";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// DROP INDEX users_name_idx /* end drop index */
  /// ```
  pub fn raw_after(mut self, param: DropIndexParams, raw_sql: &str) -> Self {
    self._raw_after.push((param, raw_sql.trim().to_string()));
    self
  }

  /// Adds a raw SQL query before a specified parameter.
  ///
  /// The `DropIndexParams::DropIndex` works both to `.drop_index` and `.drop_index_if_exist` methods
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let raw = "/* drop index command */";
  ///
  /// let query = sql::DropIndex::new()
  ///   .raw_before(sql::DropIndexParams::DropIndex, raw)
  ///   .drop_index("users_name_idx")
  ///   .as_string();
  ///
  /// # let expected = "/* drop index command */ DROP INDEX users_name_idx";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// /* drop index command */ DROP INDEX users_name_idx
  /// ```
  pub fn raw_before(mut self, param: DropIndexParams, raw_sql: &str) -> Self {
    self._raw_before.push((param, raw_sql.trim().to_string()));
    self
  }
}

impl std::fmt::Display for DropIndex {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl std::fmt::Debug for DropIndex {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let fmts = fmt::multiline();
    write!(f, "{}", fmt::format(self.concat(&fmts), &fmts))
  }
}
