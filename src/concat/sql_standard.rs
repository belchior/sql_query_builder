use crate::{concat::concat_raw_before_after, fmt, structure::LogicalOperator, utils};

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
      let tables = utils::join(items, comma);
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

pub(crate) trait ConcatOrderBy<Clause: PartialEq> {
  fn concat_order_by(
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
      let columns = utils::join(items, comma);

      format!("ORDER BY{space}{columns}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(items_raw_before, items_raw_after, query, fmts, clause, sql)
  }
}

pub(crate) trait ConcatSet<Clause: PartialEq> {
  fn concat_set(
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
      let values = utils::join(items, comma);
      format!("SET{space}{values}{space}{lb}")
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
