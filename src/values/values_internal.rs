use crate::{
  concat::{concat_raw_before_after, Concat},
  fmt,
  structure::{Values, ValuesClause},
};

impl Concat for Values {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_string();

    query = self.concat_raw(query, &fmts, &self._raw);
    query = self.concat_values(query, &fmts);

    query.trim_end().to_string()
  }
}

impl Values {
  fn concat_values(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._values.is_empty() == false {
      let sep = format!("{comma}{lb}");
      let values = self
        ._values
        .iter()
        .filter(|item| item.is_empty() == false)
        .map(|item| item.as_str())
        .collect::<Vec<_>>()
        .join(&sep);
      format!("VALUES{space}{lb}{values}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      ValuesClause::Values,
      sql,
    )
  }
}
