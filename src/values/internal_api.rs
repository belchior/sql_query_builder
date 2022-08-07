use crate::{
  behavior::{Concat, ConcatMethods},
  fmt,
  structure::{ValuesBuilder, ValuesClause},
};

impl<'a> ConcatMethods<'a, ValuesClause> for ValuesBuilder {}

impl Concat for ValuesBuilder {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_owned();

    query = self.concat_raw(query, &fmts, &self._raw);
    query = self.concat_values(
      &self._raw_before,
      &self._raw_after,
      query,
      &fmts,
      ValuesClause::Values,
      &self._values,
    );

    query.trim_end().to_owned()
  }
}
