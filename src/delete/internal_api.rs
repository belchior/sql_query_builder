use crate::{
  behavior::{concat_raw_before_after, Concat, ConcatMethods, WithQuery},
  fmt,
  structure::{DeleteBuilder, DeleteClause},
};

impl WithQuery for DeleteBuilder<'_> {}

impl<'a> ConcatMethods<'a, DeleteClause> for DeleteBuilder<'_> {}

impl Concat for DeleteBuilder<'_> {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_owned();

    query = self.concat_raw(query, &fmts, &self._raw);
    #[cfg(feature = "postgresql")]
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
    #[cfg(feature = "postgresql")]
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

    query.trim_end().to_owned()
  }
}

impl DeleteBuilder<'_> {
  fn concat_delete_from(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._delete_from.is_empty() == false {
      let table_name = self._delete_from;
      format!("DELETE FROM{space}{table_name}{space}{lb}")
    } else {
      "".to_owned()
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
