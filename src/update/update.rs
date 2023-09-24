#[cfg(feature = "sqlite")]
use crate::structure::UpdateVars;
use crate::{
  behavior::{push_unique, Concat, TransactionQuery, WithQuery},
  fmt,
  structure::{Update, UpdateClause},
};

impl WithQuery for Update {}

impl TransactionQuery for Update {}

impl Update {
  /// The same as [where_clause](Update::where_clause) method, useful to write more idiomatic SQL query
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let update_query = sql::Update::new()
  ///   .where_clause("login = $1")
  ///   .and("active = true")
  ///   .as_string();
  ///
  /// # let expected = "WHERE login = $1 AND active = true";
  /// # assert_eq!(update_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  ///  WHERE login = $1 AND active = true
  /// ```
  pub fn and(mut self, condition: &str) -> Self {
    self = self.where_clause(condition);
    self
  }

  /// Gets the current state of the [Update] and returns it as string
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let update_query = sql::Update::new()
  ///   .update("users")
  ///   .set("login = 'foo'")
  ///   .as_string();
  ///
  /// # let expected = "UPDATE users SET login = 'foo'";
  /// # assert_eq!(update_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  ///  UPDATE users SET login = 'foo'
  /// ```
  pub fn as_string(&self) -> String {
    let fmts = fmt::one_line();
    self.concat(&fmts)
  }

  /// Prints the current state of the [Update] to the standard output in a more ease to read version.
  /// This method is useful to debug complex queries or just print the generated SQL while you type
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let update = sql::Update::new()
  ///   .update("users")
  ///   .set("login = 'foo'")
  ///   .set("name = 'Foo'")
  ///   .debug();
  ///
  /// # let expected = "UPDATE users SET login = 'foo', name = 'Foo'";
  /// # assert_eq!(update.as_string(), expected);
  /// ```
  ///
  /// Prints to the standard output
  ///
  /// ```sql
  /// -- ------------------------------------------------------------------------------
  /// UPDATE users
  /// SET login = 'foo', name = 'Foo'
  /// -- ------------------------------------------------------------------------------
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::multiline();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// Creates instance of the Update command
  pub fn new() -> Self {
    Self::default()
  }

