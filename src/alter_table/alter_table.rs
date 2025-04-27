use crate::{
  behavior::TransactionQuery,
  concat::Concat,
  fmt,
  structure::{AlterTable, AlterTableAction, AlterTableActionItem, AlterTableOrderedAction},
  utils::push_unique,
};

impl TransactionQuery for AlterTable {}

impl AlterTable {
  /// Adds columns or table constraints, this method overrides the previous value
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::AlterTable::new()
  ///   .add("COLUMN age int not null")
  ///   .as_string();
  ///
  /// # let expected = "ADD COLUMN age int not null";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// ADD COLUMN age int not null
  /// ```
  ///
  /// ### Available on crate feature `postgresql` and `mysql` only.
  /// Multiples call of this method will build the SQL respecting the order of the calls
  ///
  /// ### Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "mysql"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::AlterTable::new()
  ///   .add("COLUMN login varchar not null")
  ///   .add("CONSTRAINT login_unique unique(login)")
  ///   .as_string();
  ///
  /// # let expected = "ADD COLUMN login varchar not null, ADD CONSTRAINT login_unique unique(login)";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// ADD COLUMN login varchar not null,
  /// ADD CONSTRAINT login_unique unique(login)
  /// ```
  pub fn add(mut self, add_exp: &str) -> Self {
    let action = AlterTableActionItem(AlterTableOrderedAction::Add, add_exp.trim().to_string());
    push_unique(&mut self._ordered_actions, action);
    self
  }

  /// Defines the name of the table to be altered, this method overrides the previous value
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::AlterTable::new()
  ///   .alter_table("users")
  ///   .as_string();
  ///
  /// # let expected = "ALTER TABLE users";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// ALTER TABLE users
  /// ```
  pub fn alter_table(mut self, table_name: &str) -> Self {
    self._alter_table = table_name.trim().to_string();
    self
  }

  /// Gets the current state of the [AlterTable] and returns it as string
  ///
  /// ### Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::AlterTable::new()
  ///   .alter_table("users")
  ///   .rename_to("users_old")
  ///   .as_string();
  ///
  /// # let expected = "ALTER TABLE users RENAME TO users_old";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// ALTER TABLE users RENAME TO users_old
  /// ```
  pub fn as_string(&self) -> String {
    let fmts = fmt::one_line();
    self.concat(&fmts)
  }

  /// Prints the current state of the [AlterTable] to the standard output in a more ease to read version.
  /// This method is useful to debug complex queries or just print the generated SQL while you type
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::AlterTable::new()
  ///   .alter_table("users")
  ///   .add("name varchar(100) not null")
  ///   .debug()
  ///   .as_string();
  /// ```
  ///
  /// Prints to the standard output
  ///
  /// ```sql
  /// -- ------------------------------------------------------------------------------
  /// ALTER TABLE users
  ///   ADD name varchar(100) not null
  /// -- ------------------------------------------------------------------------------
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::multiline();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// Drops columns or table constraints, this method overrides the previous value.
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let query = sql::AlterTable::new()
  ///   .drop("column login")
  ///   .as_string();
  ///
  /// # let expected = "DROP column login";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// DROP column login
  /// ```
  ///
  /// ### Available on crate feature `postgresql` and `mysql` only.
  /// Multiples call of this method will build the SQL respecting the order of the calls
  ///
  /// ### Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "mysql"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::AlterTable::new()
  ///   .drop("column login")
  ///   .drop("constraint login_unique")
  ///   .as_string();
  ///
  /// # let expected = "DROP column login, DROP constraint login_unique";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// DROP column login, DROP constraint login_unique
  /// ```
  pub fn drop(mut self, drop_exp: &str) -> Self {
    let action = AlterTableActionItem(AlterTableOrderedAction::Drop, drop_exp.trim().to_string());
    push_unique(&mut self._ordered_actions, action);
    self
  }

  /// Creates instance of the [AlterTable] command
  pub fn new() -> Self {
    Self::default()
  }

