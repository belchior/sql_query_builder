use crate::{
  behavior::{push_unique, Concat, TransactionQuery, WithQuery},
  fmt,
  structure::{LogicalOperator, Select, SelectClause},
};

impl TransactionQuery for Select {}

impl WithQuery for Select {}

impl Select {
  /// Gets the current state of the [Select] and returns it as string
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let select_query = sql::Select::new()
  ///   .select("id")
  ///   .from("users")
  ///   .where_clause("login = 'foo'")
  ///   .as_string();
  ///
  /// # let expected = "SELECT id FROM users WHERE login = 'foo'";
  /// # assert_eq!(select_query, expected);
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

  /// Prints the current state of the [Select] to the standard output in a more ease to read version.
  /// This method is useful to debug complex queries or just print the generated SQL while you type
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let select = sql::Select::new()
  ///   .select("*")
  ///   .from("users")
  ///   .where_clause("login = foo")
  ///   .where_clause("active = true")
  ///   .debug();
  /// ```
  ///
  /// Prints to the standard output
  ///
  /// ```sql
  /// -- ------------------------------------------------------------------------------
  /// SELECT *
  /// FROM users
  /// WHERE login = foo AND active = true
  /// -- ------------------------------------------------------------------------------
  /// ```
  ///
  /// You can debug different parts of the select putting it in another position
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let select_query = sql::Select::new()
  ///   .select("*")
  ///   .from("users")
  ///   .debug()
  ///   .where_clause("login = foo")
  ///   .where_clause("active = true")
  ///   .as_string();
  /// ```
  ///
  /// Prints to the standard output
  ///
  /// ```sql
  /// -- ------------------------------------------------------------------------------
  /// SELECT *
  /// FROM users
  /// -- ------------------------------------------------------------------------------
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::multiline();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// The `from` clause
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let select = sql::Select::new()
  ///   .from("users");
  ///
  /// # let expected = "FROM users";
  /// # assert_eq!(select.as_string(), expected);
  /// ```
  pub fn from(mut self, tables: &str) -> Self {
    push_unique(&mut self._from, tables.trim().to_string());
    self
  }

  /// The `group by` clause
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let select = sql::Select::new()
  ///   .group_by("id");
  ///
  /// # let expected = "GROUP BY id";
  /// # assert_eq!(select.as_string(), expected);
  /// ```
  pub fn group_by(mut self, column: &str) -> Self {
    push_unique(&mut self._group_by, column.trim().to_string());
    self
  }

  /// The `having` clause
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let select_query = sql::Select::new()
  ///   .group_by("status")
  ///   .having("status != 'disabled'")
  ///   .as_string();
  ///
  /// # let expected = "GROUP BY status HAVING status != 'disabled'";
  /// # assert_eq!(select_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// GROUP BY status HAVING status != 'disabled'
  /// ```
  pub fn having(mut self, condition: &str) -> Self {
    push_unique(&mut self._having, condition.trim().to_string());
    self
  }

  /// The `cross join` clause
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let select_query = sql::Select::new()
  ///   .from("users")
  ///   .cross_join("addresses")
  ///   .as_string();
  ///
  /// # let expected = "FROM users CROSS JOIN addresses";
  /// # assert_eq!(select_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// FROM users CROSS JOIN addresses
  /// ```
  pub fn cross_join(mut self, table: &str) -> Self {
    let table = table.trim();
    let table = format!("CROSS JOIN {table}");
    push_unique(&mut self._join, table);
    self
  }

  /// The `inner join` clause
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let select_query = sql::Select::new()
  ///   .from("users")
  ///   .inner_join("addresses on addresses.user_login = users.login")
  ///   .as_string();
  ///
  /// # let expected = "FROM users INNER JOIN addresses on addresses.user_login = users.login";
  /// # assert_eq!(select_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// FROM users INNER JOIN addresses on addresses.user_login = users.login
  /// ```
  pub fn inner_join(mut self, table: &str) -> Self {
    let table = table.trim();
    let table = format!("INNER JOIN {table}");
    push_unique(&mut self._join, table);
    self
  }

