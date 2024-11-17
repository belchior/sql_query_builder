use crate::fmt;

pub(crate) mod non_standard;
pub(crate) mod sql_standard;
pub(crate) mod sqlite;

pub trait Concat {
  fn concat(&self, fmts: &fmt::Formatter) -> String;

  fn concat_raw(&self, query: String, fmts: &fmt::Formatter, items: &Vec<String>) -> String {
    if items.is_empty() {
      return query;
    }
    let fmt::Formatter { lb, space, .. } = fmts;
    let raw_sql = items.join(space).trim().to_string();

    format!("{query}{raw_sql}{space}{lb}")
  }
}

pub(crate) fn concat_raw_before_after<Clause: PartialEq>(
  items_before: &Vec<(Clause, String)>,
  items_after: &Vec<(Clause, String)>,
  query: String,
  fmts: &fmt::Formatter,
  clause: Clause,
  sql: String,
) -> String {
  let fmt::Formatter { space, .. } = fmts;
  let raw_before = raw_queries(items_before, &clause).join(space).trim().to_string();
  let raw_after = raw_queries(items_after, &clause).join(space).trim().to_string();
  let space_after = if raw_after.is_empty() == false { space } else { "" };
  let space_before = if raw_before.is_empty() == false { space } else { "" };

  format!("{query}{raw_before}{space_before}{sql}{raw_after}{space_after}")
}

pub(crate) fn raw_queries<Clause: PartialEq>(raw_list: &Vec<(Clause, String)>, clause: &Clause) -> Vec<String> {
  raw_list
    .iter()
    .filter(|item| item.0 == *clause)
    .map(|item| item.1.clone())
    .collect::<Vec<_>>()
}