  /// Prints the current state of the [AlterTable] to the standard output similar to debug method,
  /// the difference is that this method prints in one line.
  pub fn print(self) -> Self {
    let fmts = fmt::one_line();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// Adds at the beginning a raw SQL query. Is useful to create a more complex alter table signature.
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let create_table_query = sql::AlterTable::new()
  ///   .raw("ALTER TABLE IF EXISTS users")
  ///   .drop("legacy_column")
  ///   .as_string();
  ///
  /// # let expected = "ALTER TABLE IF EXISTS users DROP legacy_column";
  /// # assert_eq!(expected, create_table_query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// ALTER TABLE IF EXISTS users DROP legacy_column
  /// ```
  pub fn raw(mut self, raw_sql: &str) -> Self {
    push_unique(&mut self._raw, raw_sql.trim().to_string());
    self
  }

  /// Adds a raw SQL query after a specified parameter.
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let raw = "ADD COLUMN name varchar(100) not null";
  ///
  /// let query = sql::AlterTable::new()
  ///   .alter_table("users")
  ///   .raw_after(sql::AlterTableAction::AlterTable, raw)
  ///   .as_string();
  ///
  /// # let expected = "ALTER TABLE users ADD COLUMN name varchar(100) not null";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// ALTER TABLE users ADD COLUMN name varchar(100) not null
  /// ```
  pub fn raw_after(mut self, param: AlterTableAction, raw_sql: &str) -> Self {
    self._raw_after.push((param, raw_sql.trim().to_string()));
    self
  }

  /// Adds a raw SQL query before a specified parameter.
  ///
  /// ### Example
  ///
  /// ```
  /// # use sql_query_builder as sql;
  /// let raw = "/* alter table command */";
  ///
  /// let query = sql::AlterTable::new()
  ///   .raw_before(sql::AlterTableAction::AlterTable, raw)
  ///   .alter_table("users")
  ///   .as_string();
  ///
  /// # let expected = "/* alter table command */ ALTER TABLE users";
  /// # assert_eq!(expected, query);
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// /* alter table command */ ALTER TABLE users
  /// ```
  pub fn raw_before(mut self, action: AlterTableAction, raw_sql: &str) -> Self {
    self._raw_before.push((action, raw_sql.trim().to_string()));
    self
  }
}

#[cfg(any(doc, feature = "postgresql", feature = "sqlite", feature = "mysql"))]
#[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
#[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
impl AlterTable {
  /// Changes the column name or table constraints, this method overrides the previous value
  ///
  /// ### Example
  ///
  ///```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::AlterTable::new()
  ///   .alter_table("users")
  ///   .rename("COLUMN address TO city")
  ///   .to_string();
  ///
  /// # let expected = "ALTER TABLE users RENAME COLUMN address TO city";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// ALTER TABLE users RENAME COLUMN address TO city
  /// ```
  ///
  /// ### Available on crate feature `mysql` only.
  /// Changes the table name, column name or table constraints,
  /// multiples call of this method will build the SQL respecting the order of the calls
  ///
  /// ### Example
  ///
  ///```
  /// # #[cfg(feature = "mysql")]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::AlterTable::new()
  ///   .alter_table("users")
  ///   .rename("TO users_old")
  ///   .rename("COLUMN name TO full_name")
  ///   .to_string();
  ///
  /// # let expected = "ALTER TABLE users RENAME TO users_old, RENAME COLUMN name TO full_name";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// ALTER TABLE users
  ///   RENAME TO users_old,
  ///   RENAME COLUMN name TO full_name
  /// ```
  pub fn rename(mut self, action: &str) -> Self {
    #[cfg(feature = "mysql")]
    {
      let action = AlterTableActionItem(AlterTableOrderedAction::Rename, action.trim().to_string());
      push_unique(&mut self._ordered_actions, action);
    }
    #[cfg(not(feature = "mysql"))]
    {
      self._rename = action.trim().to_string();
    }

    self
  }
}

#[cfg(any(doc, feature = "postgresql", feature = "sqlite"))]
#[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
impl AlterTable {
  /// Changes the name of the table, this method overrides the previous value
  ///
  /// ### Example
  ///
  /// ```
  /// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::AlterTable::new()
  ///   .alter_table("users")
  ///   .rename_to("users_old")
  ///   .to_string();
  ///
  /// # let expected = "ALTER TABLE users RENAME TO users_old";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// ALTER TABLE users RENAME TO users_old
  /// ```
  pub fn rename_to(mut self, table_name: &str) -> Self {
    self._rename_to = table_name.trim().to_string();
    self
  }
}

#[cfg(any(doc, feature = "postgresql", feature = "mysql"))]
#[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
#[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
impl AlterTable {
  /// Alter columns or table constraints.
  /// Multiples call of this method will build the SQL respecting the order of the calls
  ///
  /// ### Example
  ///
  ///```
  /// # #[cfg(any(feature = "postgresql", feature = "mysql"))]
  /// # {
  /// # use sql_query_builder as sql;
  /// let query = sql::AlterTable::new()
  ///   .alter("COLUMN created_at SET DEFAULT now()")
  ///   .to_string();
  ///
  /// # let expected = "ALTER COLUMN created_at SET DEFAULT now()";
  /// # assert_eq!(expected, query);
  /// # }
  /// ```
  ///
  /// Outputs
  ///
  /// ```sql
  /// ALTER COLUMN created_at SET DEFAULT now()
  /// ```
  pub fn alter(mut self, alter_exp: &str) -> Self {
    let action = AlterTableActionItem(AlterTableOrderedAction::Alter, alter_exp.trim().to_string());
    push_unique(&mut self._ordered_actions, action);
    self
  }
}

impl std::fmt::Display for AlterTable {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl std::fmt::Debug for AlterTable {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let fmts = fmt::multiline();
    write!(f, "{}", fmt::format(self.concat(&fmts), &fmts))
  }
}
