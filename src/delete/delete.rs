use crate::{
  behavior::{push_unique, Concat, TransactionQuery, WithQuery},
  fmt,
  structure::{Delete, DeleteClause},
};

impl WithQuery for Delete {}

impl TransactionQuery for Delete {}

impl Delete {
  /// The same as [where_clause](Delete::where_clause) method, useful to write more idiomatic SQL query
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let delete = sql::Delete::new()
  ///   .delete_from("users")
  ///   .where_clause("created_at < $1")
  ///   .and("active = false");
  ///
  /// # let expected = "DELETE FROM users WHERE created_at < $1 AND active = false";
  /// # assert_eq!(delete.to_string(), expected);
  /// ```
  pub fn and(mut self, condition: &str) -> Self {
    self = self.where_clause(condition);
    self
  }

  /// Gets the current state of the [Delete] and returns it as string
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::Delete::new()
  ///   .delete_from("users")
  ///   .where_clause("id = $1")
  ///   .as_string();
  ///
  /// # let expected = "DELETE FROM users WHERE id = $1";
  /// # assert_eq!(query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// DELETE FROM users WHERE id = $1
  /// ```
  pub fn as_string(&self) -> String {
    let fmts = fmt::one_line();
    self.concat(&fmts)
  }

  /// Prints the current state of the [Delete] to the standard output in a more ease to read version.
  /// This method is useful to debug complex queries or just print the generated SQL while you type
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let delete_query = sql::Delete::new()
  ///   .delete_from("users")
  ///   .where_clause("login = 'foo'")
  ///   .debug()
  ///   .where_clause("name = 'Foo'")
  ///   .as_string();
  /// ```
  ///
  /// Prints to the standard output
  ///
  /// ```sql
  /// -- ------------------------------------------------------------------------------
  /// DELETE FROM users
  /// WHERE login = 'foo'
  /// -- ------------------------------------------------------------------------------
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::multiline();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// The `delete` clause. This method overrides the previous value
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let delete = sql::Delete::new()
  ///   .delete_from("orders");
  /// #
  /// # let expected = "DELETE FROM orders";
  /// # assert_eq!(delete.to_string(), expected);
  ///
  /// let delete = sql::Delete::new()
  ///   .delete_from("addresses")
  ///   .delete_from("orders");
  ///
  /// # let expected = "DELETE FROM orders";
  /// # assert_eq!(delete.to_string(), expected);
  /// ```
  pub fn delete_from(mut self, table_name: &str) -> Self {
    self._delete_from = table_name.trim().to_owned();
    self
  }

  /// Creates instance of the Delete command
  pub fn new() -> Self {
    Self::default()
  }

  /// Prints the current state of the [Delete] to the standard output similar to debug method,
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
  /// # use sql_query_builder as sql;
  /// let raw_query = "delete from users";
  /// let delete_query = sql::Delete::new()
  ///   .raw(raw_query)
  ///   .where_clause("login = 'foo'")
  ///   .as_string();
  ///
  /// # let expected = "delete from users WHERE login = 'foo'";
  /// # assert_eq!(delete_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// delete from users WHERE login = 'foo'
  /// ```
  pub fn raw(mut self, raw_sql: &str) -> Self {
    push_unique(&mut self._raw, raw_sql.trim().to_owned());
    self
  }

  /// Adds a raw SQL query after a specified clause.
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let raw = "where name = 'Foo'";
  /// let delete_query = sql::Delete::new()
  ///   .delete_from("users")
  ///   .raw_after(sql::DeleteClause::DeleteFrom, raw)
  ///   .as_string();
  ///
  /// # let expected = "DELETE FROM users where name = 'Foo'";
  /// # assert_eq!(delete_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// DELETE FROM users where name = 'Foo'
  /// ```
  pub fn raw_after(mut self, clause: DeleteClause, raw_sql: &str) -> Self {
    self._raw_after.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// Adds a raw SQL query before a specified clause.
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let raw = "delete from users";
  /// let delete_query = sql::Delete::new()
  ///   .raw_before(sql::DeleteClause::Where, raw)
  ///   .where_clause("name = 'Bar'")
  ///   .as_string();
  ///
  /// # let expected = "delete from users WHERE name = 'Bar'";
  /// # assert_eq!(delete_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// delete from users WHERE name = 'Bar'
  /// ```
  pub fn raw_before(mut self, clause: DeleteClause, raw_sql: &str) -> Self {
    self._raw_before.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// The `where` clause
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let delete = sql::Delete::new()
  ///   .delete_from("users")
  ///   .where_clause("login = 'foo'")
  ///   .where_clause("name = 'Foo'");
  ///
  /// # let expected = "DELETE FROM users WHERE login = 'foo' AND name = 'Foo'";
  /// # assert_eq!(delete.to_string(), expected);
  /// ```
  pub fn where_clause(mut self, condition: &str) -> Self {
    push_unique(&mut self._where, condition.trim().to_owned());
    self
  }
}

#[cfg(any(doc, feature = "postgresql", feature = "sqlite"))]
impl Delete {
  /// The `returning` clause, this method can be used enabling a feature flag
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let delete = sql::Delete::new()
  ///   .delete_from("users")
  ///   .returning("id")
  ///   .returning("login");
  ///
  /// # let expected = "DELETE FROM users RETURNING id, login";
  /// # assert_eq!(delete.to_string(), expected);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// DELETE FROM users RETURNING id, login
  /// ```
  pub fn returning(mut self, output_name: &str) -> Self {
    push_unique(&mut self._returning, output_name.trim().to_owned());
    self
  }

  /// The `with` clause, this method can be used enabling a feature flag
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let deactivated_users = sql::Select::new().select("id").from("users").where_clause("ative = false");
  /// let delete = sql::Delete::new()
  ///   .with("deactivated_users", deactivated_users)
  ///   .delete_from("users")
  ///   .where_clause("id in (select * from deactivated_users)")
  ///   .debug();
  ///
  /// # let expected = "\
  ///   WITH deactivated_users AS (\
  ///     SELECT id \
  ///     FROM users \
  ///     WHERE ative = false\
  ///   ) \
  ///   DELETE FROM users \
  ///   WHERE id in (select * from deactivated_users)\
  /// ";
  /// # assert_eq!(delete.to_string(), expected);
  /// # }
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
  pub fn with(mut self, name: &str, query: impl WithQuery + 'static) -> Self {
    self._with.push((name.trim().to_owned(), std::sync::Arc::new(query)));
    self
  }
}

impl std::fmt::Display for Delete {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl std::fmt::Debug for Delete {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let fmts = fmt::multiline();
    write!(f, "{}", fmt::format(self.concat(&fmts), &fmts))
  }
}
