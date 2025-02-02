use crate::{
  behavior::TransactionQuery,
  concat::Concat,
  fmt,
  structure::{Insert, InsertClause, Select},
  utils::push_unique,
};

impl TransactionQuery for Insert {}

impl Insert {
  /// Gets the current state of the [Insert] and returns it as string
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::Insert::new()
  ///   .insert_into("users (login)")
  ///   .values("('foo')")
  ///   .as_string();
  ///
  /// # let expected = "INSERT INTO users (login) VALUES ('foo')";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// INSERT INTO users (login) VALUES ('foo')
  /// ```
  pub fn as_string(&self) -> String {
    let fmts = fmt::one_line();
    self.concat(&fmts)
  }

  /// Prints the current state of the [Insert] to the standard output in a more ease to read version.
  /// This method is useful to debug complex queries or just print the generated SQL while you type
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::Insert::new()
  ///   .insert_into("users (login, name)")
  ///   .values("('foo', 'Foo')")
  ///   .debug()
  ///   .values("('bar', 'Bar')")
  ///   .as_string();
  /// ```
  ///
  /// Prints to the standard output
  ///
  /// ```sql
  /// -- ------------------------------------------------------------------------------
  /// INSERT INTO users (login, name)
  /// VALUES ('foo', 'Foo')
  /// -- ------------------------------------------------------------------------------
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::multiline();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// The `default values` clause
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(not(feature = "mysql"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::Insert::new()
  ///   .insert_into("users")
  ///   .default_values()
  ///   .to_string();
  ///
  /// # let expected = "INSERT INTO users DEFAULT VALUES";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// INSERT INTO users DEFAULT VALUES
  /// ```
  #[cfg(not(feature = "mysql"))]
  pub fn default_values(mut self) -> Self {
    self._default_values = true;
    self._values = vec![];
    self
  }

  /// The `insert into` clause. This method overrides the previous value
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let insert = sql::Insert::new()
  ///   .insert_into("users (login, name)");
  /// #
  /// # let expected = "INSERT INTO users (login, name)";
  /// # assert_eq!(expected, insert.to_string());
  ///
  /// let insert = sql::Insert::new()
  ///   .insert_into("addresses (state, country)")
  ///   .insert_into("users (login, name)");
  ///
  /// # let expected = "INSERT INTO users (login, name)";
  /// # assert_eq!(expected, insert.to_string());
  /// ```
  pub fn insert_into(mut self, table_name: &str) -> Self {
    self._insert_into = table_name.trim().to_string();

    #[cfg(feature = "sqlite")]
    {
      self._insert_or = "".to_string();
      self._replace_into = "".to_string();
    }

    #[cfg(feature = "mysql")]
    {
      self._insert = "".to_string();
      self._into = "".to_string();
      self._partition = vec![];
      self._column = vec![];
    }

    self
  }

  /// Creates instance of the Insert command
  pub fn new() -> Self {
    Self::default()
  }

  /// The `overriding` clause. This method overrides the previous value
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::Insert::new()
  ///   .insert_into("users (login)")
  ///   .overriding("user value")
  ///   .as_string();
  ///
  /// # let expected = "INSERT INTO users (login) OVERRIDING user value";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// INSERT INTO users (login) OVERRIDING user value
  /// ```
  #[cfg(not(any(feature = "sqlite", feature = "mysql")))]
  pub fn overriding(mut self, option: &str) -> Self {
    self._overriding = option.trim().to_string();
    self
  }

  /// Prints the current state of the [Insert] to the standard output similar to debug method,
  /// the difference is that this method prints in one line.
  pub fn print(self) -> Self {
    let fmts = fmt::one_line();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// The `select` clause. This method overrides the previous value
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::Insert::new()
  ///   .insert_into("users (login, name)")
  ///   .select(
  ///     sql::Select::new()
  ///       .select("login, name")
  ///       .from("users_bk")
  ///       .where_clause("active = true"),
  ///   )
  ///   .as_string();
  ///
  /// # let expected = "\
  /// #   INSERT INTO users (login, name) \
  /// #   SELECT login, name \
  /// #   FROM users_bk \
  /// #   WHERE active = true\
  /// # ";
  /// # assert_eq!(expected, query);
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
  pub fn select(mut self, select: Select) -> Self {
    self._select = Some(select);
    self
  }

  /// Adds at the beginning a raw SQL query.
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let raw_query = "insert into users (login, name)";
  ///
  /// let query = sql::Insert::new()
  ///   .raw(raw_query)
  ///   .values("('foo', 'Foo')")
  ///   .as_string();
  ///
  /// # let expected = "insert into users (login, name) VALUES ('foo', 'Foo')";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// insert into users (login, name) VALUES ('foo', 'Foo')
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
  /// let raw = "values ('foo', 'Foo')";
  ///
  /// let query = sql::Insert::new()
  ///   .insert_into("users (login, name)")
  ///   .raw_after(sql::InsertClause::InsertInto, raw)
  ///   .as_string();
  ///
  /// # let expected = "INSERT INTO users (login, name) values ('foo', 'Foo')";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// INSERT INTO users (login, name) values ('foo', 'Foo')
  /// ```
  pub fn raw_after(mut self, clause: InsertClause, raw_sql: &str) -> Self {
    self._raw_after.push((clause, raw_sql.trim().to_string()));
    self
  }

  /// Adds a raw SQL query before a specified clause.
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let raw = "insert into users (login, name)";
  ///
  /// let query = sql::Insert::new()
  ///   .raw_before(sql::InsertClause::Values, raw)
  ///   .values("('bar', 'Bar')")
  ///   .as_string();
  ///
  /// # let expected = "insert into users (login, name) VALUES ('bar', 'Bar')";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// insert into users (login, name) VALUES ('bar', 'Bar')
  /// ```
  pub fn raw_before(mut self, clause: InsertClause, raw_sql: &str) -> Self {
    self._raw_before.push((clause, raw_sql.trim().to_string()));
    self
  }

  /// The `values` clause
  ///
  /// # Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::Insert::new()
  ///   .insert_into("users (login, name)")
  ///   .values("('foo', 'Foo')")
  ///   .values("('bar', 'Bar')")
  ///   .as_string();
  ///
  /// # let expected = "INSERT INTO users (login, name) VALUES ('foo', 'Foo'), ('bar', 'Bar')";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// INSERT INTO users (login, name) VALUES ('foo', 'Foo'), ('bar', 'Bar')
  /// ```
  pub fn values(mut self, value: &str) -> Self {
    push_unique(&mut self._values, value.trim().to_string());
    #[cfg(not(feature = "mysql"))]
    {
      self._default_values = false
    }
    self
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
use crate::behavior::WithQuery;

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
impl WithQuery for Insert {}

#[cfg(any(doc, feature = "postgresql", feature = "sqlite"))]
#[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
impl Insert {
  /// The `on conflict` clause. This method overrides the previous value
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::Insert::new()
  ///   .insert_into("users (login)")
  ///   .on_conflict("do nothing")
  ///   .as_string();
  ///
  /// # let expected = "INSERT INTO users (login) ON CONFLICT do nothing";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// INSERT INTO users (login) ON CONFLICT do nothing
  /// ```
  pub fn on_conflict(mut self, conflict: &str) -> Self {
    self._on_conflict = conflict.trim().to_string();
    self
  }

  /// The `returning` clause
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::Insert::new()
  ///   .insert_into("users")
  ///   .returning("id")
  ///   .returning("login")
  ///   .to_string();
  ///
  /// # let expected = "INSERT INTO users RETURNING id, login";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// INSERT INTO users RETURNING id, login
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
  /// let active_users = sql::Select::new()
  ///   .select("*")
  ///   .from("users_bk")
  ///   .where_clause("ative = true");
  ///
  /// let query = sql::Insert::new()
  ///   .with("active_users", active_users)
  ///   .insert_into("users")
  ///   .select(sql::Select::new().select("*").from("active_users"))
  ///   .to_string();
  ///
  /// # let expected = "\
  /// #   WITH active_users AS (\
  /// #     SELECT * \
  /// #     FROM users_bk \
  /// #     WHERE ative = true\
  /// #   ) \
  /// #   INSERT INTO users \
  /// #   SELECT * \
  /// #   FROM active_users\
  /// # ";
  /// # assert_eq!(expected, query);
  /// # }
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
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  pub fn with(mut self, name: &str, query: impl WithQuery + 'static) -> Self {
    self._with.push((name.trim().to_string(), std::sync::Arc::new(query)));
    self
  }
}

#[cfg(any(doc, feature = "sqlite"))]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
impl Insert {
  /// The `insert or <keyword> into` clause
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(feature = "sqlite")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let insert = sql::Insert::new()
  ///   .insert_or("abort into users (login, name)");
  /// #
  /// # let expected = "INSERT OR abort into users (login, name)";
  /// # assert_eq!(expected, insert.to_string());
  ///
  /// let insert = sql::Insert::new()
  ///   .insert_or("fail into addresses (state, country)")
  ///   .insert_or("abort into users (login, name)");
  ///
  /// # let expected = "INSERT OR abort into users (login, name)";
  /// # assert_eq!(expected, insert.to_string());
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// INSERT OR abort into users (login, name)
  /// ```
  pub fn insert_or(mut self, expression: &str) -> Self {
    self._insert_or = expression.trim().to_string();
    self._insert_into = "".to_string();
    self._replace_into = "".to_string();
    self
  }

  /// The `replace into` clause, this method overrides the previous value
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(feature = "sqlite")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let insert = sql::Insert::new()
  ///   .replace_into("users (login, name)");
  /// #
  /// # let expected = "REPLACE INTO users (login, name)";
  /// # assert_eq!(expected, insert.to_string());
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// REPLACE INTO users (login, name)
  /// ```
  pub fn replace_into(mut self, table_name: &str) -> Self {
    self._replace_into = table_name.trim().to_string();
    self._insert_into = "".to_string();
    self._insert_or = "".to_string();
    self
  }
}

#[cfg(any(doc, feature = "mysql"))]
#[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
impl Insert {
  /// Defines the columns of the table used to insert values.
  ///
  /// ### Example
  ///
  ///```
  /// # #[cfg(feature = "mysql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::Insert::new()
  ///   .into("users")
  ///   .column("login")
  ///   .column("name")
  ///   .as_string();
  ///
  /// # let expected = "INTO users (login, name)";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// INTO users (login, name)
  /// ```
  pub fn column(mut self, column_name: &str) -> Self {
    push_unique(&mut self._column, column_name.trim().to_string());
    self._insert_into = "".to_string();
    self
  }

  /// The `insert` clause, used to defined modifiers to change de insert execution
  ///
  /// ### Example
  ///
  ///```
  /// # #[cfg(feature = "mysql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::Insert::new()
  ///   .insert("LOW_PRIORITY")
  ///   .into("users")
  ///   .column("login")
  ///   .as_string();
  ///
  /// # let expected = "INSERT LOW_PRIORITY INTO users (login)";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// INSERT LOW_PRIORITY INTO users (login)
  /// ```
  pub fn insert(mut self, modifier: &str) -> Self {
    self._insert = modifier.trim().to_string();
    self._insert_into = "".to_string();
    self
  }

  /// The `into` clause, defines the name of the table to be used
  ///
  /// ### Example
  ///
  ///```
  /// # #[cfg(feature = "mysql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::Insert::new()
  ///   .into("users")
  ///   .column("login")
  ///   .as_string();
  ///
  /// # let expected = "INTO users (login)";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// INTO users (login)
  /// ```
  pub fn into(mut self, table: &str) -> Self {
    self._into = table.trim().to_string();
    self._insert_into = "".to_string();
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
  /// let query = sql::Insert::new()
  ///   .into("employees")
  ///   .partition("p1")
  ///   .to_string();
  ///
  /// # let expected = "INTO employees PARTITION (p1)";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// INTO employees PARTITION (p1)
  /// ```
  pub fn partition(mut self, name: &str) -> Self {
    push_unique(&mut self._partition, name.trim().to_string());
    self._insert_into = "".to_string();
    self
  }

  /// The `ON DUPLICATE KEY UPDATE` clause
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(feature = "mysql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::Insert::new()
  ///   .insert_into("t1 (a, b, c)")
  ///   .values("(1, 2, 3)")
  ///   .on_duplicate_key_update("c = c+1")
  ///   .as_string();
  ///
  /// # let expected = "\
  /// #   INSERT INTO t1 (a, b, c) \
  /// #   VALUES (1, 2, 3) \
  /// #   ON DUPLICATE KEY UPDATE c = c+1\
  /// # ";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// INSERT INTO t1 (a, b, c)
  /// VALUES (1, 2, 3)
  /// ON DUPLICATE KEY UPDATE c = c+1
  /// ```
  pub fn on_duplicate_key_update(mut self, assignment: &str) -> Self {
    push_unique(&mut self._on_duplicate_key_update, assignment.trim().to_string());
    self
  }

  /// The `set` clause
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(feature = "mysql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let update_query = sql::Insert::new()
  ///   .set("name = 'Bar'")
  ///   .as_string();
  ///
  /// # let expected = "SET name = 'Bar'";
  /// # assert_eq!(expected, update_query);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// SET name = 'Bar'
  /// ```
  pub fn set(mut self, assignment: &str) -> Self {
    push_unique(&mut self._set, assignment.trim().to_string());
    self
  }
}

impl std::fmt::Display for Insert {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl std::fmt::Debug for Insert {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let fmts = fmt::multiline();
    write!(f, "{}", fmt::format(self.concat(&fmts), &fmts))
  }
}
