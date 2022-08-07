use crate::{
  behavior::{concat_raw_before_after, Concat, ConcatMethods, WithQuery},
  fmt,
  structure::{InsertBuilder, InsertClause},
};

impl WithQuery for InsertBuilder<'_> {}

impl<'a> ConcatMethods<'a, InsertClause> for InsertBuilder<'_> {}

impl Concat for InsertBuilder<'_> {
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
        InsertClause::With,
        &self._with,
      );
    }
    query = self.concat_insert_into(query, &fmts);
    query = self.concat_overriding(query, &fmts);
    query = self.concat_values(
      &self._raw_before,
      &self._raw_after,
      query,
      &fmts,
      InsertClause::Values,
      &self._values,
    );
    query = self.concat_select(query, &fmts);
    query = self.concat_on_conflict(query, &fmts);

    #[cfg(feature = "postgresql")]
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

    query.trim_end().to_owned()
  }
}

impl InsertBuilder<'_> {
  fn concat_insert_into(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._insert_into.is_empty() == false {
      let insert_into = self._insert_into;
      format!("INSERT INTO{space}{insert_into}{space}{lb}")
    } else {
      "".to_owned()
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
      let overriding = self._overriding;
      format!("OVERRIDING{space}{overriding}{space}{lb}")
    } else {
      "".to_owned()
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
      let overriding = self._on_conflict;
      format!("ON CONFLICT{space}{overriding}{space}{lb}")
    } else {
      "".to_owned()
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
      "".to_owned()
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
}
