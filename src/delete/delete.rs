use crate::{
  behavior::TransactionQuery,
  concat::Concat,
  fmt,
  structure::{Delete, DeleteClause, LogicalOperator},
  utils::push_unique,
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
  /// # assert_eq!(expected, query);
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
  /// let query = sql::Delete::new()
  ///   .delete_from("users")
  ///   .where_clause("login = 'foo'")
  ///   .where_clause("name = 'Foo'")
  ///   .debug()
  ///   .as_string();
  /// ```
  ///
  /// Prints to the standard output
  ///
  /// ```sql
  /// -- ------------------------------------------------------------------------------
  /// DELETE FROM users
  /// WHERE
  ///   login = 'foo'
  ///   AND name = 'Foo'
  /// -- ------------------------------------------------------------------------------
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::multiline();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// The `delete` and `from` clauses. This method overrides the previous value
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let delete = sql::Delete::new()
  ///   .delete_from("orders");
  /// #
  /// # let expected = "DELETE FROM orders";
  /// # assert_eq!(expected, delete.to_string());
  ///
  /// let delete = sql::Delete::new()
  ///   .delete_from("addresses")
  ///   .delete_from("orders");
  ///
  /// # let expected = "DELETE FROM orders";
  /// # assert_eq!(expected, delete.to_string());
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// DELETE FROM orders
  /// ```
  pub fn delete_from(mut self, table: &str) -> Self {
    self._delete_from = table.trim().to_string();
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
  /// let query = sql::Delete::new()
  ///   .raw(raw_query)
  ///   .where_clause("login = 'foo'")
  ///   .as_string();
  ///
  /// # let expected = "delete from users WHERE login = 'foo'";
  /// # assert_eq!(expected, query);
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
  /// let query = sql::Delete::new()
  ///   .delete_from("users")
  ///   .raw_after(sql::DeleteClause::DeleteFrom, raw)
  ///   .as_string();
  ///
  /// # let expected = "DELETE FROM users where name = 'Foo'";
  /// # assert_eq!(expected, query);
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
  /// let query = sql::Delete::new()
  ///   .raw_before(sql::DeleteClause::Where, raw)
  ///   .where_clause("name = 'Bar'")
  ///   .as_string();
  ///
  /// # let expected = "delete from users WHERE name = 'Bar'";
  /// # assert_eq!(expected, query);
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

  /// The method will concatenate multiples calls using the `and` operator. This method is un alias of `where_clause`.
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::Delete::new()
  ///   .where_clause("login = $1")
  ///   .where_and("product_id = $2")
  ///   .where_and("created_at >= current_date")
  ///   .as_string();
  ///
  /// # let expected = "WHERE login = $1 AND product_id = $2 AND created_at >= current_date";
  /// # assert_eq!(expected, query);
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

  /// The `where` clause, this method will concatenate multiples calls using the `and` operator.
  /// If you intended to use the `or` operator you should use the [where_or](Delete::where_or) method
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::Delete::new()
  ///   .where_clause("login = $1")
  ///   .where_clause("status = 'deactivated'")
  ///   .as_string();
  ///
  /// # let expected = "WHERE login = $1 AND status = 'deactivated'";
  /// # assert_eq!(expected, query);
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
  /// let query = sql::Delete::new()
  ///   .where_clause("login = 'foo'")
  ///   .where_or("login = 'bar'")
  ///   .as_string();
  ///
  /// # let expected = "WHERE login = 'foo' OR login = 'bar'";
  /// # assert_eq!(expected, query);
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

#[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
use crate::behavior::WithQuery;

#[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
impl WithQuery for Delete {}

#[cfg(any(doc, feature = "postgresql", feature = "sqlite", feature = "mysql"))]
#[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
#[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
impl Delete {
  /// The `with` clause
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let deactivated_users = sql::Select::new()
  ///   .select("id")
  ///   .from("users")
  ///   .where_clause("ative = false");
  ///
  /// let delete = sql::Delete::new()
  ///   .with("deactivated_users", deactivated_users)
  ///   .delete_from("users")
  ///   .where_clause("id in (select * from deactivated_users)")
  ///   .debug();
  ///
  /// # let expected = "\
  /// #   WITH deactivated_users AS (\
  /// #     SELECT id \
  /// #     FROM users \
  /// #     WHERE ative = false\
  /// #   ) \
  /// #   DELETE FROM users \
  /// #   WHERE id in (select * from deactivated_users)\
  /// # ";
  /// # assert_eq!(expected, delete.to_string());
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
  #[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
  pub fn with(mut self, name: &str, query: impl WithQuery + 'static + Send + Sync) -> Self {
    self._with.push((name.trim().to_string(), std::sync::Arc::new(query)));
    self
  }
}

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
  /// # assert_eq!(expected, delete.to_string());
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
}

#[cfg(any(doc, feature = "sqlite", feature = "mysql"))]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
#[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
impl Delete {
  /// The `order by` clause.
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(any(feature = "sqlite", feature = "mysql"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let delete = sql::Delete::new()
  ///   .order_by("created_at asc");
  ///
  /// # let expected = "ORDER BY created_at asc";
  /// # assert_eq!(expected, delete.as_string());
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// ORDER BY created_at asc
  /// ```
  ///
  /// Note: For crate feature `sqlite` this clause is behind a flag at SQLite, [more info](https://sqlite.org/lang_delete.html#optional_limit_and_order_by_clauses).
  pub fn order_by(mut self, column: &str) -> Self {
    push_unique(&mut self._order_by, column.trim().to_string());
    self
  }
}

#[cfg(any(doc, feature = "mysql"))]
#[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
impl Delete {
  /// The `delete` clause. MySQL allow single and multi-table deletes
  ///
  /// # Single-table delete
  ///
  /// ```
  /// # #[cfg(feature = "mysql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let delete = sql::Delete::new()
  ///   .delete("low_priority")
  ///   .from("t1")
  ///   .where_clause("t1.id = '123'");
  ///
  /// # let expected = "DELETE low_priority FROM t1 WHERE t1.id = '123'";
  /// # assert_eq!(expected, delete.to_string());
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// DELETE low_priority FROM t1 WHERE t1.id = '123'
  /// ```
  ///
  /// If the delete clause has no argument you can use the [delete_from](Delete::delete_from) method.
  ///
  /// ```
  /// # #[cfg(feature = "mysql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let delete = sql::Delete::new()
  ///   .delete_from("t1")
  ///   .where_clause("t1.id = '123'");
  ///
  /// # let expected = "DELETE FROM t1 WHERE t1.id = '123'";
  /// # assert_eq!(expected, delete.to_string());
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// DELETE FROM t1 WHERE t1.id = '123'
  /// ```
  ///
  /// # Multi-table deletes
  ///
  /// ```
  /// # #[cfg(feature = "mysql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let delete = sql::Delete::new()
  ///   .delete("t1")
  ///   .delete("t2")
  ///   .from("t1")
  ///   .inner_join("t2")
  ///   .inner_join("t3")
  ///   .where_clause("t1.id = t2.id")
  ///   .where_and("t2.id = t3.id");
  ///
  /// # let expected = "\
  /// #   DELETE t1, t2 \
  /// #   FROM t1 \
  /// #   INNER JOIN t2 \
  /// #   INNER JOIN t3 \
  /// #   WHERE \
  /// #     t1.id = t2.id \
  /// #     AND t2.id = t3.id\
  /// # ";
  /// # assert_eq!(expected, delete.to_string());
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// DELETE t1, t2
  /// FROM t1
  /// INNER JOIN t2
  /// INNER JOIN t3
  /// WHERE
  ///   t1.id = t2.id
  ///   AND t2.id = t3.id
  /// ```
  pub fn delete(mut self, table: &str) -> Self {
    push_unique(&mut self._delete, table.trim().to_string());
    self
  }

  /// The `from` clause
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(feature = "mysql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::Delete::new()
  ///   .from("users")
  ///   .as_string();
  ///
  /// # let expected = "FROM users";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// FROM users
  /// ```
  pub fn from(mut self, table: &str) -> Self {
    push_unique(&mut self._from, table.trim().to_string());
    self
  }

  /// The `cross join` clause
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(feature = "mysql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::Delete::new()
  ///   .cross_join("addresses")
  ///   .as_string();
  ///
  /// # let expected = "CROSS JOIN addresses";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// CROSS JOIN addresses
  /// ```
  pub fn cross_join(mut self, table: &str) -> Self {
    let table = table.trim();
    if table.is_empty() == false {
      let join = format!("CROSS JOIN {table}");
      push_unique(&mut self._join, join);
    }
    self
  }

  /// The `inner join` clause
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(feature = "mysql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::Delete::new()
  ///   .inner_join("addresses on addresses.user_login = users.login")
  ///   .as_string();
  ///
  /// # let expected = "INNER JOIN addresses on addresses.user_login = users.login";
  /// # assert_eq!(query, expected);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// INNER JOIN addresses on addresses.user_login = users.login
  /// ```
  pub fn inner_join(mut self, table: &str) -> Self {
    let table = table.trim();
    if table.is_empty() == false {
      let join = format!("INNER JOIN {table}");
      push_unique(&mut self._join, join);
    }
    self
  }

  /// The `left join` clause
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(feature = "mysql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::Delete::new()
  ///   .left_join("addresses on addresses.user_login = users.login")
  ///   .as_string();
  ///
  /// # let expected = "LEFT JOIN addresses on addresses.user_login = users.login";
  /// # assert_eq!(query, expected);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// LEFT JOIN addresses on addresses.user_login = users.login
  /// ```
  pub fn left_join(mut self, table: &str) -> Self {
    let table = table.trim();
    if table.is_empty() == false {
      let join = format!("LEFT JOIN {table}");
      push_unique(&mut self._join, join);
    }
    self
  }

  /// The `right join` clause
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(feature = "mysql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::Delete::new()
  ///   .right_join("addresses on addresses.user_login = users.login")
  ///   .as_string();
  ///
  /// # let expected = "RIGHT JOIN addresses on addresses.user_login = users.login";
  /// # assert_eq!(query, expected);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// RIGHT JOIN addresses on addresses.user_login = users.login
  /// ```
  pub fn right_join(mut self, table: &str) -> Self {
    let table = table.trim();
    if table.is_empty() == false {
      let join = format!("RIGHT JOIN {table}");
      push_unique(&mut self._join, join);
    }
    self
  }

  /// The `limit` clause, this method overrides the previous value
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(feature = "mysql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let delete = sql::Delete::new()
  ///   .limit("123");
  ///
  /// let delete = sql::Delete::new()
  ///   .limit("1000")
  ///   .limit("123");
  ///
  /// # let expected = "LIMIT 123";
  /// # assert_eq!(expected, delete.as_string());
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// LIMIT 123
  /// ```
  pub fn limit(mut self, num: &str) -> Self {
    self._limit = num.trim().to_string();
    self
  }

  /// The `partition` clause
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(feature = "mysql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::Delete::new()
  ///   .delete_from("employees")
  ///   .partition("p1")
  ///   .to_string();
  ///
  /// # let expected = "DELETE FROM employees PARTITION (p1)";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// DELETE FROM employees PARTITION (p1)
  /// ```
  pub fn partition(mut self, name: &str) -> Self {
    push_unique(&mut self._partition, name.trim().to_string());
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
