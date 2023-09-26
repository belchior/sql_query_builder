#[cfg(any(feature = "postgresql", feature = "sqlite"))]
use crate::behavior::ConcatCommon;
#[cfg(feature = "sqlite")]
use crate::behavior::ConcatSqlite;
use crate::{
  behavior::{concat_raw_before_after, Concat, ConcatSqlStandard},
  fmt,
  structure::{Update, UpdateClause},
};

impl ConcatSqlStandard<UpdateClause> for Update {}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
impl ConcatCommon<UpdateClause> for Update {}

impl Update {
  fn concat_set(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._set.is_empty() == false {
      let values = self._set.join(comma);
      format!("SET{space}{values}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(&self._raw_before, &self._raw_after, query, fmts, UpdateClause::Set, sql)
  }

  #[cfg(not(feature = "sqlite"))]
  fn concat_update(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._update.is_empty() == false {
      let table_name = &self._update;
      format!("UPDATE{space}{table_name}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      UpdateClause::Update,
      sql,
    )
  }
}

impl Concat for Update {
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
        UpdateClause::With,
        &self._with,
      );
    }

    #[cfg(not(feature = "sqlite"))]
    {
      query = self.concat_update(query, &fmts);
    }
    #[cfg(feature = "sqlite")]
    {
      query = ConcatSqlite::concat_update(self, &self._raw_before, &self._raw_after, query, &fmts, &self._update);
    }

    query = self.concat_set(query, &fmts);

    #[cfg(any(feature = "postgresql", feature = "sqlite"))]
    {
      query = self.concat_from(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::From,
        &self._from,
      );
    }

    #[cfg(feature = "sqlite")]
    {
      query = ConcatSqlite::concat_join(
        self,
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::Join,
        &self._join,
      );
    }

    query = self.concat_where(
      &self._raw_before,
      &self._raw_after,
      query,
      &fmts,
      UpdateClause::Where,
      &self._where,
    );

    #[cfg(any(feature = "postgresql", feature = "sqlite"))]
    {
      query = self.concat_returning(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::Returning,
        &self._returning,
      );
    }

    query.trim_end().to_string()
  }
}

#[cfg(feature = "sqlite")]
impl ConcatSqlite for Update {}
