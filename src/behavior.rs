use crate::fmt;
#[cfg(feature = "sqlite")]
use crate::structure::{InsertClause, InsertVars, UpdateClause, UpdateVars};
use std::cmp::PartialEq;

pub trait Concat {
  fn concat(&self, fmts: &fmt::Formatter) -> String;
}

pub trait ConcatSqlStandard<Clause: PartialEq> {
  fn concat_from(
    &self,
    items_raw_before: &Vec<(Clause, String)>,
    items_raw_after: &Vec<(Clause, String)>,
    query: String,
    fmts: &fmt::Formatter,
    clause: Clause,
    items: &Vec<String>,
  ) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;
    let sql = if items.is_empty() == false {
      let tables = items.join(comma);
      format!("FROM{space}{tables}{space}{lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(items_raw_before, items_raw_after, query, fmts, clause, sql)
  }

  fn concat_raw(&self, query: String, fmts: &fmt::Formatter, items: &Vec<String>) -> String {
    if items.is_empty() {
      return query;
    }
    let fmt::Formatter { lb, space, .. } = fmts;
    let raw_sql = items.join(space);

    format!("{query}{raw_sql}{space}{lb}")
  }

  fn concat_values(
    &self,
    items_raw_before: &Vec<(Clause, String)>,
    items_raw_after: &Vec<(Clause, String)>,
    query: String,
    fmts: &fmt::Formatter,
    clause: Clause,
    items: &Vec<String>,
  ) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;
    let sql = if items.is_empty() == false {
      let sep = format!("{comma}{lb}");
      let values = items.join(&sep);
      format!("VALUES{space}{lb}{values}{space}{lb}")
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
    let fmt::Formatter { lb, space, indent, .. } = fmts;
    let sql = if items.is_empty() == false {
      let conditions = items.join(&format!("{space}{lb}{indent}AND{space}"));
      format!("WHERE{space}{conditions}{space}{lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(items_raw_before, items_raw_after, query, fmts, clause, sql)
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
pub trait ConcatCommon<Clause: PartialEq> {
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

  fn concat_with(
    &self,
    items_raw_before: &Vec<(Clause, String)>,
    items_raw_after: &Vec<(Clause, String)>,
    query: String,
    fmts: &fmt::Formatter,
    clause: Clause,
    items: &Vec<(String, std::sync::Arc<dyn WithQuery>)>,
  ) -> String {
    let fmt::Formatter {
      comma,
      lb,
      indent,
      space,
      ..
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
          ..*fmts
        };
        let query_string = query.concat(&inner_fmts);

        format!("{acc}{name}{space}AS{space}({lb}{indent}{query_string}{lb}){comma}{lb}")
      });
      let with = &with[..with.len() - comma.len() - lb.len()];

      format!("WITH{space}{lb}{with}{space}{lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(items_raw_before, items_raw_after, query, fmts, clause, sql)
  }
}

#[cfg(feature = "sqlite")]
pub trait ConcatSqlite {
  fn concat_insert(
    &self,
    items_raw_before: &Vec<(InsertClause, String)>,
    items_raw_after: &Vec<(InsertClause, String)>,
    query: String,
    fmts: &fmt::Formatter,
    insert: &(InsertVars, String),
  ) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;

    let (clause, sql) = match insert {
      (InsertVars::InsertInto, exp) if exp.is_empty() => (InsertClause::InsertInto, "".to_owned()),
      (InsertVars::InsertInto, exp) => (InsertClause::InsertInto, format!("INSERT INTO{space}{exp}{space}{lb}")),

      (InsertVars::InsertOr, exp) if exp.is_empty() => (InsertClause::InsertOr, "".to_owned()),
      (InsertVars::InsertOr, exp) => (InsertClause::InsertOr, format!("INSERT OR{space}{exp}{space}{lb}")),

      (InsertVars::ReplaceInto, exp) if exp.is_empty() => (InsertClause::ReplaceInto, "".to_owned()),
      (InsertVars::ReplaceInto, exp) => (
        InsertClause::ReplaceInto,
        format!("REPLACE INTO{space}{exp}{space}{lb}"),
      ),
    };

    concat_raw_before_after(items_raw_before, items_raw_after, query, fmts, clause, sql)
  }

  fn concat_join<Clause: PartialEq>(
    &self,
    items_raw_before: &Vec<(Clause, String)>,
    items_raw_after: &Vec<(Clause, String)>,
    query: String,
    fmts: &fmt::Formatter,
    clause: Clause,
    join: &Vec<String>,
  ) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if join.is_empty() == false {
      let joins = join.join(format!("{space}{lb}").as_str());
      format!("{joins}{space}{lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(&items_raw_before, &items_raw_after, query, fmts, clause, sql)
  }

  fn concat_update(
    &self,
    items_raw_before: &Vec<(UpdateClause, String)>,
    items_raw_after: &Vec<(UpdateClause, String)>,
    query: String,
    fmts: &fmt::Formatter,
    update: &(UpdateVars, String),
  ) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let (clause, sql) = match update {
      (UpdateVars::Update, table_name) if table_name.is_empty() => (UpdateClause::Update, "".to_string()),
      (UpdateVars::Update, table_name) => (UpdateClause::Update, format!("UPDATE{space}{table_name}{space}{lb}")),

      (UpdateVars::UpdateOr, expression) if expression.is_empty() => (UpdateClause::UpdateOr, "".to_string()),
      (UpdateVars::UpdateOr, expression) => (
        UpdateClause::UpdateOr,
        format!("UPDATE OR{space}{expression}{space}{lb}"),
      ),
    };

    concat_raw_before_after(items_raw_before, items_raw_after, query, fmts, clause, sql)
  }

  fn concat_values<Clause: PartialEq>(
    &self,
    items_raw_before: &Vec<(Clause, String)>,
    items_raw_after: &Vec<(Clause, String)>,
    query: String,
    fmts: &fmt::Formatter,
    clause: Clause,
    values: &Vec<String>,
    default_values: &bool,
  ) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;

    let sql = if *default_values {
      format!("DEFAULT VALUES{space}{lb}")
    } else if values.is_empty() == false {
      let sep = format!("{comma}{lb}");
      let values = values.join(&sep);
      format!("VALUES{space}{lb}{values}{space}{lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(items_raw_before, items_raw_after, query, fmts, clause, sql)
  }
}

/// Represents all commands that can be used in a transaction
pub trait TransactionQuery: Concat {}

/// Represents all commands that can be used inside the with method
pub trait WithQuery: Concat {}

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

pub fn push_unique<T: PartialEq>(list: &mut Vec<T>, value: T) {
  let prev_item = list.iter().find(|&item| *item == value);
  if prev_item.is_none() {
    list.push(value);
  }
}

pub fn raw_queries<Clause: PartialEq>(raw_list: &Vec<(Clause, String)>, clause: &Clause) -> Vec<String> {
  raw_list
    .iter()
    .filter(|item| item.0 == *clause)
    .map(|item| item.1.clone())
    .collect::<Vec<_>>()
}
