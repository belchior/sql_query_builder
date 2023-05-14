use crate::{
  behavior::{push_unique, Concat},
  fmt,
  structure::{Delete, Insert, Select, TrCmd::*, Transaction, TransactionCommand, Update},
};

impl Transaction {
  /// Gets the current state of the [Transaction] and returns it as string
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let query = sql::Transaction::new()
  ///   .start_transaction("")
  ///   .commit("")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// START TRANSACTION;
  /// COMMIT;
  /// ```
  pub fn as_string(&self) -> String {
    let fmts = fmt::one_line();
    self.concat(&fmts)
  }

  /// The `commit` command, this method will be always added at the end of the transaction and
  /// all consecutive call will override the previous value
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let query = sql::Transaction::new()
  ///   .commit("WORK")
  ///   .commit("TRANSACTION")
  ///   .start_transaction("")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// START TRANSACTION;
  /// COMMIT TRANSACTION;
  /// ```
  pub fn commit(mut self, arg: &str) -> Self {
    let cmd = TransactionCommand::new(Commit, arg.trim().to_owned());
    self._commit = Some(cmd);
    self
  }

  /// Prints the current state of the [Transaction] into console output in a more ease to read version.
  /// This method is useful to debug complex queries or just to print the generated SQL while you type
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let insert_foo = sql::Insert::new()
  ///   .insert_into("users (login, name)")
  ///   .values("('foo', 'Foo')");
  ///
  /// let transaction = sql::Transaction::new()
  ///   .start_transaction("isolation level serializable")
  ///   .insert(insert_foo)
  ///   .commit("")
  ///   .debug();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// -- ------------------------------------------------------------------------------
  /// START TRANSACTION isolation level serializable;
  /// INSERT INTO users (login, name)
  /// VALUES ('foo', 'Foo');
  /// COMMIT;
  /// -- ------------------------------------------------------------------------------
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::multiline();
    println!("{}", fmt::format(self.concat(&fmts), &fmts));
    self
  }

  /// The `delete` command, access the [Delete] for more info
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let delete_foo = sql::Delete::new()
  ///   .delete_from("users")
  ///   .where_clause("login = 'foo'");
  ///
  /// let query = sql::Transaction::new()
  ///   .start_transaction("")
  ///   .delete(delete_foo)
  ///   .commit("")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// START TRANSACTION;
  /// DELETE FROM users
  /// WHERE login = 'foo';
  /// COMMIT;
  /// ```
  pub fn delete(mut self, delete: Delete) -> Self {
    let cmd = Box::new(delete);
    self._ordered_commands.push(cmd);
    self
  }

  /// The `insert` command, access the [Insert] for more info
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let insert_foo = sql::Insert::new()
  ///   .insert_into("users (login, name)")
  ///   .values("('foo', 'Foo')");
  ///
  /// let query = sql::Transaction::new()
  ///   .start_transaction("")
  ///   .insert(insert_foo)
  ///   .commit("")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// START TRANSACTION;
  /// INSERT INTO users (login, name)
  /// VALUES ('foo', 'Foo');
  /// COMMIT;
  /// ```
  pub fn insert(mut self, insert: Insert) -> Self {
    let cmd = Box::new(insert);
    self._ordered_commands.push(cmd);
    self
  }

  /// Create Transaction's instance
  pub fn new() -> Self {
    Self::default()
  }

  /// Prints the current state of the [Transaction] into console output similar to debug method,
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
  /// let raw_query = "\
  ///   start transaction; \
  ///   set transaction isolation level read committed;\
  /// ";
  /// let query = sql::Transaction::new()
  ///   .raw(raw_query)
  ///   .commit("")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// start transaction;
  /// set transaction isolation level read committed;
  /// COMMIT;
  /// ```
  pub fn raw(mut self, raw_sql: &str) -> Self {
    push_unique(&mut self._raw, raw_sql.trim().to_owned());
    self
  }

  /// The `release savepoint` command
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let query = sql::Transaction::new()
  ///   .release_savepoint("saved_foo")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// RELEASE_SAVEPOINT saved_foo;
  /// ```
  pub fn release_savepoint(mut self, name: &str) -> Self {
    let cmd = Box::new(TransactionCommand::new(ReleaseSavepoint, name.trim().to_owned()));
    self._ordered_commands.push(cmd);
    self
  }

  /// The `rollback` command, this method can be used to add a `rollback to savepoint my_savepoint` command.
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let query = sql::Transaction::new()
  ///   .rollback("")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// ROLLBACK;
  /// ```
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let query = sql::Transaction::new()
  ///   .rollback("TO SAVEPOINT my_savepoint")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// ROLLBACK TO SAVEPOINT my_savepoint;
  /// ```
  pub fn rollback(mut self, arg: &str) -> Self {
    let cmd = Box::new(TransactionCommand::new(Rollback, arg.trim().to_owned()));
    self._ordered_commands.push(cmd);
    self
  }

  /// The `savepoint` command
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let query = sql::Transaction::new()
  ///   .savepoint("my_savepoint")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// SAVEPOINT my_savepoint;
  /// ```
  pub fn savepoint(mut self, name: &str) -> Self {
    let cmd = Box::new(TransactionCommand::new(Savepoint, name.trim().to_owned()));
    self._ordered_commands.push(cmd);
    self
  }

  /// The `select` command, access the [Select] for more info
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let select_foo = sql::Select::new()
  ///   .select("login, name")
  ///   .from("users")
  ///   .where_clause("id = $1");
  ///
  /// let query = sql::Transaction::new()
  ///   .start_transaction("")
  ///   .select(select_foo)
  ///   .commit("")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// START TRANSACTION;
  /// SELECT login, name FROM users WHERE id = $1;
  /// COMMIT;
  /// ```
  pub fn select(mut self, select: Select) -> Self {
    let cmd = Box::new(select);
    self._ordered_commands.push(cmd);
    self
  }

  /// The `set transaction` command, this method will be always added after the `start transaction` and
  /// all consecutive call will override the previous value
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let query = sql::Transaction::new()
  ///   .set_transaction("read write")
  ///   .set_transaction("read only")
  ///   .start_transaction("")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// START TRANSACTION;
  /// SET TRANSACTION read only;
  /// ```
  pub fn set_transaction(mut self, mode: &str) -> Self {
    let cmd = TransactionCommand::new(SetTransaction, mode.trim().to_owned());
    self._set_transaction = Some(cmd);
    self
  }

  /// The `start transaction` command, this method will be always added at the beginning of the transation and
  /// all consecutive call will override the previous value
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let query = sql::Transaction::new()
  ///   .commit("")
  ///   .start_transaction("read write")
  ///   .start_transaction("isolation level serializable")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// START TRANSACTION isolation level serializable;
  /// COMMIT;
  /// ```
  pub fn start_transaction(mut self, mode: &str) -> Self {
    let cmd = TransactionCommand::new(StartTransaction, mode.trim().to_owned());
    self._start_transaction = Some(cmd);
    self
  }

  /// The `update` command, access the [Update] for more info
  ///
  /// # Example
  ///
  /// ```
  /// use sql_query_builder as sql;
  ///
  /// let update_foo = sql::Update::new()
  ///   .update("users")
  ///   .set("name = 'Foooo'")
  ///   .where_clause("id = $1");
  ///
  /// let query = sql::Transaction::new()
  ///   .start_transaction("")
  ///   .update(update_foo)
  ///   .commit("")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// START TRANSACTION;
  /// UPDATE users SET name = 'Foooo' WHERE id = $1;
  /// COMMIT;
  /// ```
  pub fn update(mut self, update: Update) -> Self {
    let cmd = Box::new(update);
    self._ordered_commands.push(cmd);
    self
  }
}

#[cfg(any(doc, feature = "postgresql"))]
impl Transaction {
  /// The `begin` command, this method will be always added at the beginning of the transation and
  /// all consecutive call will override the previous value. The method can be used enabling the feature flag `postgresql`
  ///
  /// # Example
  ///
  /// ```text
  /// use sql_query_builder as sql;
  ///
  /// let query = sql::Transaction::new()
  ///   .commit("")
  ///   .begin("isolation level serializable")
  ///   .begin("")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// BEGIN;
  /// COMMIT;
  /// ```
  pub fn begin(mut self, mode: &str) -> Self {
    let cmd = TransactionCommand::new(Begin, mode.trim().to_owned());
    self._begin = Some(cmd);
    self
  }
}

impl std::fmt::Display for Transaction {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl std::fmt::Debug for Transaction {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let fmts = fmt::multiline();
    write!(f, "{}", fmt::format(self.concat(&fmts), &fmts))
  }
}
