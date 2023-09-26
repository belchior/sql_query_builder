#[cfg(any(feature = "postgresql", feature = "sqlite"))]
use crate::behavior::ConcatCommon;
use crate::{
  behavior::{concat_raw_before_after, Concat, ConcatSqlStandard},
  fmt,
  structure::{Delete, DeleteClause},
};

impl ConcatSqlStandard<DeleteClause> for Delete {}

impl Delete {
  fn concat_delete_from(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._delete_from.is_empty() == false {
      let table_name = &self._delete_from;
      format!("DELETE FROM{space}{table_name}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      DeleteClause::DeleteFrom,
      sql,
    )
  }
}

impl Concat for Delete {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_string();

    query = self.concat_raw(query, &fmts, &self._raw);
    #[cfg(any(feature = "postgresql", feature = "sqlite"))]
    {
      query = self.concat_with(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        DeleteClause::With,
        &self._with,
      );
    }
    query = self.concat_delete_from(query, &fmts);
    query = self.concat_where(
      &self._raw_before,
      &self._raw_after,
      query,
      &fmts,
      DeleteClause::Where,
      &self._where,
    );
    #[cfg(any(feature = "postgresql", feature = "sqlite"))]
    {
      query = self.concat_returning(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        DeleteClause::Returning,
        &self._returning,
      );
    }

    query.trim_end().to_string()
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
impl ConcatCommon<DeleteClause> for Delete {}
