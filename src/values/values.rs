use crate::{
  behavior::{push_unique, Concat, WithQuery},
  fmt,
  structure::{Values, ValuesClause},
};

impl Values {
  /// Gets the current state of the Values and returns it as string
  pub fn as_string(&self) -> String {
    let fmts = fmt::one_line();
    self.concat(&fmts)
  }

  /// Prints the current state of the Values into console output in a more ease to read version.
  /// This method is useful to debug complex queries or just to print the generated SQL while you type
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let values = sql::Values::new()
  ///   .values("(1, 'one'), (2, 'two')")
  ///   .values("(3, 'three')")
  ///   .debug();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// VALUES (1, 'one'), (2, 'two'), (3, 'three')
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::multiline();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// Create Values's instance
  pub fn new() -> Self {
    Self::default()
  }

  /// Prints the current state of the Values into console output similar to debug method,
  /// the difference is that this method prints in one line.
  pub fn print(self) -> Self {
    let fmts = fmt::one_line();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// Adds at the beginning a raw SQL query.
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let raw_query = "insert into my_table(nun, txt)";
  /// let values = sql::Values::new()
  ///   .raw(raw_query)
  ///   .values("(1, 'one'), (2, 'two')")
  ///   .debug();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// insert into my_table(num, txt)
  /// VALUES (1, 'one'), (2, 'two')
  /// ```
  pub fn raw(mut self, raw_sql: &str) -> Self {
    push_unique(&mut self._raw, raw_sql.trim().to_owned());
    self
  }

  /// Adds a raw SQL query after a specified clause.
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let raw_query = ", (3, 'three')";
  /// let values = sql::Values::new()
  ///   .values("(1, 'one'), (2, 'two')")
  ///   .raw_after(sql::ValuesClause::Values, raw_query)
  ///   .debug();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// VALUES (1, 'one'), (2, 'two') , (3, 'three')
  /// ```
  pub fn raw_after(mut self, clause: ValuesClause, raw_sql: &str) -> Self {
    self._raw_after.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// Adds a raw SQL query before a specified clause.
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let raw_query = "/* the values command */";
  /// let values = sql::Values::new()
  ///   .raw_before(sql::ValuesClause::Values, raw_query)
  ///   .values("(1, 'one'), (2, 'two')")
  ///   .debug();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// /* the values command */
  /// VALUES (1, 'one'), (2, 'two')
  /// ```
  pub fn raw_before(mut self, clause: ValuesClause, raw_sql: &str) -> Self {
    self._raw_before.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// The values clause
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let values = sql::Values::new()
  ///   .values("(1, 'one'), (2, 'two')")
  ///   .values("(3, 'three')");
  /// ```
  pub fn values(mut self, expression: &str) -> Self {
    push_unique(&mut self._values, expression.trim().to_owned());
    self
  }
}

impl WithQuery for Values {}

impl std::fmt::Display for Values {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl std::fmt::Debug for Values {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let fmts = fmt::multiline();
    write!(f, "{}", fmt::format(self.concat(&fmts), &fmts))
  }
}
