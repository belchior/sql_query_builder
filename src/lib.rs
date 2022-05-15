#![doc = include_str!("../README.md")]

mod concat;
mod fmt;
mod structure;

pub use structure::{Clause, SelectBuilder};

impl<'a> SelectBuilder<'a> {
  /// The same as `where_clause` method, useful to write more ideomatic SQL query
  /// ```
  /// use sql_query_builder::SelectBuilder;
  ///
  /// let select = SelectBuilder::new()
  ///   .where_clause("login = foo")
  ///   .and("active = true");
  /// ```
  pub fn and(mut self, clause: &'a str) -> Self {
    self = self.where_clause(clause);
    self
  }

  /// Gets the current state of the select returns as string
  pub fn as_string(&self) -> String {
    let fmts = fmt::Formatter::one_line();
    self.concat(&fmts)
  }

  /// Prints the current state of the select into console output in a more ease to read version.
  /// This method is useful to debug complex queries or just to print the generated SQL while you type
  /// ```
  /// use sql_query_builder::SelectBuilder;
  ///
  /// let select = SelectBuilder::new()
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
  /// ```
  /// use sql_query_builder::SelectBuilder;
  ///
  /// let select = SelectBuilder::new()
  ///   .select("*")
  ///   .from("users")
  ///   .debug()
  ///   .where_clause("login = foo")
  ///   .and("active = true");
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// SELECT *
  /// FROM users
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::Formatter::multi_line();
    println!("{}", fmt::colorize(self.concat(&fmts)));
    self
  }

  /// The except clause
  pub fn except(mut self, select: Self) -> Self {
    self._except.push(select);
    self
  }

  /// The from clause
  pub fn from(mut self, tables: &'a str) -> Self {
    self._from.push(tables.to_owned());
    self
  }

  /// The group by clause
  pub fn group_by(mut self, column: &'a str) -> Self {
    self._group_by.push(column.to_owned());
    self
  }

  /// The having clause
  pub fn having(mut self, condition: &'a str) -> Self {
    self._having.push(condition.to_owned());
    self
  }

  /// The cross join clause
  pub fn cross_join(mut self, table: &'a str) -> Self {
    self._join.push(format!("CROSS JOIN {table}"));
    self
  }

  /// The inner join clause
  pub fn inner_join(mut self, condition: &'a str) -> Self {
    self._join.push(format!("INNER JOIN {condition}"));
    self
  }

  /// The left join clause
  pub fn left_join(mut self, condition: &'a str) -> Self {
    self._join.push(format!("LEFT JOIN {condition}"));
    self
  }

  /// The right join clause
  pub fn right_join(mut self, condition: &'a str) -> Self {
    self._join.push(format!("RIGHT JOIN {condition}"));
    self
  }

  /// The intersect clause
  pub fn intersect(mut self, select: Self) -> Self {
    self._intersect.push(select);
    self
  }

  /// The limit clause. This method overrides the previous value, the two select produce the same SQL query
  ///
  /// ```
  /// use sql_query_builder::SelectBuilder;
  ///
  /// let select = SelectBuilder::new()
  ///   .limit("123");
  ///
  /// let select = SelectBuilder::new()
  ///   .limit("1000")
  ///   .limit("123");
  /// ```
  pub fn limit(mut self, num: &'a str) -> Self {
    self._limit = num;
    self
  }

  /// Create select builder instance
  pub fn new() -> Self {
    Self::default()
  }

  /// The offset clause. This method overrides the previous value, the two select produce the same SQL query
  ///
  /// ```
  /// use sql_query_builder::SelectBuilder;
  ///
  /// let select = SelectBuilder::new()
  ///   .offset("1500");
  ///
  /// let select = SelectBuilder::new()
  ///   .offset("1000")
  ///   .offset("1500");
  /// ```
  pub fn offset(mut self, num: &'a str) -> Self {
    self._offset = num;
    self
  }

  /// The order by clause
  pub fn order_by(mut self, column: &'a str) -> Self {
    self._order_by.push(column.to_owned());
    self
  }

  /// Prints the current state of the select into console output similar to debug method,
  /// the diference is that this method prints in one line.
  pub fn print(self) -> Self {
    let fmts = fmt::Formatter::one_line();
    println!("{}", fmt::colorize(self.concat(&fmts)));
    self
  }

  /// Adds at the beginning a raw SQL query.
  ///
  /// ```
  /// use sql_query_builder::SelectBuilder;
  ///
  /// let raw_query = "select * from users u inner join address addr on u.login = addr.owner_login";
  /// let select = SelectBuilder::new()
  ///   .raw(raw_query)
  ///   .where_clause("u.login = foo");
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// select * from users u inner join address addr on u.login = addr.owner_login
  /// WHERE u.login = foo
  /// ```
  pub fn raw(mut self, raw_sql: &'a str) -> Self {
    self._raw.push(raw_sql.to_owned());
    self
  }

  /// Adds a raw SQL query after a specified clause.
  ///
  /// ```
  /// use sql_query_builder::{Clause, SelectBuilder};
  ///
  /// let raw_join = "inner join address addr on u.login = addr.owner_login";
  /// let select = SelectBuilder::new()
  ///   .select("*")
  ///   .from("users u")
  ///   .raw_after(Clause::From, raw_join)
  ///   .where_clause("u.login = foo");
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
  pub fn raw_after(mut self, clause: Clause, raw_sql: &'a str) -> Self {
    self._raw_after.push((clause, raw_sql.to_owned()));
    self
  }

  /// Adds a raw SQL query before a specified clause.
  ///
  /// ```
  /// use sql_query_builder::{Clause, SelectBuilder};
  ///
  /// let raw_query = "from users u inner join address addr on u.login = addr.owner_login";
  /// let select = SelectBuilder::new()
  ///   .select("*")
  ///   .raw_before(Clause::Where, raw_query)
  ///   .where_clause("u.login = foo");
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// SELECT *
  /// from users u inner join address addr on u.login = addr.owner_login
  /// WHERE u.login = foo
  /// ```
  pub fn raw_before(mut self, clause: Clause, raw_sql: &'a str) -> Self {
    self._raw_before.push((clause, raw_sql.to_owned()));
    self
  }

  /// The select by clause
  pub fn select(mut self, column: &'a str) -> Self {
    self._select.push(column.to_owned());
    self
  }

  /// The union by clause
  pub fn union(mut self, select: Self) -> Self {
    self._union.push(select);
    self
  }

  /// The where by clause
  pub fn where_clause(mut self, condition: &'a str) -> Self {
    self._where.push(condition.to_owned());
    self
  }

  /// The with by clause
  pub fn with(mut self, name: &'a str, select: Self) -> Self {
    self._with.push((name, select));
    self
  }
}

impl<'a> std::fmt::Display for SelectBuilder<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl<'a> std::fmt::Debug for SelectBuilder<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let fmts = fmt::Formatter::multi_line();
    write!(f, "{}", fmt::colorize(self.concat(&fmts)))
  }
}
