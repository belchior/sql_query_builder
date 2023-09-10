use crate::{
  behavior::{Concat, ConcatSqlStandard, TransactionQuery},
  fmt,
  structure::{
    TrCmd::{self, *},
    TransactionCommand,
  },
  Transaction,
};

impl ConcatSqlStandard<TransactionCommand> for Transaction {}

impl TransactionQuery for TransactionCommand {}

impl Concat for Transaction {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_owned();

    query = self.concat_raw(query, &fmts, &self._raw);

    #[cfg(any(feature = "postgresql", feature = "sqlite"))]
    {
      query = self.concat_begin(query, &fmts);
    }

    query = self.concat_start_transaction(query, &fmts);

    query = self.concat_set_transaction(query, &fmts);

    query = self.concat_ordered_commands(query, &fmts);

    query = self.concat_commit(query, &fmts);

    #[cfg(any(feature = "postgresql", feature = "sqlite"))]
    {
      query = self.concat_end(query, &fmts);
    }

    query.trim_end().to_owned()
  }
}

impl Concat for TransactionCommand {
  fn concat(&self, fmts: &crate::fmt::Formatter) -> String {
    let fmt::Formatter { space, .. } = fmts;
    let arg = if self.1.is_empty() {
      "".to_owned()
    } else {
      format!("{space}{0}", self.1)
    };
    match self.0 {
      Commit => format!("COMMIT{arg}"),
      ReleaseSavepoint => format!("RELEASE SAVEPOINT{arg}"),
      Rollback => format!("ROLLBACK{arg}"),
      Savepoint => format!("SAVEPOINT{arg}"),

      #[cfg(any(feature = "postgresql", feature = "sqlite"))]
      Begin => format!("BEGIN{arg}"),
      #[cfg(any(feature = "postgresql", feature = "sqlite"))]
      End => format!("END{arg}"),

      #[cfg(not(feature = "sqlite"))]
      SetTransaction => format!("SET TRANSACTION{arg}"),
      #[cfg(not(feature = "sqlite"))]
      StartTransaction => format!("START TRANSACTION{arg}"),
    }
  }
}

impl Transaction {
  fn concat_commit(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = match &self._commit {
      Some(cmd) => format!("{0};{space}{lb}", cmd.concat(fmts)),
      None => "".to_owned(),
    };

    format!("{query}{sql}")
  }

  fn concat_set_transaction(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = match &self._set_transaction {
      Some(cmd) => format!("{0};{space}{lb}", cmd.concat(fmts)),
      None => "".to_owned(),
    };

    format!("{query}{sql}")
  }

  fn concat_start_transaction(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = match &self._start_transaction {
      Some(cmd) => format!("{0};{space}{lb}", cmd.concat(fmts)),
      None => "".to_owned(),
    };

    format!("{query}{sql}")
  }

  fn concat_ordered_commands(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = self._ordered_commands.iter().fold("".to_owned(), |acc, cmd| {
      format!("{acc}{0};{space}{lb}", cmd.concat(fmts))
    });

    format!("{query}{sql}")
  }
}

impl TransactionCommand {
  pub(crate) fn new(clause: TrCmd, arg: String) -> Self {
    Self(clause, arg)
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
impl Transaction {
  fn concat_begin(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = match &self._begin {
      Some(cmd) => format!("{0};{space}{lb}", cmd.concat(fmts)),
      None => "".to_owned(),
    };

    format!("{query}{sql}")
  }

  fn concat_end(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = match &self._end {
      Some(cmd) => format!("{0};{space}{lb}", cmd.concat(fmts)),
      None => "".to_owned(),
    };

    format!("{query}{sql}")
  }
}
