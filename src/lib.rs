mod concat;
mod fmt;
mod structure;

pub use structure::{Clause, SelectBuilder};

impl<'a> SelectBuilder<'a> {
  pub fn and(mut self, clause: &'a str) -> Self {
    self = self.where_clause(clause);
    self
  }

  pub fn as_string(&self) -> String {
    let fmts = fmt::Formatter::one_line();
    self.concat(&fmts)
  }

  pub fn debug(self) -> Self {
    let fmts = fmt::Formatter::multi_line();
    println!("{}", fmt::colorize(self.concat(&fmts)));
    self
  }

  pub fn except(mut self, select: Self) -> Self {
    self._except.push(select);
    self
  }

  pub fn from(mut self, tables: &'a str) -> Self {
    self._from.push(tables.to_owned());
    self
  }

  pub fn group_by(mut self, column: &'a str) -> Self {
    self._group_by.push(column.to_owned());
    self
  }

  pub fn having(mut self, condition: &'a str) -> Self {
    self._having.push(condition.to_owned());
    self
  }

  pub fn intersect(mut self, select: Self) -> Self {
    self._intersect.push(select);
    self
  }

  pub fn limit(mut self, num: &'a str) -> Self {
    self._limit = num;
    self
  }

  pub fn new() -> Self {
    Self::default()
  }

  pub fn offset(mut self, num: &'a str) -> Self {
    self._offset = num;
    self
  }

  pub fn order_by(mut self, column: &'a str) -> Self {
    self._order_by.push(column.to_owned());
    self
  }

  pub fn print(self) -> Self {
    let fmts = fmt::Formatter::one_line();
    println!("{}", fmt::colorize(self.concat(&fmts)));
    self
  }

  pub fn raw(mut self, raw_sql: &'a str) -> Self {
    self._raw.push(raw_sql.to_owned());
    self
  }

  pub fn raw_after(mut self, clause: Clause, raw_sql: &'a str) -> Self {
    self._raw_after.push((clause, raw_sql.to_owned()));
    self
  }

  pub fn raw_before(mut self, clause: Clause, raw_sql: &'a str) -> Self {
    self._raw_before.push((clause, raw_sql.to_owned()));
    self
  }

  pub fn select(mut self, column: &'a str) -> Self {
    self._select.push(column.to_owned());
    self
  }

  pub fn union(mut self, select: Self) -> Self {
    self._union.push(select);
    self
  }

  pub fn where_clause(mut self, condition: &'a str) -> Self {
    self._where.push(condition.to_owned());
    self
  }

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