  /// Prints the current state of the [Update] to the standard output similar to debug method,
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
  /// let raw_query = "update users";
  ///
  /// let update_query = sql::Update::new()
  ///   .raw(raw_query)
  ///   .set("login = 'foo'")
  ///   .as_string();
  ///
  /// # let expected = "update users SET login = 'foo'";
  /// # assert_eq!(update_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// update users
  /// SET login = 'foo'
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
  /// let raw = "set name = 'Foo'";
  ///
  /// let update_query = sql::Update::new()
  ///   .update("users")
  ///   .raw_after(sql::UpdateClause::Update, raw)
  ///   .as_string();
  ///
  /// # let expected = "UPDATE users set name = 'Foo'";
  /// # assert_eq!(update_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// UPDATE users
  /// set name = 'Foo'
  /// ```
  pub fn raw_after(mut self, clause: UpdateClause, raw_sql: &str) -> Self {
    self._raw_after.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// Adds a raw SQL query before a specified clause.
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let raw = "update users";
  ///
  /// let update_query = sql::Update::new()
  ///   .raw_before(sql::UpdateClause::Set, raw)
  ///   .set("name = 'Bar'")
  ///   .as_string();
  ///
  /// # let expected = "update users SET name = 'Bar'";
  /// # assert_eq!(update_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// update users
  /// SET name = 'Bar'
  /// ```
  pub fn raw_before(mut self, clause: UpdateClause, raw_sql: &str) -> Self {
    self._raw_before.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// The `set` clause
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let update_query = sql::Update::new()
  ///   .set("name = 'Bar'")
  ///   .as_string();
  ///
  /// # let expected = "SET name = 'Bar'";
  /// # assert_eq!(update_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// SET name = 'Bar'
  /// ```
  pub fn set(mut self, value: &str) -> Self {
    push_unique(&mut self._set, value.trim().to_owned());
    self
  }

  /// The `update` clause, this method overrides the previous value
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let update_query = sql::Update::new()
  ///   .update("orders")
  ///   .as_string();
  ///
  /// # let expected = "UPDATE orders";
  /// # assert_eq!(update_query, expected);
  /// ```
  #[cfg(not(feature = "sqlite"))]
  pub fn update(mut self, table_name: &str) -> Self {
    self._update = table_name.trim().to_owned();
    self
  }

  /// The `where` clause
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let update_query = sql::Update::new()
  ///   .update("users")
  ///   .set("name = $1")
  ///   .where_clause("login = $2")
  ///   .as_string();
  ///
  /// # let expected = "UPDATE users SET name = $1 WHERE login = $2";
  /// # assert_eq!(update_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// UPDATE users SET name = $1 WHERE login = $2
  /// ```
  pub fn where_clause(mut self, condition: &str) -> Self {
    push_unique(&mut self._where, condition.trim().to_owned());
    self
  }
}

#[cfg(any(doc, feature = "postgresql", feature = "sqlite"))]
impl Update {
  /// The `from` clause, this method can be used enabling a feature flag
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let update = sql::Update::new()
  ///   .update("users")
  ///   .set("users.status = 'active'")
  ///   .from("users_bk")
  ///   .where_clause("users_bk.status = 'active'")
  ///   .debug();
  ///
  /// # let expected = "\
  /// #   UPDATE users \
  /// #   SET users.status = 'active' \
  /// #   FROM users_bk \
  /// #   WHERE users_bk.status = 'active'\
  /// # ";
  /// # assert_eq!(update.as_string(), expected);
  /// # }
  /// ```
  ///
  /// Prints to the standard output
  ///
  /// ```sql
  /// -- ------------------------------------------------------------------------------
  /// UPDATE users
  /// SET users.status = 'active'
  /// FROM users_bk
  /// WHERE users_bk.status = 'active'
  /// -- ------------------------------------------------------------------------------
  /// ```
  pub fn from(mut self, tables: &str) -> Self {
    push_unique(&mut self._from, tables.trim().to_owned());
    self
  }

  /// The `returning` clause, this method can be used enabling a feature flag
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let update_query = sql::Update::new()
  ///   .returning("name, login")
  ///   .as_string();
  ///
  /// # let expected = "RETURNING name, login";
  /// # assert_eq!(update_query, expected);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// RETURNING name, login
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
  /// let user = sql::Insert::new()
  ///   .insert_into("users(login, name)")
  ///   .values("('foo', 'Foo')")
  ///   .returning("group_id");
  ///
  /// let update = sql::Update::new()
  ///   .with("user", user)
  ///   .update("user_group")
  ///   .set("count = count + 1")
  ///   .where_clause("id = (select group_id from user)")
  ///   .debug();
  ///
  /// # let expected = "\
  /// #   WITH \
  /// #   user AS (\
  /// #     INSERT INTO users(login, name) \
  /// #     VALUES ('foo', 'Foo') \
  /// #     RETURNING group_id\
  /// #   ) \
  /// #   UPDATE user_group \
  /// #   SET count = count + 1 \
  /// #   WHERE id = (select group_id from user)\
  /// # ";
  /// # assert_eq!(update.as_string(), expected);
  /// # }
  /// ```
  ///
  /// Prints to the standard output
  ///
  /// ```sql
  /// -- ------------------------------------------------------------------------------
  /// WITH
  /// user AS (
  ///   INSERT INTO users(login, name)
  ///   VALUES ('foo', 'Foo')
  ///   RETURNING group_id
  /// )
  /// UPDATE user_group
  /// SET count = count + 1
  /// WHERE id = (select group_id from user)
  /// -- ------------------------------------------------------------------------------
  /// ```
  pub fn with(mut self, name: &str, query: impl WithQuery + 'static) -> Self {
    self._with.push((name.trim().to_owned(), std::sync::Arc::new(query)));
    self
  }
}

#[cfg(any(doc, feature = "sqlite"))]
impl Update {
  /// The `cross join` clause, this method can be used enabling the feature flag `sqlite`
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(feature = "sqlite")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let update_query = sql::Update::new()
  ///   .cross_join("orders")
  ///   .as_string();
  ///
  /// # let expected = "CROSS JOIN orders";
  /// # assert_eq!(update_query, expected);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// CROSS JOIN orders
  /// ```
  pub fn cross_join(mut self, table: &str) -> Self {
    let table = table.trim();
    let table = format!("CROSS JOIN {table}");
    push_unique(&mut self._join, table);
    self
  }

  /// The `inner join` clause, this method can be used enabling the feature flag `sqlite`
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(feature = "sqlite")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let update_query = sql::Update::new()
  ///   .inner_join("orders on orders.owner_login = users.login")
  ///   .as_string();
  ///
  /// # let expected = "INNER JOIN orders on orders.owner_login = users.login";
  /// # assert_eq!(update_query, expected);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// INNER JOIN orders on orders.owner_login = users.login
  /// ```
  pub fn inner_join(mut self, table: &str) -> Self {
    let table = table.trim();
    let table = format!("INNER JOIN {table}");
    push_unique(&mut self._join, table);
    self
  }

  /// The `left join` clause, this method can be used enabling the feature flag `sqlite`
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(feature = "sqlite")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let update_query = sql::Update::new()
  ///   .left_join("orders on orders.owner_login = users.login")
  ///   .as_string();
  ///
  /// # let expected = "LEFT JOIN orders on orders.owner_login = users.login";
  /// # assert_eq!(update_query, expected);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// LEFT JOIN orders on orders.owner_login = users.login
  /// ```
  pub fn left_join(mut self, table: &str) -> Self {
    let table = table.trim();
    let table = format!("LEFT JOIN {table}");
    push_unique(&mut self._join, table);
    self
  }

  /// The `right join` clause, this method can be used enabling the feature flag `sqlite`
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(feature = "sqlite")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let update_query = sql::Update::new()
  ///   .right_join("orders on orders.owner_login = users.login")
  ///   .as_string();
  ///
  /// # let expected = "RIGHT JOIN orders on orders.owner_login = users.login";
  /// # assert_eq!(update_query, expected);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// RIGHT JOIN orders on orders.owner_login = users.login
  /// ```
  pub fn right_join(mut self, table: &str) -> Self {
    let table = table.trim();
    let table = format!("RIGHT JOIN {table}");
    push_unique(&mut self._join, table);
    self
  }

  /// The `update` clause, this method overrides the previous value
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let update_query = sql::Update::new()
  ///   .update("orders")
  ///   .as_string();
  ///
  /// # let expected = "UPDATE orders";
  /// # assert_eq!(update_query, expected);
  /// ```
  pub fn update(mut self, table_name: &str) -> Self {
    self._update = (UpdateVars::Update, table_name.trim().to_owned());
    self
  }

  /// The `update or <keyword>` clause, this method overrides the previous value and can be used enabling the feature flag `sqlite`
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(feature = "sqlite")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let update_query = sql::Update::new()
  ///   .update_or("ABORT orders")
  ///   .as_string();
  ///
  /// # let expected = "UPDATE OR ABORT orders";
  /// # assert_eq!(update_query, expected);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// UPDATE OR ABORT orders
  /// ```
  pub fn update_or(mut self, expression: &str) -> Self {
    self._update = (UpdateVars::UpdateOr, expression.trim().to_owned());
    self
  }
}

impl std::fmt::Display for Update {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl std::fmt::Debug for Update {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let fmts = fmt::multiline();
    write!(f, "{}", fmt::format(self.concat(&fmts), &fmts))
  }
}
