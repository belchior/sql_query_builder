use crate::{
  behavior::{push_unique, Concat, TransactionQuery, WithQuery},
  fmt,
  structure::{Select, SelectClause},
};

impl TransactionQuery for Select {}

impl WithQuery for Select {}

impl Select {
  /// The same as [where_clause](Select::where_clause) method, useful to write more idiomatic SQL query
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let select = sql::Select::new()
  ///   .where_clause("login = foo")
  ///   .and("active = true");
  /// ```
  pub fn and(mut self, condition: &str) -> Self {
    self = self.where_clause(condition);
    self
  }

  /// Gets the current state of the [Select] and returns it as string
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let query = sql::Select::new()
  ///   .select("id")
  ///   .from("users")
  ///   .where_clause("login = 'foo'")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// SELECT id FROM users WHERE login = 'foo'
  /// ```
  pub fn as_string(&self) -> String {
    let fmts = fmt::one_line();
    self.concat(&fmts)
  }

  /// Prints the current state of the Select into console output in a more ease to read version.
  /// This method is useful to debug complex queries or just to print the generated SQL while you type
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let select = sql::Select::new()
  ///   .select("*")
  ///   .from("users")
  ///   .where_clause("login = foo")
  ///   .and("active = true")
  ///   .debug();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// SELECT *
  /// FROM users
  /// WHERE login = foo AND active = true
  /// ```
  ///
  /// You can debug different parts of the select putting it in another position
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let select_query = sql::Select::new()
  ///   .select("*")
  ///   .from("users")
  ///   .debug()
  ///   .where_clause("login = foo")
  ///   .and("active = true")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// SELECT *
  /// FROM users
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::multiline();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// The `from` clause
  pub fn from(mut self, tables: &str) -> Self {
    push_unique(&mut self._from, tables.trim().to_owned());
    self
  }

  /// The `group by` clause
  pub fn group_by(mut self, column: &str) -> Self {
    push_unique(&mut self._group_by, column.trim().to_owned());
    self
  }

  /// The `having` clause
  pub fn having(mut self, condition: &str) -> Self {
    push_unique(&mut self._having, condition.trim().to_owned());
    self
  }

  /// The `cross join` clause
  pub fn cross_join(mut self, table: &str) -> Self {
    let table = table.trim();
    let table = format!("CROSS JOIN {table}");
    push_unique(&mut self._join, table);
    self
  }

  /// The `inner join` clause
  pub fn inner_join(mut self, table: &str) -> Self {
    let table = table.trim();
    let table = format!("INNER JOIN {table}");
    push_unique(&mut self._join, table);
    self
  }

  /// The `left join` clause
  pub fn left_join(mut self, table: &str) -> Self {
    let table = table.trim();
    let table = format!("LEFT JOIN {table}");
    push_unique(&mut self._join, table);
    self
  }

  /// The `right join` clause
  pub fn right_join(mut self, table: &str) -> Self {
    let table = table.trim();
    let table = format!("RIGHT JOIN {table}");
    push_unique(&mut self._join, table);
    self
  }

  /// Create Select's instance
  pub fn new() -> Self {
    Self::default()
  }

  /// The `order by` clause
  pub fn order_by(mut self, column: &str) -> Self {
    push_unique(&mut self._order_by, column.trim().to_owned());
    self
  }

