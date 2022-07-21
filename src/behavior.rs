use crate::fmt;
use std::cmp::PartialEq;

pub fn push_unique<T: Eq>(list: &mut Vec<T>, value: T) {
  let prev_item = list.iter().find(|&item| *item == value);
  if prev_item.is_none() {
    list.push(value);
  }
}

pub fn raw_queries<'a, Clause: PartialEq>(raw_list: &'a Vec<(Clause, String)>, clause: &'a Clause) -> Vec<String> {
  raw_list
    .iter()
    .filter(|item| item.0 == *clause)
    .map(|item| item.1.clone())
    .collect::<Vec<_>>()
}

/// Represents all statements that can be used in the with method
pub trait WithQuery: Concat {}

pub trait Concat {
  fn concat(&self, fmts: &fmt::Formatter) -> String;
}

pub fn concat_raw_before_after<Clause: PartialEq>(
  items_before: &Vec<(Clause, String)>,
  items_after: &Vec<(Clause, String)>,
  query: String,
  fmts: &fmt::Formatter,
  clause: Clause,
  sql: String,
) -> String {
  let fmt::Formatter { space, .. } = fmts;
  let raw_before = raw_queries(items_before, &clause).join(space);
  let raw_after = raw_queries(items_after, &clause).join(space);
  let space_after = if raw_after.is_empty() == false { space } else { "" };
  let space_before = if raw_before.is_empty() == false { space } else { "" };

  format!("{query}{raw_before}{space_before}{sql}{raw_after}{space_after}")
}

pub trait ConcatMethods<'a, Clause: PartialEq> {
  fn concat_raw(&self, query: String, fmts: &fmt::Formatter, items: &Vec<String>) -> String {
    if items.is_empty() {
      return query;
    }
    let fmt::Formatter { lb, space, .. } = fmts;
    let raw_sql = items.join(space);

    format!("{query}{raw_sql}{space}{lb}")
  }

  #[cfg(feature = "postgresql")]
  fn concat_returning(
    &self,
    items_raw_before: &Vec<(Clause, String)>,
    items_raw_after: &Vec<(Clause, String)>,
    query: String,
    fmts: &fmt::Formatter,
    clause: Clause,
    items: &Vec<String>,
  ) -> String {
    let fmt::Formatter { lb, space, comma, .. } = fmts;
    let sql = if items.is_empty() == false {
      let output_names = items.join(comma);
      format!("RETURNING{space}{output_names}{space}{lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(items_raw_before, items_raw_after, query, fmts, clause, sql)
  }

  fn concat_where(
    &self,
    items_raw_before: &Vec<(Clause, String)>,
    items_raw_after: &Vec<(Clause, String)>,
    query: String,
    fmts: &fmt::Formatter,
    clause: Clause,
    items: &Vec<String>,
  ) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if items.is_empty() == false {
      let conditions = items.join(" AND ");
      format!("WHERE{space}{conditions}{space}{lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(items_raw_before, items_raw_after, query, fmts, clause, sql)
  }

  #[cfg(feature = "postgresql")]
  fn concat_with(
    &self,
    items_raw_before: &Vec<(Clause, String)>,
    items_raw_after: &Vec<(Clause, String)>,
    query: String,
    fmts: &fmt::Formatter,
    clause: Clause,
    items: &Vec<(&'a str, std::sync::Arc<dyn WithQuery>)>,
  ) -> String {
    let fmt::Formatter {
      comma,
      lb,
      indent,
      space,
    } = fmts;
    let sql = if items.is_empty() == false {
      let with = items.iter().fold("".to_owned(), |acc, item| {
        let (name, query) = item;
        let inner_lb = format!("{lb}{indent}");
        let inner_fmts = fmt::Formatter {
          comma,
          lb: inner_lb.as_str(),
          indent,
          space,
        };
        let query_string = query.concat(&inner_fmts);

        format!("{acc}{name}{space}AS{space}({lb}{indent}{query_string}{lb}){comma}")
      });
      let with = &with[..with.len() - comma.len()];

      format!("WITH{space}{with}{space}{lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(items_raw_before, items_raw_after, query, fmts, clause, sql)
  }
}
