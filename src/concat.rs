use crate::{fmt::Formatter, structure::Clause, SelectBuilder};

impl SelectBuilder<'_> {
  pub(crate) fn concat(&self, fmts: &Formatter) -> String {
    let mut query = "".to_owned();

    query = self.concat_raw(query, &fmts);
    query = self.concat_with(query, &fmts);
    query = self.concat_select(query, &fmts);
    query = self.concat_from(query, &fmts);
    query = self.concat_join(query, &fmts);
    query = self.concat_where(query, &fmts);
    query = self.concat_group_by(query, &fmts);
    query = self.concat_having(query, &fmts);
    query = self.concat_order_by(query, &fmts);
    query = self.concat_limit(query, &fmts);
    query = self.concat_offset(query, &fmts);
    query = self.concat_union(query, &fmts);
    query = self.concat_except(query, &fmts);
    query = self.concat_intersect(query, &fmts);

    query.trim_end().to_owned()
  }

  fn concat_except(&self, query: String, fmts: &Formatter) -> String {
    let Formatter { lb, space, .. } = fmts;
    let sql = if self._except.is_empty() == false {
      let excepts_string = self._except.iter().fold("".to_owned(), |acc, select| {
        let select_string = select.concat(&fmts);
        format!("{acc}EXCEPT{space}{lb}{select_string}{space}{lb}")
      });

      format!("{excepts_string}")
    } else {
      "".to_owned()
    };

    self.concat_clause(query, fmts, Clause::Except, sql)
  }

  fn concat_from(&self, query: String, fmts: &Formatter) -> String {
    let Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._from.is_empty() == false {
      let tables = self._from.join(comma);
      format!("FROM {tables}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_clause(query, fmts, Clause::From, sql)
  }

  fn concat_join(&self, query: String, fmts: &Formatter) -> String {
    let Formatter { lb, space, .. } = fmts;
    let sql = if self._join.is_empty() == false {
      let joins = self._join.join(format!("{space}{lb}").as_str());
      format!("{joins}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_clause(query, fmts, Clause::Join, sql)
  }

  fn concat_group_by(&self, query: String, fmts: &Formatter) -> String {
    let Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._group_by.is_empty() == false {
      let columns = self._group_by.join(comma);
      format!("GROUP BY {columns}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_clause(query, fmts, Clause::GroupBy, sql)
  }

  fn concat_having(&self, query: String, fmts: &Formatter) -> String {
    let Formatter { lb, space, .. } = fmts;
    let sql = if self._having.is_empty() == false {
      let conditions = self._having.join(" AND ");
      format!("HAVING {conditions}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_clause(query, fmts, Clause::Having, sql)
  }

  fn concat_intersect(&self, query: String, fmts: &Formatter) -> String {
    let Formatter { lb, space, .. } = fmts;
    let sql = if self._intersect.is_empty() == false {
      let intersects_string = self._intersect.iter().fold("".to_owned(), |acc, select| {
        let select_string = select.concat(&fmts);
        format!("{acc}INTERSECT{space}{lb}{select_string}{space}{lb}")
      });

      format!("{intersects_string}")
    } else {
      "".to_owned()
    };

    self.concat_clause(query, fmts, Clause::Intersect, sql)
  }

  fn concat_limit(&self, query: String, fmts: &Formatter) -> String {
    let Formatter { lb, space, .. } = fmts;
    let sql = if self._limit.is_empty() == false {
      let count = self._limit;
      format!("LIMIT {count}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_clause(query, fmts, Clause::Limit, sql)
  }

  fn concat_offset(&self, query: String, fmts: &Formatter) -> String {
    let Formatter { lb, space, .. } = fmts;
    let sql = if self._offset.is_empty() == false {
      let start = self._offset;
      format!("OFFSET {start}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_clause(query, fmts, Clause::Offset, sql)
  }

  fn concat_order_by(&self, query: String, fmts: &Formatter) -> String {
    let Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._order_by.is_empty() == false {
      let columns = self._order_by.join(comma);
      format!("ORDER BY {columns}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_clause(query, fmts, Clause::OrderBy, sql)
  }

  fn concat_raw(&self, query: String, fmts: &Formatter) -> String {
    if self._raw.is_empty() {
      return query;
    }
    let Formatter { lb, space, .. } = fmts;
    let raw_sql = self._raw.join(space);

    format!("{query}{raw_sql}{space}{lb}")
  }

  fn concat_select(&self, query: String, fmts: &Formatter) -> String {
    let Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._select.is_empty() == false {
      let columns = self._select.join(comma);
      format!("SELECT {columns}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_clause(query, fmts, Clause::Select, sql)
  }

  fn concat_clause(&self, query: String, fmts: &Formatter, clause: Clause, sql: String) -> String {
    let Formatter { space, .. } = fmts;
    let raw_after = self.queries_after(clause).join(space);
    let raw_before = self.queries_before(clause).join(space);
    let space_after = if raw_after.is_empty() == false { space } else { "" };
    let space_before = if raw_before.is_empty() == false { space } else { "" };

    format!("{query}{raw_before}{space_before}{sql}{raw_after}{space_after}")
  }

  fn concat_union(&self, query: String, fmts: &Formatter) -> String {
    let Formatter { lb, space, .. } = fmts;
    let sql = if self._union.is_empty() == false {
      let unions_string = self._union.iter().fold("".to_owned(), |acc, select| {
        let select_string = select.concat(&fmts);
        format!("{acc}UNION{space}{lb}{select_string}{space}{lb}")
      });

      format!("{unions_string}")
    } else {
      "".to_owned()
    };

    self.concat_clause(query, fmts, Clause::Union, sql)
  }

  fn concat_where(&self, query: String, fmts: &Formatter) -> String {
    let Formatter { lb, space, .. } = fmts;
    let sql = if self._where.is_empty() == false {
      let conditions = self._where.join(" AND ");
      format!("WHERE {conditions}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_clause(query, fmts, Clause::Where, sql)
  }

  fn concat_with(&self, query: String, fmts: &Formatter) -> String {
    let Formatter {
      comma,
      lb,
      indent,
      space,
    } = fmts;
    let sql = if self._with.is_empty() == false {
      let with = self._with.iter().fold("".to_owned(), |acc, item| {
        let (name, select) = item;
        let inner_lb = format!("{lb}{indent}");
        let inner_fmts = Formatter {
          comma,
          lb: inner_lb.as_str(),
          indent,
          space,
        };
        let select_string = select.concat(&inner_fmts);

        format!("{acc}{name} AS ({lb}{indent}{select_string}{lb}){comma}")
      });
      let with = &with[..with.len() - comma.len()];

      format!("WITH {with}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_clause(query, fmts, Clause::With, sql)
  }

  fn queries_after(&self, clause: Clause) -> Vec<String> {
    self
      ._raw_after
      .iter()
      .filter(|item| item.0 == clause)
      .map(|item| item.1.clone())
      .collect::<Vec<_>>()
  }

  fn queries_before(&self, clause: Clause) -> Vec<String> {
    self
      ._raw_before
      .iter()
      .filter(|item| item.0 == clause)
      .map(|item| item.1.clone())
      .collect::<Vec<_>>()
  }
}
