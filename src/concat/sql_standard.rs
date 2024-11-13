use crate::{concat::concat_raw_before_after, fmt, structure::LogicalOperator};

pub(crate) trait ConcatFrom<Clause: PartialEq> {
  fn concat_from(
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
      let tables = items
        .iter()
        .filter(|item| item.is_empty() == false)
        .map(|item| item.as_str())
        .collect::<Vec<_>>()
        .join(comma);
      format!("FROM{space}{tables}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(items_raw_before, items_raw_after, query, fmts, clause, sql)
  }
}

pub(crate) trait ConcatJoin<Clause: PartialEq> {
  fn concat_join(
    &self,
    items_raw_before: &Vec<(Clause, String)>,
    items_raw_after: &Vec<(Clause, String)>,
    query: String,
    fmts: &fmt::Formatter,
    clause: Clause,
    items: &Vec<String>,
  ) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if items.is_empty() == false {
      let joins = items.join(format!("{space}{lb}").as_str());
      format!("{joins}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(items_raw_before, items_raw_after, query, fmts, clause, sql)
  }
}

pub(crate) trait ConcatWhere<Clause: PartialEq> {
  fn concat_where(
    &self,
    items_raw_before: &Vec<(Clause, String)>,
    items_raw_after: &Vec<(Clause, String)>,
    query: String,
    fmts: &fmt::Formatter,
    clause: Clause,
    items: &Vec<(LogicalOperator, String)>,
  ) -> String {
    let fmt::Formatter { lb, space, indent, .. } = fmts;
    let sql = if items.is_empty() == false {
      let filtered_items = items
        .iter()
        .filter(|item| item.1.is_empty() == false)
        .collect::<Vec<_>>();
      let ((_, cond), tail) = filtered_items.split_first().unwrap();
      let first_condition = format!("{indent}{cond}");
      let conditions = tail.iter().fold(first_condition, |acc, (log_op, condition)| {
        format!("{acc}{space}{lb}{indent}{log_op}{space}{condition}")
      });

      format!("WHERE{lb}{space}{conditions}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(items_raw_before, items_raw_after, query, fmts, clause, sql)
  }
}
