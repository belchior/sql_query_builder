use crate::{
  behavior::{push_unique, Concat, WithQuery},
  fmt,
  structure::{UpdateBuilder, UpdateClause},
};

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
    let fmts = fmt::one_line();
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
    let fmts = fmt::multiline();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// The from clause, this method can be used enabling the feature flag `postgresql`
  #[cfg(feature = "postgresql")]
  pub fn from(mut self, tables: &'a str) -> Self {
    push_unique(&mut self._from, tables.trim().to_owned());
    self
  }

  /// Create UpdateBuilder's instance
  pub fn new() -> Self {
    Self::default()
  }

  /// Prints the current state of the UpdateBuilder into console output similar to debug method,
  /// the difference is that this method prints in one line.
  pub fn print(self) -> Self {
    let fmts = fmt::one_line();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
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
  /// ```
  /// use sql_query_builder::{InsertBuilder, UpdateBuilder};
  ///
  /// let user = InsertBuilder::new()
  ///   .insert_into("users(login, name)")
  ///   .values("('foo', 'Foo')")
  ///   .returning("group_id");
  /// let update = UpdateBuilder::new()
  ///   .with("user", user)
  ///   .update("user_group")
  ///   .set("count = count + 1")
  ///   .where_clause("id = (select group_id from user)")
  ///   .debug();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// WITH user AS (
  ///   INSERT INTO users(login, name)
  ///   VALUES ('foo', 'Foo')
  ///   RETURNING group_id
  /// )
  /// UPDATE user_group
  /// SET count = count + 1
  /// WHERE id = (select group_id from user)
  /// ```
  #[cfg(feature = "postgresql")]
  pub fn with(mut self, name: &'a str, query: impl WithQuery + 'static) -> Self {
    self._with.push((name.trim(), std::sync::Arc::new(query)));
    self
  }
}

impl WithQuery for UpdateBuilder<'_> {}

impl std::fmt::Display for UpdateBuilder<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl std::fmt::Debug for UpdateBuilder<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let fmts = fmt::multiline();
    write!(f, "{}", fmt::format(self.concat(&fmts), &fmts))
  }
}
