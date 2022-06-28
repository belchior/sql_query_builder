use crate::{
  behavior::BuilderInner,
  fmt,
  structure::{InsertBuilder, InsertClause, SelectBuilder},
};

impl<'a> InsertBuilder<'a> {
  /// Gets the current state of the InsertBuilder and returns it as string
  pub fn as_string(&self) -> String {
    let fmts = fmt::Formatter::one_line();
    self.concat(&fmts)
  }

  /// Prints the current state of the InsertBuilder into console output in a more ease to read version.
  /// This method is useful to debug complex queries or just to print the generated SQL while you type
  /// ```
  /// use sql_query_builder::InsertBuilder;
  ///
  /// let insert_query = InsertBuilder::new()
  ///   .insert_into("users (login, name)")
  ///   .values("('foo', 'Foo')")
  ///   .debug()
  ///   .values("('bar', 'Bar')")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// INSERT INTO users (login, name)
  /// VALUES ('foo', 'Foo')
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::Formatter::multi_line();
    println!("{}", fmt::colorize(self.concat(&fmts)));
    self
  }

  /// The insert into clause. This method overrides the previous value
  ///
  /// ```
  /// use sql_query_builder::InsertBuilder;
  ///
  /// let insert = InsertBuilder::new()
  ///   .insert_into("users (login, name)");
  ///
  /// let insert = InsertBuilder::new()
  ///   .insert_into("address (state, country)")
  ///   .insert_into("users (login, name)");
  /// ```
  pub fn insert_into(mut self, table_name: &'a str) -> Self {
    self._insert_into = table_name.trim();
    self
  }

  /// Create InsertBuilder's instance
  pub fn new() -> Self {
    Self::default()
  }

  /// The overriding clause
  pub fn overriding(mut self, option: &'a str) -> Self {
    self._overriding = option.trim();
    self
  }

  /// Prints the current state of the InsertBuilder into console output similar to debug method,
  /// the difference is that this method prints in one line.
  pub fn print(self) -> Self {
    let fmts = fmt::Formatter::one_line();
    println!("{}", fmt::colorize(self.concat(&fmts)));
    self
  }

  /// The select clause. This method overrides the previous value
  ///
  /// ```
  /// use sql_query_builder::{InsertClause, InsertBuilder, SelectBuilder};
  ///
  /// let insert_query = InsertBuilder::new()
  ///   .insert_into("users (login, name)")
  ///   .select(
  ///     SelectBuilder::new()
  ///       .select("login, name")
  ///       .from("users_bk")
  ///       .where_clause("active = true"),
  ///   )
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// INSERT INTO users (login, name)
  /// SELECT login, name
  /// FROM users_bk
  /// WHERE active = true
  /// ```
  pub fn select(mut self, select: SelectBuilder<'a>) -> Self {
    self._select = Some(select);
    self
  }

  /// Adds at the beginning a raw SQL query.
  ///
  /// ```
  /// use sql_query_builder::InsertBuilder;
  ///
  /// let raw_query = "insert into users (login, name)";
  /// let insert_query = InsertBuilder::new()
  ///   .raw(raw_query)
  ///   .values("('foo', 'Foo')")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// insert into users (login, name)
  /// VALUES ('bar', 'Bar')
  /// ```
  pub fn raw(mut self, raw_sql: &'a str) -> Self {
    self._raw.push(raw_sql.trim().to_owned());
    self
  }

  /// Adds a raw SQL query after a specified clause.
  ///
  /// ```
  /// use sql_query_builder::{InsertClause, InsertBuilder};
  ///
  /// let raw = "values ('foo', 'Foo')";
  /// let insert_query = InsertBuilder::new()
  ///   .insert_into("users (login, name)")
  ///   .raw_after(InsertClause::InsertInto, raw)
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// INSERT INTO users (login, name)
  /// values ('foo', 'Foo')
  /// ```
  pub fn raw_after(mut self, clause: InsertClause, raw_sql: &'a str) -> Self {
    self._raw_after.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// Adds a raw SQL query before a specified clause.
  ///
  /// ```
  /// use sql_query_builder::{InsertClause, InsertBuilder};
  ///
  /// let raw = "insert into users (login, name)";
  /// let insert_query = InsertBuilder::new()
  ///   .raw_before(InsertClause::Values, raw)
  ///   .values("('bar', 'Bar')")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// insert into users (login, name)
  /// VALUES ('bar', 'Bar')
  /// ```
  pub fn raw_before(mut self, clause: InsertClause, raw_sql: &'a str) -> Self {
    self._raw_before.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// The values clause
  pub fn values(mut self, value: &'a str) -> Self {
    self._values.push(value.trim().to_owned());
    self
  }
}

impl BuilderInner<'_, InsertClause> for InsertBuilder<'_> {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_owned();

    query = self.concat_raw(query, &fmts);
    query = self.concat_insert_into(query, &fmts);
    query = self.concat_overriding(query, &fmts);
    query = self.concat_values(query, &fmts);
    query = self.concat_select(query, &fmts);

    query.trim_end().to_owned()
  }

  fn raws(&self) -> &Vec<String> {
    &self._raw
  }

  fn raw_after(&self) -> &Vec<(InsertClause, String)> {
    &self._raw_after
  }

  fn raw_before(&self) -> &Vec<(InsertClause, String)> {
    &self._raw_before
  }
}

impl InsertBuilder<'_> {
  fn concat_insert_into(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._insert_into.is_empty() == false {
      let insert_into = self._insert_into;
      format!("INSERT INTO{space}{insert_into}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(InsertClause::InsertInto, query, fmts, sql)
  }

  fn concat_overriding(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._overriding.is_empty() == false {
      let overriding = self._overriding;
      format!("OVERRIDING {overriding}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(InsertClause::Overriding, query, fmts, sql)
  }

  fn concat_select(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if let Some(select) = &self._select {
      let select_string = select.concat(fmts);
      format!("{select_string}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(InsertClause::Select, query, fmts, sql)
  }

  fn concat_values(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._values.is_empty() == false {
      let values = self._values.join(comma);
      format!("VALUES{space}{values}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(InsertClause::Values, query, fmts, sql)
  }
}

impl<'a> std::fmt::Display for InsertBuilder<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl<'a> std::fmt::Debug for InsertBuilder<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let fmts = fmt::Formatter::multi_line();
    write!(f, "{}", fmt::colorize(self.concat(&fmts)))
  }
}
