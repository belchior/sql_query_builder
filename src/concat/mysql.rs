#[cfg(feature = "mysql")]
use crate::{concat::concat_raw_before_after, fmt, utils};

#[cfg(feature = "mysql")]
pub(crate) trait ConcatPartition<Clause: PartialEq> {
  fn concat_partition(
    &self,
    items_raw_before: &Vec<(Clause, String)>,
    items_raw_after: &Vec<(Clause, String)>,
    query: String,
    fmts: &fmt::Formatter,
    clause: Clause,
    items: &Vec<String>,
  ) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;

    let sql = if items.is_empty() == false {
      let column_names = utils::join(items, comma);

      if column_names.is_empty() == false {
        format!("PARTITION{space}({column_names}){space}{lb}")
      } else {
        "".to_string()
      }
    } else {
      "".to_string()
    };

    concat_raw_before_after(items_raw_before, items_raw_after, query, fmts, clause, sql)
  }
}