  /// The `left join` clause
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let select_query = sql::Select::new()
  ///   .from("users")
  ///   .left_join("addresses on addresses.user_login = users.login")
  ///   .as_string();
  ///
  /// # let expected = "FROM users LEFT JOIN addresses on addresses.user_login = users.login";
  /// # assert_eq!(select_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// FROM users LEFT JOIN addresses on addresses.user_login = users.login
  /// ```
  pub fn left_join(mut self, table: &str) -> Self {
    let table = table.trim();
    let table = format!("LEFT JOIN {table}");
    push_unique(&mut self._join, table);
    self
  }

  /// The `right join` clause
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let select_query = sql::Select::new()
  ///   .from("users")
  ///   .right_join("addresses on addresses.user_login = users.login")
  ///   .as_string();
  ///
  /// # let expected = "FROM users RIGHT JOIN addresses on addresses.user_login = users.login";
  /// # assert_eq!(select_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// FROM users RIGHT JOIN addresses on addresses.user_login = users.login
  /// ```
  pub fn right_join(mut self, table: &str) -> Self {
    let table = table.trim();
    let table = format!("RIGHT JOIN {table}");
    push_unique(&mut self._join, table);
    self
  }

  /// Creates instance of the Select command
  pub fn new() -> Self {
    Self::default()
  }

  /// The `order by` clause
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let select = sql::Select::new()
  ///   .select("name, login")
  ///   .order_by("login asc");
  ///
  /// # let expected = "SELECT name, login ORDER BY login asc";
  /// # assert_eq!(select.as_string(), expected);
  /// ```
  pub fn order_by(mut self, column: &str) -> Self {
    push_unique(&mut self._order_by, column.trim().to_string());
    self
  }

  /// Prints the current state of the [Select] to the standard output similar to debug method,
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
  /// let raw_query = "select * from users";
  ///
  /// let select_query = sql::Select::new()
  ///   .raw(raw_query)
  ///   .where_clause("users.login = 'foo'")
  ///   .as_string();
  ///
  /// # let expected = "select * from users WHERE users.login = 'foo'";
  /// # assert_eq!(select_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// select * from users WHERE users.login = 'foo'
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
  /// let raw_join = "inner join addresses addr on u.login = addr.owner_login";
  ///
  /// let select_query = sql::Select::new()
  ///   .from("users u")
  ///   .raw_after(sql::SelectClause::From, raw_join)
  ///   .where_clause("u.login = foo")
  ///   .as_string();
  ///
  /// # let expected = "\
  /// #   FROM users u \
  /// #   inner join addresses addr on u.login = addr.owner_login \
  /// #   WHERE u.login = foo\
  /// # ";
  /// # assert_eq!(select_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// FROM users u
  /// inner join addresses addr on u.login = addr.owner_login
  /// WHERE u.login = foo
  /// ```
  pub fn raw_after(mut self, clause: SelectClause, raw_sql: &str) -> Self {
    self._raw_after.push((clause, raw_sql.trim().to_string()));
    self
  }

  /// Adds a raw SQL query before a specified clause.
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let raw_query = "from users";
  ///
  /// let select_query = sql::Select::new()
  ///   .raw_before(sql::SelectClause::Where, raw_query)
  ///   .where_clause("users.login = 'foo'")
  ///   .as_string();
  ///
  /// # let expected = "\
  /// #   from users \
  /// #   WHERE users.login = 'foo'\
  /// # ";
  /// # assert_eq!(select_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// from users
  /// WHERE users.login = 'foo'
  /// ```
  pub fn raw_before(mut self, clause: SelectClause, raw_sql: &str) -> Self {
    self._raw_before.push((clause, raw_sql.trim().to_string()));
    self
  }

  /// The `select` clause
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let select = sql::Select::new()
  ///   .select("count(id)");
  ///
  /// # let expected = "SELECT count(id)";
  /// # assert_eq!(select.as_string(), expected);
  /// ```
  pub fn select(mut self, column: &str) -> Self {
    push_unique(&mut self._select, column.trim().to_string());
    self
  }

