#[cfg(feature = "sqlite")]
use crate::{
  concat::concat_raw_before_after,
  fmt,
  structure::{InsertClause, InsertVars, UpdateClause, UpdateVars},
};

#[cfg(feature = "sqlite")]
pub(crate) trait ConcatInsert {
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
      (InsertVars::InsertInto, exp) if exp.is_empty() => (InsertClause::InsertInto, "".to_string()),
      (InsertVars::InsertInto, exp) => (InsertClause::InsertInto, format!("INSERT INTO{space}{exp}{space}{lb}")),

      (InsertVars::InsertOr, exp) if exp.is_empty() => (InsertClause::InsertOr, "".to_string()),
      (InsertVars::InsertOr, exp) => (InsertClause::InsertOr, format!("INSERT OR{space}{exp}{space}{lb}")),

      (InsertVars::ReplaceInto, exp) if exp.is_empty() => (InsertClause::ReplaceInto, "".to_string()),
      (InsertVars::ReplaceInto, exp) => (
        InsertClause::ReplaceInto,
        format!("REPLACE INTO{space}{exp}{space}{lb}"),
      ),
    };

    concat_raw_before_after(items_raw_before, items_raw_after, query, fmts, clause, sql)
  }
}

#[cfg(feature = "sqlite")]
pub(crate) trait ConcatJoin {
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
      "".to_string()
    };

    concat_raw_before_after(&items_raw_before, &items_raw_after, query, fmts, clause, sql)
  }
}

#[cfg(feature = "sqlite")]
pub(crate) trait ConcatUpdate {
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
}
