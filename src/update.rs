use crate::{
  behavior::BuilderInner,
  fmt,
  structure::{UpdateBuilder, UpdateClause},
};

impl<'a> UpdateBuilder<'a> {
  /// Gets the current state of the UpdateBuilder and returns it as string
  pub fn as_string(&self) -> String {
    let fmts = fmt::Formatter::one_line();
    self.concat(&fmts)
  }

  /// Prints the current state of the UpdateBuilder into console output in a more ease to read version.
  /// This method is useful to debug complex queries or just to print the generated SQL while you type
  /// ```
  /// use sql_query_builder::UpdateBuilder;
  ///
  /// let update_query = UpdateBuilder::new()
  ///   .update("users")
  ///   .set("login = 'foo'")
  ///   .debug()
  ///   .set("name = 'Foo'")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// UPDATE users
  /// SET login = 'foo'
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::Formatter::multi_line();
    println!("{}", fmt::colorize(self.concat(&fmts)));
    self
  }

  /// Create UpdateBuilder's instance
  pub fn new() -> Self {
    Self::default()
  }

  /// Prints the current state of the UpdateBuilder into console output similar to debug method,
  /// the difference is that this method prints in one line.
  pub fn print(self) -> Self {
    let fmts = fmt::Formatter::one_line();
    println!("{}", fmt::colorize(self.concat(&fmts)));
    self
  }

  /// Adds at the beginning a raw SQL query.
  ///
  /// ```
  /// use sql_query_builder::UpdateBuilder;
  ///
  /// let raw_query = "update users";
  /// let update_query = UpdateBuilder::new()
  ///   .raw(raw_query)
  ///   .set("login = 'foo'")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// update users
  /// SET login = 'foo'
  /// ```
  pub fn raw(mut self, raw_sql: &'a str) -> Self {
    self._raw.push(raw_sql.to_owned());
    self
  }

  /// Adds a raw SQL query after a specified clause.
  ///
  /// ```
  /// use sql_query_builder::{UpdateClause, UpdateBuilder};
  ///
  /// let raw = "set name = 'Foo'";
  /// let update_query = UpdateBuilder::new()
  ///   .update("users")
  ///   .raw_after(UpdateClause::Update, raw)
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// UPDATE users
  /// set name = 'Foo'
  /// ```
  pub fn raw_after(mut self, clause: UpdateClause, raw_sql: &'a str) -> Self {
    self._raw_after.push((clause, raw_sql.to_owned()));
    self
  }

  /// Adds a raw SQL query before a specified clause.
  ///
  /// ```
  /// use sql_query_builder::{UpdateClause, UpdateBuilder};
  ///
  /// let raw = "update users";
  /// let update_query = UpdateBuilder::new()
  ///   .raw_before(UpdateClause::Set, raw)
  ///   .set("name = 'Bar'")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// update users
  /// SET name = 'Bar'
  /// ```
  pub fn raw_before(mut self, clause: UpdateClause, raw_sql: &'a str) -> Self {
    self._raw_before.push((clause, raw_sql.to_owned()));
    self
  }

  /// The set clause
  pub fn set(mut self, value: &'a str) -> Self {
    self._set.push(value.to_owned());
    self
  }

  /// The update clause. This method overrides the previous value
  ///
  /// ```
  /// use sql_query_builder::UpdateBuilder;
  ///
  /// let update = UpdateBuilder::new()
  ///   .update("orders");
  ///
  /// let update = UpdateBuilder::new()
  ///   .update("address")
  ///   .update("orders");
  /// ```
  pub fn update(mut self, table_name: &'a str) -> Self {
    self._update = table_name;
    self
  }

  /// The where by clause
  pub fn where_clause(mut self, condition: &'a str) -> Self {
    self._where.push(condition.to_owned());
    self
  }
}

impl BuilderInner<'_, UpdateClause> for UpdateBuilder<'_> {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_owned();

    query = self.concat_raw(query, &fmts);
    query = self.concat_update(query, &fmts);
    query = self.concat_set(query, &fmts);
    query = self.concat_where(query, &fmts);

    query.trim_end().to_owned()
  }

  fn raws(&self) -> &Vec<String> {
    &self._raw
  }

  fn raw_after(&self) -> &Vec<(UpdateClause, String)> {
    &self._raw_after
  }

  fn raw_before(&self) -> &Vec<(UpdateClause, String)> {
    &self._raw_before
  }
}

impl UpdateBuilder<'_> {
  fn concat_set(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._set.is_empty() == false {
      let values = self._set.join(comma);
      format!("SET{space}{values}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(UpdateClause::Set, query, fmts, sql)
  }

  fn concat_update(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._update.is_empty() == false {
      let table_name = self._update;
      format!("UPDATE{space}{table_name}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(UpdateClause::Update, query, fmts, sql)
  }

  fn concat_where(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._where.is_empty() == false {
      let conditions = self._where.join(" AND ");
      format!("WHERE {conditions}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(UpdateClause::Where, query, fmts, sql)
  }
}

impl<'a> std::fmt::Display for UpdateBuilder<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl<'a> std::fmt::Debug for UpdateBuilder<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let fmts = fmt::Formatter::multi_line();
    write!(f, "{}", fmt::colorize(self.concat(&fmts)))
  }
}