  /// This method is un alias of `where_clause`. The `where_and` will concatenate mulltiples calls using the `and` operator.
  /// The intention is to enable more idiomatic concatenation of conditions.
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let select_query = sql::Select::new()
  ///   .from("carts")
  ///   .where_clause("login = $1")
  ///   .where_and("session_id = $2")
  ///   .where_and("created_at >= current_date")
  ///   .as_string();
  ///
  /// # let expected = "FROM carts WHERE login = $1 AND session_id = $2 AND created_at >= current_date";
  /// # assert_eq!(select_query, expected);
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// FROM carts
  /// WHERE
  ///   login = $1
  ///   AND session_id = $2
  ///   AND created_at >= current_date
  /// ```
  pub fn where_and(self, condition: &str) -> Self {
    self.where_clause(condition)
  }

  /// The `where` clause, this method will concatenate mulltiples calls using the `and` operator.
  /// If you intended to use the `or` operator you should use the [where_or](Select::where_or) method
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let select_query = sql::Select::new()
  ///   .where_clause("login = $1")
  ///   .where_clause("status = 'active'")
  ///   .as_string();
  ///
  /// # let expected = "WHERE login = $1 AND status = 'active'";
  /// # assert_eq!(select_query, expected);
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// WHERE
  ///   login = $1
  ///   AND status = 'active'
  /// ```
  pub fn where_clause(mut self, condition: &str) -> Self {
    push_unique(&mut self._where, (LogicalOperator::And, condition.trim().to_string()));
    self
  }

  /// The `where` clause that concatenate multiples calls using the OR operator.
  /// If you intended to use the `and` operator you should use the [where_clause](Select::where_clause) method
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let select_query = sql::Select::new()
  ///   .where_clause("login = 'foo'")
  ///   .where_or("login = 'bar'")
  ///   .as_string();
  ///
  /// # let expected = "WHERE login = 'foo' OR login = 'bar'";
  /// # assert_eq!(select_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// WHERE
  ///   login = 'foo'
  ///   OR login = 'bar'
  /// ```
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let select_query = sql::Select::new()
  ///   .where_clause("login = 'foo'")
  ///   .where_or("login = 'bar'")
  ///   .where_clause("login = 'joe'")
  ///   .as_string();
  ///
  /// # let expected = "\
  /// #   WHERE \
  /// #     login = 'foo' \
  /// #     OR login = 'bar' \
  /// #     AND login = 'joe'\
  /// # ";
  /// # assert_eq!(select_query, expected);
  /// ```
  /// Output
  ///
  /// ```sql
  /// WHERE
  ///   login = 'foo'
  ///   OR login = 'bar'
  ///   AND login = 'joe'
  /// ```
  ///
  /// # Example
  ///
  /// If the `where_or` was the first clause then the operator will be ignored
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let select_query = sql::Select::new()
  ///   .where_or("login = 'joe'")
  ///   .where_clause("login = 'foo'")
  ///   .as_string();
  ///
  /// # let expected = "WHERE login = 'joe' AND login = 'foo'";
  /// # assert_eq!(select_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// WHERE
  ///   login = 'joe'
  ///   AND login = 'foo'
  /// ```
  pub fn where_or(mut self, condition: &str) -> Self {
    push_unique(&mut self._where, (LogicalOperator::Or, condition.trim().to_string()));
    self
  }

  /// The `window` clause
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let select_query = sql::Select::new()
  ///   .window("win as (partition by department)")
  ///   .as_string();
  ///
  /// # let expected = "WINDOW win as (partition by department)";
  /// # assert_eq!(select_query, expected);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// WINDOW win as (partition by department)
  /// ```
  pub fn window(mut self, name: &str) -> Self {
    push_unique(&mut self._window, name.trim().to_string());
    self
  }
}

#[cfg(any(doc, feature = "postgresql", feature = "sqlite"))]
#[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
impl Select {
  /// The `except` clause
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let select_users = sql::Select::new()
  ///   .select("login")
  ///   .from("users");
  ///
  /// let select_inactives = sql::Select::new()
  ///   .select("login")
  ///   .from("users")
  ///   .where_clause("status = 'inactive'");
  ///
  /// let select_query = select_users.except(select_inactives).as_string();
  ///
  /// # let expected = "\
  /// #   (SELECT login FROM users) \
  /// #   EXCEPT \
  /// #   (SELECT login FROM users WHERE status = 'inactive')\
  /// # ";
  /// # assert_eq!(select_query, expected);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// (SELECT login FROM users)
  /// EXCEPT
  /// (SELECT login FROM users WHERE status = 'inactive')
  /// ```
  pub fn except(mut self, select: Self) -> Self {
    self._except.push(select);
    self
  }

