mod concat;
mod fmt;
mod structure;

use fmt::Formatter;
use structure::SelectBuilder;

impl<'a> SelectBuilder<'a> {
  pub fn and(mut self, clause: &'a str) -> Self {
    self = self.where_clause(clause);
    self
  }

  pub fn as_string(&self) -> String {
    let fmts = Formatter::new(false);
    self.concat(&fmts)
  }

  pub fn debug(self) -> Self {
    let fmts = Formatter::new(true);
    println!("{}", self.concat(&fmts));
    self
  }

  pub fn from(mut self, tables: &'a str) -> Self {
    self._from.push(tables.to_owned());
    self
  }

  pub fn inner_join(mut self, table: &'a str, on: &'a str) -> Self {
    self._join.push(format!("INNER JOIN {table} ON {on}"));
    self
  }

  pub fn limit(mut self, num: &'a str) -> Self {
    self._limit = num;
    self
  }

  pub fn new() -> Self {
    Self::default()
  }

  pub fn order_by(mut self, column: &'a str) -> Self {
    self._order_by.push(column.to_owned());
    self
  }

  pub fn print(self) -> Self {
    let fmts = Formatter::new(false);
    println!("{}", self.concat(&fmts));
    self
  }

  pub fn raw(mut self, raw_sql: &'a str) -> Self {
    self._raw.push(raw_sql.to_owned());
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

  pub fn where_clause(mut self, clause: &'a str) -> Self {
    self._where.push(clause.to_owned());
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
    let fmts = Formatter::new(true);
    write!(f, "{}", self.concat(&fmts))
  }
}
