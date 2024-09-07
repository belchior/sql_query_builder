use crate::{
  behavior::{push_unique, Concat, TransactionQuery},
  fmt,
  structure::{Delete, DeleteClause, LogicalOperator},
};

impl TransactionQuery for Delete {}

impl Delete {
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
    self._delete_from = table_name.trim().to_string();
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
  ///
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
    push_unique(&mut self._raw, raw_sql.trim().to_string());
    self
  }

  /// Adds a raw SQL query after a specified clause.
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let raw = "where name = 'Foo'";
  ///
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
    self._raw_after.push((clause, raw_sql.trim().to_string()));
    self
  }

  /// Adds a raw SQL query before a specified clause.
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let raw = "delete from users";
  ///
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
    self._raw_before.push((clause, raw_sql.trim().to_string()));
    self
  }

  /// This method is un alias of `where_clause`. The `where_and` will concatenate mulltiples calls using the `and` operator.
  /// The intention is to enable more idiomatic concatenation of conditions.
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let delete_query = sql::Delete::new()
  ///   .where_clause("login = $1")
  ///   .where_and("product_id = $2")
  ///   .where_and("created_at >= current_date")
  ///   .as_string();
  ///
  /// # let expected = "WHERE login = $1 AND product_id = $2 AND created_at >= current_date";
  /// # assert_eq!(delete_query, expected);
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// WHERE
  ///   login = $1
  ///   AND product_id = $2
  ///   AND created_at >= current_date
  /// ```
  pub fn where_and(self, condition: &str) -> Self {
    self.where_clause(condition)
  }

  /// The `where` clause, this method will concatenate mulltiples calls using the `and` operator.
  /// If you intended to use the `or` operator you should use the [where_or](Delete::where_or) method
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let delete_query = sql::Delete::new()
  ///   .where_clause("login = $1")
  ///   .where_clause("status = 'deactivated'")
  ///   .as_string();
  ///
  /// # let expected = "WHERE login = $1 AND status = 'deactivated'";
  /// # assert_eq!(delete_query, expected);
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// WHERE
  ///   login = $1
  ///   AND status = 'deactivated'
  /// ```
  pub fn where_clause(mut self, condition: &str) -> Self {
    push_unique(&mut self._where, (LogicalOperator::And, condition.trim().to_string()));
    self
  }

  /// The `where` clause that concatenate multiples calls using the OR operator.
  /// If you intended to use the `and` operator you should use the [where_clause](Delete::where_clause) method
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let delete_query = sql::Delete::new()
  ///   .where_clause("login = 'foo'")
  ///   .where_or("login = 'bar'")
  ///   .as_string();
  ///
  /// # let expected = "WHERE login = 'foo' OR login = 'bar'";
  /// # assert_eq!(delete_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// WHERE
  ///   login = 'foo'
  ///   OR login = 'bar'
  /// ```
  pub fn where_or(mut self, condition: &str) -> Self {
    push_unique(&mut self._where, (LogicalOperator::Or, condition.trim().to_string()));
    self
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
use crate::behavior::WithQuery;

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
impl WithQuery for Delete {}

#[cfg(any(doc, feature = "postgresql", feature = "sqlite"))]
#[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
impl Delete {
  /// The `returning` clause
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
    push_unique(&mut self._returning, output_name.trim().to_string());
    self
  }

  /// The `with` clause
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
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  pub fn with(mut self, name: &str, query: impl WithQuery + 'static) -> Self {
    self._with.push((name.trim().to_string(), std::sync::Arc::new(query)));
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
