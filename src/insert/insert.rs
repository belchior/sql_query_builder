use crate::{
  behavior::{push_unique, Concat, WithQuery},
  fmt,
  structure::{Insert, InsertClause, Select},
};

impl<'a> Insert<'a> {
  /// Gets the current state of the Insert and returns it as string
  ///
  /// # Examples
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let query = sql::Insert::new()
  ///   .insert_into("users (login)")
  ///   .values("('foo')")
  ///   .as_string();
  /// ```
  ///
  /// Output
  /// ```sql
  /// INSERT INTO users (login) VALUES ('foo')
  /// ```
  pub fn as_string(&self) -> String {
    let fmts = fmt::one_line();
    self.concat(&fmts)
  }

  /// Prints the current state of the Insert into console output in a more ease to read version.
  /// This method is useful to debug complex queries or just to print the generated SQL while you type
  ///
  /// # Examples
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let insert_query = sql::Insert::new()
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
    let fmts = fmt::multiline();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// The insert into clause. This method overrides the previous value
  ///
  /// # Examples
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let insert = sql::Insert::new()
  ///   .insert_into("users (login, name)");
  ///
  /// let insert = sql::Insert::new()
  ///   .insert_into("address (state, country)")
  ///   .insert_into("users (login, name)");
  /// ```
  pub fn insert_into(mut self, table_name: &'a str) -> Self {
    self._insert_into = table_name.trim();
    self
  }

  /// Create Insert's instance
  pub fn new() -> Self {
    Self::default()
  }

  /// The on conflict clause. This method overrides the previous value
  pub fn on_conflict(mut self, conflict: &'a str) -> Self {
    self._on_conflict = conflict.trim();
    self
  }

  /// The overriding clause. This method overrides the previous value
  pub fn overriding(mut self, option: &'a str) -> Self {
    self._overriding = option.trim();
    self
  }

  /// Prints the current state of the Insert into console output similar to debug method,
  /// the difference is that this method prints in one line.
  pub fn print(self) -> Self {
    let fmts = fmt::one_line();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// The select clause. This method overrides the previous value
  ///
  /// # Examples
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let insert_query = sql::Insert::new()
  ///   .insert_into("users (login, name)")
  ///   .select(
  ///     sql::Select::new()
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
  pub fn select(mut self, select: Select<'a>) -> Self {
    self._select = Some(select);
    self
  }

  /// Adds at the beginning a raw SQL query.
  ///
  /// # Examples
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let raw_query = "insert into users (login, name)";
  /// let insert_query = sql::Insert::new()
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
  pub fn raw(mut self, raw_sql: &str) -> Self {
    push_unique(&mut self._raw, raw_sql.trim().to_owned());
    self
  }

  /// Adds a raw SQL query after a specified clause.
  ///
  /// # Examples
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let raw = "values ('foo', 'Foo')";
  /// let insert_query = sql::Insert::new()
  ///   .insert_into("users (login, name)")
  ///   .raw_after(sql::InsertClause::InsertInto, raw)
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// INSERT INTO users (login, name)
  /// values ('foo', 'Foo')
  /// ```
  pub fn raw_after(mut self, clause: InsertClause, raw_sql: &str) -> Self {
    self._raw_after.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// Adds a raw SQL query before a specified clause.
  ///
  /// # Examples
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let raw = "insert into users (login, name)";
  /// let insert_query = sql::Insert::new()
  ///   .raw_before(sql::InsertClause::Values, raw)
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
  pub fn raw_before(mut self, clause: InsertClause, raw_sql: &str) -> Self {
    self._raw_before.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// The returning clause, this method can be used enabling the feature flag `postgresql`
  #[cfg(any(doc, feature = "postgresql"))]
  pub fn returning(mut self, output_name: &str) -> Self {
    push_unique(&mut self._returning, output_name.trim().to_owned());
    self
  }

  /// The values clause
  pub fn values(mut self, value: &str) -> Self {
    push_unique(&mut self._values, value.trim().to_owned());
    self
  }

  /// The with clause, this method can be used enabling the feature flag `postgresql`
  ///
  /// # Examples
  /// ```text
  /// use sql_query_builder as sql;
  ///
  /// let active_users = sql::Select::new().select("*").from("users_bk").where_clause("ative = true");
  /// let insert = sql::Insert::new()
  ///   .with("active_users", active_users)
  ///   .insert_into("users")
  ///   .select(sql::Select::new().select("*").from("active_users"))
  ///   .debug();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// WITH active_users AS (
  ///   SELECT *
  ///   FROM users_bk
  ///   WHERE ative = true
  /// )
  /// INSERT INTO users
  /// SELECT *
  /// FROM active_users
  /// ```
  #[cfg(any(doc, feature = "postgresql"))]
  pub fn with(mut self, name: &'a str, query: impl WithQuery + 'static) -> Self {
    self._with.push((name.trim(), std::sync::Arc::new(query)));
    self
  }
}

impl WithQuery for Insert<'_> {}

impl std::fmt::Display for Insert<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl std::fmt::Debug for Insert<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let fmts = fmt::multiline();
    write!(f, "{}", fmt::format(self.concat(&fmts), &fmts))
  }
}
