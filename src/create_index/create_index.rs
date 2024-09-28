use crate::{
  behavior::{push_unique, Concat, TransactionQuery},
  fmt,
  structure::{CreateIndex, CreateIndexParams, LogicalOperator},
};

impl TransactionQuery for CreateIndex {}

impl CreateIndex {
  /// Gets the current state of the [CreateIndex] and returns it as string
  ///
  /// ### Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::CreateIndex::new()
  ///   .create_index("users_name_idx")
  ///   .as_string();
  ///
  /// # let expected = "CREATE INDEX users_name_idx";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// CREATE INDEX users_name_idx
  /// ```
  pub fn as_string(&self) -> String {
    let fmts = fmt::one_line();
    self.concat(&fmts)
  }

  /// Defines the column of the table used to create the index
  ///
  /// ### Example
  ///
  ///```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::CreateIndex::new()
  ///   .on("users")
  ///   .column("login")
  ///   .column("name")
  ///   .as_string();
  ///
  /// # let expected = "ON users (login, name)";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// ON users (login, name)
  /// ```
  pub fn column(mut self, column_name: &str) -> Self {
    push_unique(&mut self._column, column_name.trim().to_string());
    self
  }

  /// Defines a create index parameter, this method overrides the previous value
  ///
  /// ### Example
  ///
  ///```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::CreateIndex::new()
  ///   .create_index("users_name_idx")
  ///   .create_index("orders_product_name_idx")
  ///   .as_string();
  ///
  /// # let expected = "CREATE INDEX orders_product_name_idx";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// CREATE INDEX orders_product_name_idx
  /// ```
  pub fn create_index(mut self, index_name: &str) -> Self {
    self._index_name = index_name.trim().to_string();
    self._create_index = true;
    self
  }

  /// Defines a create index parameter with the `if not exists` modifier, this method overrides the previous value
  ///
  /// ### Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::CreateIndex::new()
  ///   .create_index("users_name_idx")
  ///   .create_index_if_not_exists("orders_product_name_idx")
  ///   .to_string();
  ///
  /// # let expected = "CREATE INDEX IF NOT EXISTS orders_product_name_idx";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// CREATE INDEX IF NOT EXISTS orders_product_name_idx
  /// ```
  pub fn create_index_if_not_exists(mut self, index_name: &str) -> Self {
    self._index_name = index_name.trim().to_string();
    self._create_index = true;
    self._if_not_exists = true;
    self
  }

  /// Prints the current state of the [CreateIndex] to the standard output in a more ease to read version.
  /// This method is useful to debug complex queries or just print the generated SQL while you type
  ///
  /// ### Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::CreateIndex::new()
  ///   .create_index("users_name_idx")
  ///   .on("users")
  ///   .column("name")
  ///   .debug()
  ///   .as_string();
  ///
  /// # let expected = "\
  /// #   CREATE INDEX users_name_idx \
  /// #   ON users (name)\
  /// # ";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Prints to the standard output
  ///
  /// ```sql
  /// -- ------------------------------------------------------------------------------
  /// CREATE INDEX users_name_idx
  /// ON users
  /// (name)
  /// -- ------------------------------------------------------------------------------
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::multiline();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// Creates instance of the [CreateIndex] command
  pub fn new() -> Self {
    Self::default()
  }

  /// Defines the `on table_name` clause, this method overrides the previous value
  ///
  /// ### Example
  ///
  ///```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::CreateIndex::new()
  ///   .on("users")
  ///   .on("orders")
  ///   .as_string();
  ///
  /// # let expected = "ON orders";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// ON orders
  /// ```
  pub fn on(mut self, table_name: &str) -> Self {
    self._on = table_name.trim().to_string();
    self
  }

