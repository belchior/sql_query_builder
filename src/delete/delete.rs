use crate::{
  behavior::{push_unique, Concat, WithQuery},
  fmt,
  structure::{DeleteBuilder, DeleteClause},
};

impl<'a> DeleteBuilder<'a> {
  /// The same as `where_clause` method, useful to write more idiomatic SQL query
  /// ```
  /// use sql_query_builder::DeleteBuilder;
  ///
  /// let delete = DeleteBuilder::new()
  ///   .delete_from("users")
  ///   .where_clause("created_at < $1")
  ///   .and("active = false");
  /// ```
  pub fn and(mut self, condition: &'a str) -> Self {
    self = self.where_clause(condition);
    self
  }

  /// Gets the current state of the DeleteBuilder and returns it as string
  pub fn as_string(&self) -> String {
    let fmts = fmt::one_line();
    self.concat(&fmts)
  }

  /// Prints the current state of the DeleteBuilder into console output in a more ease to read version.
  /// This method is useful to debug complex queries or just to print the generated SQL while you type
  /// ```
  /// use sql_query_builder::DeleteBuilder;
  ///
  /// let delete_query = DeleteBuilder::new()
  ///   .delete_from("users")
  ///   .where_clause("login = 'foo'")
  ///   .debug()
  ///   .where_clause("name = 'Foo'")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// DELETE FROM users
  /// WHERE login = 'foo'
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::multiline();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// The delete clause. This method overrides the previous value
  ///
  /// ```
  /// use sql_query_builder::DeleteBuilder;
  ///
  /// let delete = DeleteBuilder::new()
  ///   .delete_from("orders");
  ///
  /// let delete = DeleteBuilder::new()
  ///   .delete_from("address")
  ///   .delete_from("orders");
  /// ```
  pub fn delete_from(mut self, table_name: &'a str) -> Self {
    self._delete_from = table_name.trim();
    self
  }

  /// Create DeleteBuilder's instance
  pub fn new() -> Self {
    Self::default()
  }

  /// Prints the current state of the DeleteBuilder into console output similar to debug method,
  /// the difference is that this method prints in one line.
  pub fn print(self) -> Self {
    let fmts = fmt::one_line();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// Adds at the beginning a raw SQL query.
  ///
  /// ```
  /// use sql_query_builder::DeleteBuilder;
  ///
  /// let raw_query = "delete from users";
  /// let delete_query = DeleteBuilder::new()
  ///   .raw(raw_query)
  ///   .where_clause("login = 'foo'")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// delete from users
  /// WHERE login = 'foo'
  /// ```
  pub fn raw(mut self, raw_sql: &'a str) -> Self {
    push_unique(&mut self._raw, raw_sql.trim().to_owned());
    self
  }

  /// Adds a raw SQL query after a specified clause.
  ///
  /// ```
  /// use sql_query_builder::{DeleteClause, DeleteBuilder};
  ///
  /// let raw = "where name = 'Foo'";
  /// let delete_query = DeleteBuilder::new()
  ///   .delete_from("users")
  ///   .raw_after(DeleteClause::DeleteFrom, raw)
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// DELETE FROM users
  /// where name = 'Foo'
  /// ```
  pub fn raw_after(mut self, clause: DeleteClause, raw_sql: &'a str) -> Self {
    self._raw_after.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// Adds a raw SQL query before a specified clause.
  ///
  /// ```
  /// use sql_query_builder::{DeleteClause, DeleteBuilder};
  ///
  /// let raw = "delete from users";
  /// let delete_query = DeleteBuilder::new()
  ///   .raw_before(DeleteClause::Where, raw)
  ///   .where_clause("name = 'Bar'")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// delete from users
  /// WHERE name = 'Bar'
  /// ```
  pub fn raw_before(mut self, clause: DeleteClause, raw_sql: &'a str) -> Self {
    self._raw_before.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// The returning clause, this method can be used enabling the feature flag `postgresql`
  #[cfg(feature = "postgresql")]
  pub fn returning(mut self, output_name: &'a str) -> Self {
    push_unique(&mut self._returning, output_name.trim().to_owned());
    self
  }

  /// The where clause
  /// ```
  /// use sql_query_builder::DeleteBuilder;
  ///
  /// let delete = DeleteBuilder::new()
  ///   .delete_from("users")
  ///   .where_clause("login = 'foo'");
  /// ```
  pub fn where_clause(mut self, condition: &'a str) -> Self {
    push_unique(&mut self._where, condition.trim().to_owned());
    self
  }

  /// The with clause, this method can be used enabling the feature flag `postgresql`
  /// ```
  /// use sql_query_builder::{DeleteBuilder, SelectBuilder};
  ///
  /// let deactivated_users = SelectBuilder::new().select("id").from("users").where_clause("ative = false");
  /// let delete = DeleteBuilder::new()
  ///   .with("deactivated_users", deactivated_users)
  ///   .delete_from("users")
  ///   .where_clause("id in (select * from deactivated_users)")
  ///   .debug();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// WITH deactivated_users AS (
  ///   SELECT id
  ///   FROM users
  ///   WHERE ative = false
  /// )
  /// DELETE FROM users
  /// WHERE id in (select * from deactivated_users)
  /// ```
  #[cfg(feature = "postgresql")]
  pub fn with(mut self, name: &'a str, query: impl WithQuery + 'static) -> Self {
    self._with.push((name.trim(), std::sync::Arc::new(query)));
    self
  }
}

impl WithQuery for DeleteBuilder<'_> {}

impl std::fmt::Display for DeleteBuilder<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl std::fmt::Debug for DeleteBuilder<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let fmts = fmt::multiline();
    write!(f, "{}", fmt::format(self.concat(&fmts), &fmts))
  }
}
