use crate::{
  behavior::{push_unique, Concat, Query, Raw},
  fmt,
  structure::{UpdateBuilder, UpdateClause},
};
use std::sync::Arc;

impl<'a> UpdateBuilder<'a> {
  /// The same as `where_clause` method, useful to write more idiomatic SQL query
  /// ```
  /// use sql_query_builder::UpdateBuilder;
  ///
  /// let update = UpdateBuilder::new()
  ///   .update("users")
  ///   .set("name = $1")
  ///   .where_clause("login = $2")
  ///   .and("active = true");
  /// ```
  pub fn and(mut self, condition: &'a str) -> Self {
    self = self.where_clause(condition);
    self
  }

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
    push_unique(&mut self._raw, raw_sql.trim().to_owned());
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
    self._raw_after.push((clause, raw_sql.trim().to_owned()));
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
    self._raw_before.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// The returning clause, this method can be used enabling the feature flag `postgresql`
  #[cfg(feature = "postgresql")]
  pub fn returning(mut self, output_name: &'a str) -> Self {
    push_unique(&mut self._returning, output_name.trim().to_owned());
    self
  }

  /// The set clause
  pub fn set(mut self, value: &'a str) -> Self {
    push_unique(&mut self._set, value.trim().to_owned());
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
    self._update = table_name.trim();
    self
  }

  /// The where clause
  /// ```
  /// use sql_query_builder::UpdateBuilder;
  ///
  /// let update = UpdateBuilder::new()
  ///   .update("users")
  ///   .set("name = $1")
  ///   .where_clause("login = $2");
  /// ```
  pub fn where_clause(mut self, condition: &'a str) -> Self {
    push_unique(&mut self._where, condition.trim().to_owned());
    self
  }

  /// The with clause, this method can be used enabling the feature flag `postgresql`
  #[cfg(feature = "postgresql")]
  pub fn with(mut self, name: &'a str, query: impl Query + 'static) -> Self {
    self._with.push((name.trim(), Arc::new(query)));
    self
  }
}

impl Query for UpdateBuilder<'_> {}

impl Concat for UpdateBuilder<'_> {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_owned();

    query = self.concat_raw(query, &fmts);
    #[cfg(feature = "postgresql")]
    {
      query = self.concat_with(query, &fmts);
    }
    query = self.concat_update(query, &fmts);
    query = self.concat_set(query, &fmts);
    query = self.concat_where(query, &fmts);

    #[cfg(feature = "postgresql")]
    {
      query = self.concat_returning(query, &fmts);
    }

    query.trim_end().to_owned()
  }
}

impl UpdateBuilder<'_> {
  #[cfg(feature = "postgresql")]
  fn concat_returning(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, comma, .. } = fmts;
    let sql = if self._returning.is_empty() == false {
      let output_names = self._returning.join(comma);
      format!("RETURNING{space}{output_names}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(UpdateClause::Returning, query, fmts, sql)
  }

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

  #[cfg(feature = "postgresql")]
  fn concat_with(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter {
      comma,
      lb,
      indent,
      space,
    } = fmts;
    let sql = if self._with.is_empty() == false {
      let with = self._with.iter().fold("".to_owned(), |acc, item| {
        let (name, query) = item;
        let inner_lb = format!("{lb}{indent}");
        let inner_fmts = fmt::Formatter {
          comma,
          lb: inner_lb.as_str(),
          indent,
          space,
        };
        let query_string = query.concat(&inner_fmts);

        format!("{acc}{name}{space}AS{space}({lb}{indent}{query_string}{lb}){comma}")
      });
      let with = &with[..with.len() - comma.len()];

      format!("WITH{space}{with}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(UpdateClause::With, query, fmts, sql)
  }
}

impl Raw<'_, UpdateClause> for UpdateBuilder<'_> {
  fn _raw(&self) -> &Vec<String> {
    &self._raw
  }

  fn _raw_after(&self) -> &Vec<(UpdateClause, String)> {
    &self._raw_after
  }

  fn _raw_before(&self) -> &Vec<(UpdateClause, String)> {
    &self._raw_before
  }
}

impl std::fmt::Display for UpdateBuilder<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl std::fmt::Debug for UpdateBuilder<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let fmts = fmt::Formatter::multi_line();
    write!(f, "{}", fmt::colorize(self.concat(&fmts)))
  }
}