  /// Prints the current state of the [CreateIndex] to the standard output similar to debug method,
  /// the difference is that this method prints in one line.
  pub fn print(self) -> Self {
    let fmts = fmt::one_line();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// Adds at the beginning a raw SQL query. Is useful to create a more complex create index command.
  ///
  /// ### Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let create_index_query = sql::CreateIndex::new()
  ///   .raw("/* start index command */")
  ///   .create_index("users_name_idx")
  ///   .as_string();
  ///
  /// # let expected = "/* start index command */ CREATE INDEX users_name_idx";
  /// # assert_eq!(expected, create_index_query);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// /* create index command */ CREATE INDEX users_name_idx
  /// ```
  pub fn raw(mut self, raw_sql: &str) -> Self {
    push_unique(&mut self._raw, raw_sql.trim().to_string());
    self
  }

  /// Adds a raw SQL query after a specified parameter.
  ///
  /// The `CreateIndexParams::CreateIndex` works both to `.create_index` and `.create_index_if_not_exists` methods.
  ///
  /// ### Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let raw = "/* after create index */";
  ///
  /// let query = sql::CreateIndex::new()
  ///   .create_index("users_name_idx")
  ///   .raw_after(sql::CreateIndexParams::CreateIndex, raw)
  ///   .on("users")
  ///   .column("name")
  ///   .as_string();
  ///
  /// # let expected = "\
  /// #   CREATE INDEX users_name_idx \
  /// #   /* after create index */ \
  /// #   ON users (name)\
  /// # ";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// CREATE INDEX users_name_idx
  /// /* after create index */
  /// ON users (name)
  /// ```
  pub fn raw_after(mut self, param: CreateIndexParams, raw_sql: &str) -> Self {
    self._raw_after.push((param, raw_sql.trim().to_string()));
    self
  }

  /// Adds a raw SQL query before a specified parameter.
  ///
  /// The `CreateIndexParams::CreateIndex` works both to `.create_index` and `.create_index_if_not_exists` methods.
  ///
  /// ### Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let raw = "/* before create index */";
  ///
  /// let query = sql::CreateIndex::new()
  ///   .raw_before(sql::CreateIndexParams::CreateIndex, raw)
  ///   .create_index("users_name_idx")
  ///   .on("users")
  ///   .column("name")
  ///   .as_string();
  ///
  /// # let expected = "\
  /// #   /* before create index */ \
  /// #   CREATE INDEX users_name_idx \
  /// #   ON users (name)\
  /// # ";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// /* before create index */
  /// CREATE INDEX users_name_idx
  /// ON users (name)
  /// ```
  pub fn raw_before(mut self, param: CreateIndexParams, raw_sql: &str) -> Self {
    self._raw_before.push((param, raw_sql.trim().to_string()));
    self
  }

  /// Defines the `unique` parameter
  ///
  /// ### Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::CreateIndex::new()
  ///   .create_index("users_name_idx")
  ///   .unique()
  ///   .to_string();
  ///
  /// # let expected = "CREATE UNIQUE INDEX users_name_idx";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// CREATE UNIQUE INDEX users_name_idx
  /// ```
  pub fn unique(mut self) -> Self {
    self._unique = true;
    self
  }

  /// The method will concatenate multiples calls using the `and` operator. This method is un alias of `where_clause`.
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let select_query = sql::CreateIndex::new()
  ///   .create_index("users_name_idx")
  ///   .on("users")
  ///   .column("name")
  ///   .where_and("created_at >= $1")
  ///   .as_string();
  ///
  /// # let expected = "\
  /// #   CREATE INDEX users_name_idx \
  /// #   ON users (name) \
  /// #   WHERE created_at >= $1\
  /// # ";
  /// # assert_eq!(select_query, expected);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// CREATE INDEX users_name_idx
  /// ON users (name)
  /// WHERE created_at >= $1
  /// ```
  pub fn where_and(self, condition: &str) -> Self {
    self.where_clause(condition)
  }

  /// The `where` clause, this method will concatenate multiples calls using the `and` operator.
  /// If you intended to use the `or` operator you should use the [where_or](CreateIndex::where_or) method
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let select_query = sql::CreateIndex::new()
  ///   .create_index("users_name_idx")
  ///   .on("users")
  ///   .column("name")
  ///   .where_clause("status = 'active'")
  ///   .as_string();
  ///
  /// # let expected = "\
  /// #   CREATE INDEX users_name_idx \
  /// #   ON users (name) \
  /// #   WHERE status = 'active'\
  /// # ";
  /// # assert_eq!(select_query, expected);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// CREATE INDEX users_name_idx
  /// ON users (name)
  /// WHERE status = 'active'
  /// ```
  pub fn where_clause(mut self, condition: &str) -> Self {
    push_unique(&mut self._where, (LogicalOperator::And, condition.trim().to_string()));
    self
  }

  /// The `where` clause that concatenate multiples calls using the OR operator.
  /// If you intended to use the `and` operator you should use the [where_clause](CreateIndex::where_clause) method
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let select_query = sql::CreateIndex::new()
  ///   .create_index("users_name_idx")
  ///   .on("users")
  ///   .column("name")
  ///   .where_clause("created_at >= $1")
  ///   .where_or("status = 'active'")
  ///   .as_string();
  ///
  /// # let expected = "\
  /// #   CREATE INDEX users_name_idx \
  /// #   ON users (name) \
  /// #   WHERE \
  /// #     created_at >= $1 \
  /// #     OR status = 'active'\
  /// # ";
  /// # assert_eq!(select_query, expected);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// CREATE INDEX users_name_idx
  /// ON users (name)
  /// WHERE
  ///   created_at >= $1
  ///   OR status = 'active'
  /// ```
  pub fn where_or(mut self, condition: &str) -> Self {
    push_unique(&mut self._where, (LogicalOperator::Or, condition.trim().to_string()));
    self
  }
}

#[cfg(any(doc, feature = "postgresql"))]
#[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
impl CreateIndex {
  /// Defines the `concurrently` parameter
  ///
  /// ### Example
  ///
  /// ```
  /// # #[cfg(feature = "postgresql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::CreateIndex::new()
  ///   .create_index("users_name_idx")
  ///   .concurrently()
  ///   .to_string();
  ///
  /// # let expected = "CREATE INDEX CONCURRENTLY users_name_idx";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// CREATE INDEX CONCURRENTLY users_name_idx
  /// ```
  pub fn concurrently(mut self) -> Self {
    self._concurrently = true;
    self
  }

