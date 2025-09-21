use crate::{
  concat::{
    sql_standard::{ConcatFrom, ConcatJoin, ConcatSet, ConcatWhere},
    Concat,
  },
  fmt,
  structure::{Update, UpdateClause},
};

impl ConcatFrom<UpdateClause> for Update {}
impl ConcatWhere<UpdateClause> for Update {}
impl ConcatJoin<UpdateClause> for Update {}
impl ConcatSet<UpdateClause> for Update {}

impl Concat for Update {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_string();

    #[cfg(not(any(feature = "postgresql", feature = "sqlite", feature = "mysql")))]
    {
      query = self.concat_raw(query, &fmts, &self._raw);
      query = self.concat_update(query, &fmts);
      query = self.concat_set(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::Set,
        &self._set,
      );
      query = self.concat_where(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::Where,
        &self._where,
      );
    }

    #[cfg(feature = "postgresql")]
    {
      query = self.concat_raw(query, &fmts, &self._raw);
      query = self.concat_with(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::With,
        &self._with,
      );
      query = self.concat_update(query, &fmts);
      query = self.concat_set(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::Set,
        &self._set,
      );
      query = self.concat_from(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::From,
        &self._from,
      );
      query = self.concat_where(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::Where,
        &self._where,
      );
      query = self.concat_returning(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::Returning,
        &self._returning,
      );
    }

    #[cfg(feature = "sqlite")]
    {
      query = self.concat_raw(query, &fmts, &self._raw);
      query = self.concat_with(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::With,
        &self._with,
      );
      query = self.concat_update(&self._raw_before, &self._raw_after, query, &fmts, &self._update);
      query = self.concat_set(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::Set,
        &self._set,
      );
      query = self.concat_from(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::From,
        &self._from,
      );
      query = self.concat_join(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::Join,
        &self._join,
      );
      query = self.concat_where(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::Where,
        &self._where,
      );
      query = self.concat_returning(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::Returning,
        &self._returning,
      );
    }

    #[cfg(feature = "mysql")]
    {
      query = self.concat_raw(query, &fmts, &self._raw);
      query = self.concat_update(query, &fmts);
      query = self.concat_set(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::Set,
        &self._set,
      );
      query = self.concat_where(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::Where,
        &self._where,
      );
      query = self.concat_order_by(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::OrderBy,
        &self._order_by,
      );
      query = self.concat_limit(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::Limit,
        &self._limit,
      );
    }

    query.trim_end().to_string()
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
use crate::concat::non_standard::{ConcatReturning, ConcatWith};

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
impl ConcatReturning<UpdateClause> for Update {}
#[cfg(any(feature = "postgresql", feature = "sqlite"))]
impl ConcatWith<UpdateClause> for Update {}

#[cfg(feature = "sqlite")]
use crate::concat::sqlite::ConcatUpdate;

#[cfg(feature = "sqlite")]
impl ConcatUpdate for Update {}

#[cfg(not(feature = "sqlite"))]
use crate::concat::concat_raw_before_after;

#[cfg(not(feature = "sqlite"))]
impl Update {
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

#[cfg(feature = "mysql")]
use crate::concat::{non_standard::ConcatLimit, sql_standard::ConcatOrderBy};

#[cfg(feature = "mysql")]
impl ConcatLimit<UpdateClause> for Update {}
#[cfg(feature = "mysql")]
impl ConcatOrderBy<UpdateClause> for Update {}
