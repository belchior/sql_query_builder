#[cfg(any(feature = "postgresql", feature = "sqlite"))]
use crate::behavior::ConcatCommon;
#[cfg(feature = "sqlite")]
use crate::behavior::ConcatSqlite;
use crate::{
  behavior::{concat_raw_before_after, Concat, ConcatSqlStandard},
  fmt,
  structure::{Insert, InsertClause},
};

impl ConcatSqlStandard<InsertClause> for Insert {}

impl Insert {
  #[cfg(not(feature = "sqlite"))]
  fn concat_insert_into(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._insert_into.is_empty() == false {
      let insert_into = &self._insert_into;
      format!("INSERT INTO{space}{insert_into}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      InsertClause::InsertInto,
      sql,
    )
  }

  fn concat_overriding(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._overriding.is_empty() == false {
      let overriding = &self._overriding;
      format!("OVERRIDING{space}{overriding}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      InsertClause::Overriding,
      sql,
    )
  }

  fn concat_on_conflict(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._on_conflict.is_empty() == false {
      let overriding = &self._on_conflict;
      format!("ON CONFLICT{space}{overriding}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      InsertClause::OnConflict,
      sql,
    )
  }

  fn concat_select(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if let Some(select) = &self._select {
      let select_string = select.concat(fmts);
      format!("{select_string}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      InsertClause::Select,
      sql,
    )
  }

  fn concat_values(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;

    let (clause, sql) = if self._default_values {
      (InsertClause::DefaultValues, format!("DEFAULT VALUES{space}{lb}"))
    } else if self._values.is_empty() == false {
      let sep = format!("{comma}{lb}");
      let values = self._values.join(&sep);
      (InsertClause::Values, format!("VALUES{space}{lb}{values}{space}{lb}"))
    } else {
      (InsertClause::Values, "".to_string())
    };

    concat_raw_before_after(&self._raw_before, &self._raw_after, query, fmts, clause, sql)
  }
}

impl Concat for Insert {
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
        InsertClause::With,
        &self._with,
      );
    }

    #[cfg(not(feature = "sqlite"))]
    {
      query = self.concat_insert_into(query, &fmts);
    }
    #[cfg(feature = "sqlite")]
    {
      query = ConcatSqlite::concat_insert(self, &self._raw_before, &self._raw_after, query, &fmts, &self._insert);
    }

    query = self.concat_overriding(query, &fmts);

    query = self.concat_values(query, &fmts);

    query = self.concat_select(query, &fmts);

    query = self.concat_on_conflict(query, &fmts);

    #[cfg(any(feature = "postgresql", feature = "sqlite"))]
    {
      query = self.concat_returning(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        InsertClause::Returning,
        &self._returning,
      );
    }

    query.trim_end().to_string()
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
impl ConcatCommon<InsertClause> for Insert {}

#[cfg(feature = "sqlite")]
impl ConcatSqlite for Insert {}
