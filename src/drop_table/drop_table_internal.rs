use crate::{
  behavior::{concat_raw_before_after, Concat, ConcatSqlStandard},
  fmt,
  structure::{DropTable, DropTableParams},
};

impl ConcatSqlStandard<DropTableParams> for DropTable {}

impl DropTable {
  fn concat_drop_table(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;

    let sql = if self._drop_table.len() != 0 {
      let if_exists = if self._if_exists {
        format!("IF EXISTS{space}")
      } else {
        "".to_string()
      };

      let table_names = if cfg!(any(feature = "postgresql")) {
        self._drop_table.join(comma)
      } else {
        self._drop_table.last().unwrap().to_string()
      };

      format!("DROP TABLE{space}{if_exists}{table_names}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      DropTableParams::DropTable,
      sql,
    )
  }
}

impl Concat for DropTable {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_string();

    query = self.concat_raw(query, &fmts, &self._raw);
    query = self.concat_drop_table(query, &fmts);

    query.trim_end().to_string()
  }
}
