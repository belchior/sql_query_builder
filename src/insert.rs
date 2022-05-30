use crate::{
  behavior::BuilderInner,
  fmt,
  structure::{InsertBuilder, InsertClause},
};

impl<'a> InsertBuilder<'a> {
  pub fn as_string(&self) -> String {
    let fmts = fmt::Formatter::one_line();
    self.concat(&fmts)
  }

  pub fn debug(self) -> Self {
    let fmts = fmt::Formatter::multi_line();
    println!("{}", fmt::colorize(self.concat(&fmts)));
    self
  }

  pub fn insert_into(mut self, table_name: &'a str) -> Self {
    self._insert_into = table_name;
    self
  }

  pub fn new() -> Self {
    Self::default()
  }

  pub fn print(self) -> Self {
    let fmts = fmt::Formatter::one_line();
    println!("{}", fmt::colorize(self.concat(&fmts)));
    self
  }

  pub fn raw_after(mut self, clause: InsertClause, raw_sql: &'a str) -> Self {
    self._raw_after.push((clause, raw_sql.to_owned()));
    self
  }

  pub fn raw_before(mut self, clause: InsertClause, raw_sql: &'a str) -> Self {
    self._raw_before.push((clause, raw_sql.to_owned()));
    self
  }

  pub fn values(mut self, value: &'a str) -> Self {
    self._values.push(value.to_owned());
    self
  }
}

impl BuilderInner<'_, InsertClause> for InsertBuilder<'_> {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_owned();

    query = self.concat_insert_into(query, &fmts);
    query = self.concat_values(query, &fmts);

    query.trim_end().to_owned()
  }

  fn raw_before(&self) -> &Vec<(InsertClause, String)> {
    &self._raw_before
  }

  fn raw_after(&self) -> &Vec<(InsertClause, String)> {
    &self._raw_after
  }
}

impl InsertBuilder<'_> {
  fn concat_insert_into(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._insert_into.is_empty() == false {
      let insert_into = self._insert_into;
      format!("INSERT INTO {insert_into}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(InsertClause::InsertInto, query, fmts, sql)
  }

  fn concat_values(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._values.is_empty() == false {
      let values = self._values.join(comma);
      format!("VALUES {values}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(InsertClause::Values, query, fmts, sql)
  }
}
