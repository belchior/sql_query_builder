#[cfg(any(feature = "postgresql", feature = "sqlite"))]
use crate::{behavior::WithQuery, concat::concat_raw_before_after, fmt};

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
pub(crate) trait ConcatLimit<Clause: PartialEq> {
  fn concat_limit(
    &self,
    items_raw_before: &Vec<(Clause, String)>,
    items_raw_after: &Vec<(Clause, String)>,
    query: String,
    fmts: &fmt::Formatter,
    clause: Clause,
    limit: &str,
  ) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if limit.is_empty() == false {
      format!("LIMIT{space}{limit}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(items_raw_before, items_raw_after, query, fmts, clause, sql)
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
pub(crate) trait ConcatReturning<Clause: PartialEq> {
  fn concat_returning(
    &self,
    items_raw_before: &Vec<(Clause, String)>,
    items_raw_after: &Vec<(Clause, String)>,
    query: String,
    fmts: &fmt::Formatter,
    clause: Clause,
    items: &Vec<String>,
  ) -> String {
    let fmt::Formatter { lb, space, comma, .. } = fmts;
    let sql = if items.is_empty() == false {
      let output_names = items
        .iter()
        .filter(|item| item.is_empty() == false)
        .map(|item| item.as_str())
        .collect::<Vec<_>>()
        .join(comma);
      format!("RETURNING{space}{output_names}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(items_raw_before, items_raw_after, query, fmts, clause, sql)
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
pub(crate) trait ConcatWith<Clause: PartialEq> {
  fn concat_with(
    &self,
    items_raw_before: &Vec<(Clause, String)>,
    items_raw_after: &Vec<(Clause, String)>,
    query: String,
    fmts: &fmt::Formatter,
    clause: Clause,
    items: &Vec<(String, std::sync::Arc<dyn WithQuery + Send + Sync>)>,
  ) -> String {
    let fmt::Formatter {
      comma,
      lb,
      indent,
      space,
      ..
    } = fmts;
    let sql = if items.is_empty() == false {
      let with = items.iter().fold("".to_string(), |acc, item| {
        let (name, query) = item;
        let inner_lb = format!("{lb}{indent}");
        let inner_fmts = fmt::Formatter {
          comma,
          lb: inner_lb.as_str(),
          indent,
          space,
          ..*fmts
        };
        let query_string = query.concat(&inner_fmts);

        if query_string.is_empty() == false {
          format!("{acc}{name}{space}AS{space}({lb}{indent}{query_string}{lb}){comma}{lb}")
        } else {
          acc
        }
      });
      let with = &with[..with.len() - comma.len() - lb.len()];

      format!("WITH{space}{lb}{with}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(items_raw_before, items_raw_after, query, fmts, clause, sql)
  }
}
