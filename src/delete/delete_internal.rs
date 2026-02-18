use crate::{
  concat::{concat_raw_before_after, sql_standard::ConcatWhere, Concat},
  fmt,
  structure::{Delete, DeleteClause},
};

impl ConcatWhere<DeleteClause> for Delete {}

impl Concat for Delete {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_string();

    #[cfg(not(any(feature = "postgresql", feature = "sqlite", feature = "mysql")))]
    {
      query = self.concat_raw(query, &fmts, &self._raw);
      query = self.concat_delete_from(query, &fmts);
      query = self.concat_where(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        DeleteClause::Where,
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
        DeleteClause::With,
        &self._with,
      );
      query = self.concat_delete_from(query, &fmts);
      query = self.concat_where(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        DeleteClause::Where,
        &self._where,
      );
      query = self.concat_returning(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        DeleteClause::Returning,
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
        DeleteClause::With,
        &self._with,
      );
      query = self.concat_delete_from(query, &fmts);
      query = self.concat_where(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        DeleteClause::Where,
        &self._where,
      );
      query = self.concat_returning(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        DeleteClause::Returning,
        &self._returning,
      );
      query = self.concat_order_by(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        DeleteClause::OrderBy,
        &self._order_by,
      );
    }

    #[cfg(feature = "mysql")]
    {
      query = self.concat_raw(query, &fmts, &self._raw);
      query = self.concat_with(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        DeleteClause::With,
        &self._with,
      );
      query = self.concat_delete_from_mysql(query, &fmts);
      query = self.concat_join(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        DeleteClause::Join,
        &self._join,
      );
      query = self.concat_partition(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        DeleteClause::Partition,
        &self._partition,
      );
      query = self.concat_where(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        DeleteClause::Where,
        &self._where,
      );
      query = self.concat_order_by(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        DeleteClause::OrderBy,
        &self._order_by,
      );
      query = self.concat_limit(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        DeleteClause::Limit,
        &self._limit,
      );
    }

    query.trim_end().to_string()
  }
}

impl Delete {
  #[cfg(not(feature = "mysql"))]
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

#[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
use crate::concat::non_standard::ConcatWith;
#[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
impl ConcatWith<DeleteClause> for Delete {}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
use crate::concat::non_standard::ConcatReturning;
#[cfg(any(feature = "postgresql", feature = "sqlite"))]
impl ConcatReturning<DeleteClause> for Delete {}

#[cfg(any(feature = "sqlite", feature = "mysql"))]
use crate::concat::sql_standard::ConcatOrderBy;
#[cfg(any(feature = "sqlite", feature = "mysql"))]
impl ConcatOrderBy<DeleteClause> for Delete {}

#[cfg(feature = "mysql")]
use crate::{
  concat::{
    mysql::ConcatPartition,
    non_standard::ConcatLimit,
    sql_standard::{ConcatFrom, ConcatJoin},
  },
  utils,
};
#[cfg(feature = "mysql")]
impl ConcatFrom<DeleteClause> for Delete {}
#[cfg(feature = "mysql")]
impl ConcatJoin<DeleteClause> for Delete {}
#[cfg(feature = "mysql")]
impl ConcatLimit<DeleteClause> for Delete {}
#[cfg(feature = "mysql")]
impl ConcatPartition<DeleteClause> for Delete {}

#[cfg(feature = "mysql")]
impl Delete {
  fn concat_delete_from_mysql(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;
    let delete_values = utils::join(&self._delete, comma);
    let from_values = utils::join(&self._from, comma);

    match (&self._delete_from, delete_values, from_values) {
      (del_from, del, from) if del_from.is_empty() == false && del.is_empty() == false && from.is_empty() == false => {
        let delete_clause = concat_raw_before_after(
          &self._raw_before,
          &self._raw_after,
          "".to_string(),
          fmts,
          DeleteClause::Delete,
          format!("DELETE{space}{del}{space}"),
        );

        let from_clause = concat_raw_before_after(
          &self._raw_before,
          &self._raw_after,
          "".to_string(),
          fmts,
          DeleteClause::From,
          format!("FROM{space}{del_from}{comma}{from}{space}{lb}"),
        );

        concat_raw_before_after(
          &self._raw_before,
          &self._raw_after,
          query,
          fmts,
          DeleteClause::DeleteFrom,
          format!("{delete_clause}{from_clause}"),
        )
      }
      (del_from, del, from) if del_from.is_empty() == false && del.is_empty() == false && from.is_empty() => {
        let delete_clause = concat_raw_before_after(
          &self._raw_before,
          &self._raw_after,
          "".to_string(),
          fmts,
          DeleteClause::Delete,
          format!("DELETE{space}{del}{space}"),
        );

        let sql = format!("{delete_clause}FROM{space}{del_from}{space}{lb}");

        concat_raw_before_after(
          &self._raw_before,
          &self._raw_after,
          query,
          fmts,
          DeleteClause::DeleteFrom,
          sql,
        )
      }
      (del_from, del, from) if del_from.is_empty() == false && del.is_empty() && from.is_empty() == false => {
        let from_clause = concat_raw_before_after(
          &self._raw_before,
          &self._raw_after,
          "".to_string(),
          fmts,
          DeleteClause::From,
          format!("FROM{space}{del_from}{comma}{from}{space}{lb}"),
        );

        concat_raw_before_after(
          &self._raw_before,
          &self._raw_after,
          query,
          fmts,
          DeleteClause::DeleteFrom,
          format!("DELETE{space}{from_clause}"),
        )
      }
      (del_from, del, from) if del_from.is_empty() == false && del.is_empty() && from.is_empty() => {
        let sql = format!("DELETE{space}FROM{space}{del_from}{space}{lb}");

        concat_raw_before_after(
          &self._raw_before,
          &self._raw_after,
          query,
          fmts,
          DeleteClause::DeleteFrom,
          sql,
        )
      }
      (del_from, del, from) if del_from.is_empty() && del.is_empty() == false && from.is_empty() == false => {
        let delete_clause = concat_raw_before_after(
          &self._raw_before,
          &self._raw_after,
          "".to_string(),
          fmts,
          DeleteClause::Delete,
          format!("DELETE{space}{del}{space}"),
        );

        let from_clause = concat_raw_before_after(
          &self._raw_before,
          &self._raw_after,
          "".to_string(),
          fmts,
          DeleteClause::From,
          format!("FROM{space}{from}{space}{lb}"),
        );

        format!("{delete_clause}{from_clause}")
      }
      (del_from, del, from) if del_from.is_empty() && del.is_empty() == false && from.is_empty() => {
        concat_raw_before_after(
          &self._raw_before,
          &self._raw_after,
          query,
          fmts,
          DeleteClause::Delete,
          format!("DELETE{space}{del}{space}"),
        )
      }
      (del_from, del, from) if del_from.is_empty() && del.is_empty() && from.is_empty() == false => {
        concat_raw_before_after(
          &self._raw_before,
          &self._raw_after,
          query,
          fmts,
          DeleteClause::From,
          format!("FROM{space}{from}{space}{lb}"),
        )
      }
      (_, _, _) => query,
    }
  }
}
