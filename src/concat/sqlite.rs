#[cfg(feature = "sqlite")]
use crate::{
  concat::concat_raw_before_after,
  fmt,
  structure::{UpdateClause, UpdateVars},
};

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