  /// The `intersect` clause
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let select_users = sql::Select::new()
  ///   .select("login")
  ///   .from("users");
  ///
  /// let select_inactives = sql::Select::new()
  ///   .select("login")
  ///   .from("users")
  ///   .where_clause("status = 'inactive'");
  ///
  /// let select_query = select_users.intersect(select_inactives).as_string();
  ///
  /// # let expected = "\
  /// #   (SELECT login FROM users) \
  /// #   INTERSECT \
  /// #   (SELECT login FROM users WHERE status = 'inactive')\
  /// # ";
  /// # assert_eq!(select_query, expected);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// (SELECT login FROM users)
  /// INTERSECT
  /// (SELECT login FROM users WHERE status = 'inactive')
  /// ```
  pub fn intersect(mut self, select: Self) -> Self {
    self._intersect.push(select);
    self
  }

  /// The `limit` clause, this method overrides the previous value
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let select = sql::Select::new()
  ///   .limit("123");
  ///
  /// let select = sql::Select::new()
  ///   .limit("1000")
  ///   .limit("123");
  ///
  /// # let expected = "LIMIT 123";
  /// # assert_eq!(select.as_string(), expected);
  /// # }
  /// ```
  pub fn limit(mut self, num: &str) -> Self {
    self._limit = num.trim().to_string();
    self
  }

  /// The `offset` clause, this method overrides the previous value
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let select = sql::Select::new()
  ///   .offset("1500");
  ///
  /// let select = sql::Select::new()
  ///   .offset("1000")
  ///   .offset("1500");
  ///
  /// # let expected = "OFFSET 1500";
  /// # assert_eq!(select.as_string(), expected);
  /// # }
  /// ```
  pub fn offset(mut self, num: &str) -> Self {
    self._offset = num.trim().to_string();
    self
  }

  /// The `union` clause
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let select_users = sql::Select::new()
  ///   .select("login")
  ///   .from("users");
  ///
  /// let select_inactives = sql::Select::new()
  ///   .select("login")
  ///   .from("users")
  ///   .where_clause("status = 'inactive'");
  ///
  /// let select_query = select_users.union(select_inactives).as_string();
  ///
  /// # let expected = "\
  /// #   (SELECT login FROM users) \
  /// #   UNION \
  /// #   (SELECT login FROM users WHERE status = 'inactive')\
  /// # ";
  /// # assert_eq!(select_query, expected);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// (SELECT login FROM users)
  /// UNION
  /// (SELECT login FROM users WHERE status = 'inactive')
  /// ```
  pub fn union(mut self, select: Self) -> Self {
    self._union.push(select);
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
  /// let logins = sql::Select::new()
  ///   .select("login")
  ///   .from("users")
  ///   .where_clause("id in ($1)");
  ///
  /// let select = sql::Select::new()
  ///   .with("logins", logins)
  ///   .select("name, price")
  ///   .from("orders")
  ///   .where_clause("owner_login in (select * from logins)")
  ///   .debug();
  ///
  /// # let expected = "\
  /// #   WITH logins AS (\
  /// #     SELECT login \
  /// #     FROM users \
  /// #     WHERE id in ($1)\
  /// #   ) \
  /// #   SELECT name, price \
  /// #   FROM orders \
  /// #   WHERE owner_login in (select * from logins)\
  /// # ";
  /// # assert_eq!(select.as_string(), expected);
  /// # }
  /// ```
  ///
  /// Prints to the standard output
  ///
  /// ```sql
  /// -- ------------------------------------------------------------------------------
  /// WITH
  /// logins AS (
  ///   SELECT login
  ///   FROM users
  ///   WHERE id in ($1)
  /// )
  /// SELECT name, price
  /// FROM orders
  /// WHERE owner_login in (select * from logins)
  /// -- ------------------------------------------------------------------------------
  /// ```
  pub fn with(mut self, name: &str, query: impl WithQuery + 'static) -> Self {
    self._with.push((name.trim().to_string(), std::sync::Arc::new(query)));
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
