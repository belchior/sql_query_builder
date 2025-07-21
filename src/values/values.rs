use crate::{
  concat::Concat,
  fmt,
  structure::{Values, ValuesClause},
  utils::push_unique,
};

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
use crate::behavior::WithQuery;

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
impl WithQuery for Values {}

impl Values {
  /// Gets the current state of the [Values] and returns it as string
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(not(feature = "mysql"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let values_query = sql::Values::new()
  ///   .values("('foo', 'Foo')")
  ///   .as_string();
  ///
  /// # let expected = "VALUES ('foo', 'Foo')";
  /// # assert_eq!(expected, values_query);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// VALUES ('foo', 'Foo')
  /// ```
  pub fn as_string(&self) -> String {
    let fmts = fmt::one_line();
    self.concat(&fmts)
  }

  /// Prints the current state of the [Values] to the standard output in a more ease to read version.
  /// This method is useful to debug complex queries or just print the generated SQL while you type
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(not(feature = "mysql"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let values = sql::Values::new()
  ///   .values("(1, 'one'), (2, 'two')")
  ///   .values("(3, 'three')")
  ///   .debug();
  ///
  /// # let expected = "VALUES (1, 'one'), (2, 'two'), (3, 'three')";
  /// # assert_eq!(expected, values.as_string());
  /// # }
  /// ```
  ///
  /// Prints to the standard output
  ///
  /// ```sql
  /// -- ------------------------------------------------------------------------------
  /// VALUES (1, 'one'), (2, 'two'), (3, 'three')
  /// -- ------------------------------------------------------------------------------
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::multiline();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// Creates instance of the Values command
  pub fn new() -> Self {
    Self::default()
  }

  /// Prints the current state of the [Values] to the standard output similar to debug method,
  /// the difference is that this method prints in one line.
  pub fn print(self) -> Self {
    let fmts = fmt::one_line();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// Adds at the beginning a raw SQL query.
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(not(feature = "mysql"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let raw_query = "insert into my_table(num, txt)";
  ///
  /// let values_query = sql::Values::new()
  ///   .raw(raw_query)
  ///   .values("(1, 'one'), (2, 'two')")
  ///   .as_string();
  ///
  /// # let expected = "insert into my_table(num, txt) VALUES (1, 'one'), (2, 'two')";
  /// # assert_eq!(expected, values_query);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// insert into my_table(num, txt)
  /// VALUES (1, 'one'), (2, 'two')
  /// ```
  pub fn raw(mut self, raw_sql: &str) -> Self {
    push_unique(&mut self._raw, raw_sql.trim().to_string());
    self
  }

  /// Adds a raw SQL query after a specified clause.
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(not(feature = "mysql"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let raw_query = ", (3, 'three')";
  ///
  /// let values_query = sql::Values::new()
  ///   .values("(1, 'one'), (2, 'two')")
  ///   .raw_after(sql::ValuesClause::Values, raw_query)
  ///   .as_string();
  ///
  /// # let expected = "VALUES (1, 'one'), (2, 'two') , (3, 'three')";
  /// # assert_eq!(expected, values_query);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// VALUES (1, 'one'), (2, 'two') , (3, 'three')
  /// ```
  pub fn raw_after(mut self, clause: ValuesClause, raw_sql: &str) -> Self {
    self._raw_after.push((clause, raw_sql.trim().to_string()));
    self
  }

  /// Adds a raw SQL query before a specified clause.
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(not(feature = "mysql"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let raw_query = "/* the values command */";
  ///
  /// let values_query = sql::Values::new()
  ///   .raw_before(sql::ValuesClause::Values, raw_query)
  ///   .values("(1, 'one'), (2, 'two')")
  ///   .as_string();
  ///
  /// # let expected = "/* the values command */ VALUES (1, 'one'), (2, 'two')";
  /// # assert_eq!(expected, values_query);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// /* the values command */
  /// VALUES (1, 'one'), (2, 'two')
  /// ```
  pub fn raw_before(mut self, clause: ValuesClause, raw_sql: &str) -> Self {
    self._raw_before.push((clause, raw_sql.trim().to_string()));
    self
  }

  /// The `values` clause
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(not(feature = "mysql"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let values_query = sql::Values::new()
  ///   .values("(1, 'one'), (2, 'two')")
  ///   .values("(3, 'three')")
  ///   .as_string();
  ///
  /// # let expected = "VALUES (1, 'one'), (2, 'two'), (3, 'three')";
  /// # assert_eq!(expected, values_query);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// VALUES (1, 'one'), (2, 'two'), (3, 'three')
  /// ```
  #[cfg(not(feature = "mysql"))]
  pub fn values(mut self, expression: &str) -> Self {
    push_unique(&mut self._values, expression.trim().to_string());
    self
  }
}

#[cfg(feature = "mysql")]
#[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
impl Values {
  /// The `values` clause with the `row` constructor clause
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let values_query = sql::Values::new()
  ///   .row("(1, 'one'), row(2, 'two')")
  ///   .row("(3, 'three')")
  ///   .as_string();
  ///
  /// # let expected = "VALUES ROW(1, 'one'), row(2, 'two'), ROW(3, 'three')";
  /// # assert_eq!(expected, values_query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// VALUES ROW(1, 'one'), row(2, 'two'), ROW(3, 'three')
  /// ```
  pub fn row(mut self, expression: &str) -> Self {
    push_unique(&mut self._values, expression.trim().to_string());
    self
  }
}

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
