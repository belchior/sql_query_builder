use crate::{
  behavior::{concat_raw_before_after, Concat, ConcatSqlStandard},
  fmt,
  structure::{DropIndex, DropIndexParams},
};

impl ConcatSqlStandard<DropIndexParams> for DropIndex {}

impl DropIndex {
  fn concat_drop_index(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;

    let sql = if self._drop_index.len() != 0 {
      let if_exists = if self._if_exists {
        format!("IF EXISTS{space}")
      } else {
        "".to_string()
      };

      let index_names = if cfg!(any(feature = "postgresql")) {
        self._drop_index.join(comma)
      } else {
        self._drop_index.last().unwrap().to_string()
      };

      format!("DROP INDEX{space}{if_exists}{index_names}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      DropIndexParams::DropIndex,
      sql,
    )
  }
}

impl Concat for DropIndex {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_string();

    query = self.concat_raw(query, &fmts, &self._raw);
    query = self.concat_drop_index(query, &fmts);

    query.trim_end().to_string()
  }
}