  /// Defines the include parameter
  ///
  /// ### Example
  ///
  ///```
  /// # #[cfg(feature = "postgresql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::CreateIndex::new()
  ///   .include("login")
  ///   .include("name")
  ///   .as_string();
  ///
  /// # let expected = "INCLUDE (login, name)";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// INCLUDE (login, name)
  /// ```
  pub fn include(mut self, column_name: &str) -> Self {
    push_unique(&mut self._include, column_name.trim().to_string());
    self
  }

  /// Defines the `only` parameter
  ///
  /// ### Example
  ///
  /// ```
  /// # #[cfg(feature = "postgresql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::CreateIndex::new()
  ///   .on("users")
  ///   .only()
  ///   .to_string();
  ///
  /// # let expected = "ON ONLY users";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// ON ONLY users
  /// ```
  pub fn only(mut self) -> Self {
    self._only = true;
    self
  }

  /// Defines the index method to be used to create the index, this method overrides the previous value
  ///
  /// ### Example
  ///
  ///```
  /// # #[cfg(feature = "postgresql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::CreateIndex::new()
  ///   .using("btree")
  ///   .using("gist")
  ///   .as_string();
  ///
  /// # let expected = "USING gist";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// USING gist
  /// ```
  pub fn using(mut self, index_method: &str) -> Self {
    self._using = index_method.trim().to_string();
    self
  }
}

impl std::fmt::Display for CreateIndex {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl std::fmt::Debug for CreateIndex {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let fmts = fmt::multiline();
    write!(f, "{}", fmt::format(self.concat(&fmts), &fmts))
  }
}