  /// Prints the current state of the Select into console output similar to debug method,
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
  /// use sql_query_builder as sql;
  ///
  /// let raw_query = "select * from users u inner join address addr on u.login = addr.owner_login";
  /// let select_query = sql::Select::new()
  ///   .raw(raw_query)
  ///   .where_clause("u.login = foo")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// select * from users u inner join address addr on u.login = addr.owner_login
  /// WHERE u.login = foo
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
  /// use sql_query_builder as sql;
  ///
  /// let raw_join = "inner join address addr on u.login = addr.owner_login";
  /// let select_query = sql::Select::new()
  ///   .select("*")
  ///   .from("users u")
  ///   .raw_after(sql::SelectClause::From, raw_join)
  ///   .where_clause("u.login = foo")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// SELECT *
  /// FROM users u
  /// inner join address addr on u.login = addr.owner_login
  /// WHERE u.login = foo
  /// ```
  pub fn raw_after(mut self, clause: SelectClause, raw_sql: &str) -> Self {
    self._raw_after.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// Adds a raw SQL query before a specified clause.
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let raw_query = "from users u inner join address addr on u.login = addr.owner_login";
  /// let select_query = sql::Select::new()
  ///   .select("*")
  ///   .raw_before(sql::SelectClause::Where, raw_query)
  ///   .where_clause("u.login = foo")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// SELECT *
  /// from users u inner join address addr on u.login = addr.owner_login
  /// WHERE u.login = foo
  /// ```
  pub fn raw_before(mut self, clause: SelectClause, raw_sql: &str) -> Self {
    self._raw_before.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// The `select` clause
  pub fn select(mut self, column: &str) -> Self {
    push_unique(&mut self._select, column.trim().to_owned());
    self
  }

  /// The `where` clause
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let select = sql::Select::new()
  ///   .from("users")
  ///   .where_clause("login = $1");
  /// ```
  pub fn where_clause(mut self, condition: &str) -> Self {
    push_unique(&mut self._where, condition.trim().to_owned());
    self
  }
}

#[cfg(any(doc, feature = "postgresql", feature = "sqlite"))]
impl Select {
  /// The `except` clause, this method can be used enabling a feature flag
  pub fn except(mut self, select: Self) -> Self {
    self._except.push(select);
    self
  }

  /// The `intersect` clause, this method can be used enabling a feature flag
  pub fn intersect(mut self, select: Self) -> Self {
    self._intersect.push(select);
    self
  }

  /// The `limit` clause, this method overrides the previous value, this method can be used enabling a feature flag
  ///
  /// # Example
  ///
  /// ```ts
  /// use sql_query_builder as sql;
  ///
  /// let select = sql::Select::new()
  ///   .limit("123");
  ///
  /// let select = sql::Select::new()
  ///   .limit("1000")
  ///   .limit("123");
  /// ```
  pub fn limit(mut self, num: &str) -> Self {
    self._limit = num.trim().to_owned();
    self
  }

  /// The `offset` clause, this method overrides the previous value, this method can be used enabling a feature flag
  ///
  /// # Example
  ///
  /// ```ts
  /// use sql_query_builder as sql;
  ///
  /// let select = sql::Select::new()
  ///   .offset("1500");
  ///
  /// let select = sql::Select::new()
  ///   .offset("1000")
  ///   .offset("1500");
  /// ```
  pub fn offset(mut self, num: &str) -> Self {
    self._offset = num.trim().to_owned();
    self
  }

  /// The `union` clause, this method can be used enabling a feature flag
  pub fn union(mut self, select: Self) -> Self {
    self._union.push(select);
    self
  }

  /// The `with` clause, this method can be used enabling a feature flag
  ///
  /// # Example
  ///
  /// ```ts
  /// use sql_query_builder as sql;
  ///
  /// let logins = sql::Select::new().select("login").from("users").where_clause("id in ($1)");
  /// let select = sql::Select::new()
  ///   .with("logins", logins)
  ///   .select("name, price")
  ///   .from("orders")
  ///   .where_clause("owner_login in (select * from logins)")
  ///   .debug();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// WITH logins AS (
  ///   SELECT login
  ///   FROM users
  ///   WHERE id in ($1)
  /// )
  /// SELECT name, price
  /// FROM orders
  /// WHERE owner_login in (select * from active_users)
  /// ```
  pub fn with(mut self, name: &str, query: impl WithQuery + 'static) -> Self {
    self._with.push((name.trim().to_owned(), std::sync::Arc::new(query)));
    self
  }
}

impl std::fmt::Display for Select {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl std::fmt::Debug for Select {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let fmts = fmt::multiline();
    write!(f, "{}", fmt::format(self.concat(&fmts), &fmts))
  }
}
