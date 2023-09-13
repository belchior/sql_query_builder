#[cfg(any(feature = "postgresql", feature = "sqlite"))]
use crate::behavior::ConcatCommon;
use crate::{
  behavior::{concat_raw_before_after, Concat, ConcatSqlStandard},
  fmt,
  structure::{Select, SelectClause},
};

impl ConcatSqlStandard<SelectClause> for Select {}

impl Concat for Select {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_owned();

    query = self.concat_raw(query, &fmts, &self._raw);

    #[cfg(any(feature = "postgresql", feature = "sqlite"))]
    {
      query = self.concat_with(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        SelectClause::With,
        &self._with,
      );
    }
    query = self.concat_select(query, &fmts);
    query = self.concat_from(
      &self._raw_before,
      &self._raw_after,
      query,
      &fmts,
      SelectClause::From,
      &self._from,
    );
    query = self.concat_join(query, &fmts);
    query = self.concat_where(
      &self._raw_before,
      &self._raw_after,
      query,
      &fmts,
      SelectClause::Where,
      &self._where,
    );
    query = self.concat_group_by(query, &fmts);
    query = self.concat_having(query, &fmts);
    query = self.concat_order_by(query, &fmts);

    #[cfg(any(feature = "postgresql", feature = "sqlite"))]
    {
      query = self.concat_limit(query, &fmts);
      query = self.concat_offset(query, &fmts);
    }

    #[cfg(any(feature = "postgresql", feature = "sqlite"))]
    {
      use crate::structure::Combinator;
      query = self.concat_combinator(query, &fmts, Combinator::Except);
      query = self.concat_combinator(query, &fmts, Combinator::Intersect);
      query = self.concat_combinator(query, &fmts, Combinator::Union);
    }

    query.trim_end().to_owned()
  }
}

impl Select {
  fn concat_group_by(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._group_by.is_empty() == false {
      let columns = self._group_by.join(comma);
      format!("GROUP BY{space}{columns}{space}{lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      SelectClause::GroupBy,
      sql,
    )
  }

  fn concat_having(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._having.is_empty() == false {
      let conditions = self._having.join(" AND ");
      format!("HAVING{space}{conditions}{space}{lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      SelectClause::Having,
      sql,
    )
  }

  fn concat_join(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._join.is_empty() == false {
      let joins = self._join.join(format!("{space}{lb}").as_str());
      format!("{joins}{space}{lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      SelectClause::Join,
      sql,
    )
  }

  fn concat_order_by(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._order_by.is_empty() == false {
      let columns = self._order_by.join(comma);
      format!("ORDER BY{space}{columns}{space}{lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      SelectClause::OrderBy,
      sql,
    )
  }

  fn concat_select(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._select.is_empty() == false {
      let columns = self._select.join(comma);
      format!("SELECT{space}{columns}{space}{lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      SelectClause::Select,
      sql,
    )
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
impl ConcatCommon<SelectClause> for Select {}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
impl Select {
  fn concat_combinator(
    &self,
    query: String,
    fmts: &fmt::Formatter,
    combinator: crate::structure::Combinator,
  ) -> String {
    use crate::behavior::raw_queries;
    use crate::structure::Combinator;

    let fmt::Formatter { lb, space, .. } = fmts;
    let (clause, clause_name, clause_list) = match combinator {
      Combinator::Except => (SelectClause::Except, "EXCEPT", &self._except),
      Combinator::Intersect => (SelectClause::Intersect, "INTERSECT", &self._intersect),
      Combinator::Union => (SelectClause::Union, "UNION", &self._union),
    };

    let raw_before = raw_queries(&self._raw_before, &clause).join(space);
    let raw_after = raw_queries(&self._raw_after, &clause).join(space);

    let space_before = if raw_before.is_empty() {
      "".to_owned()
    } else {
      space.to_string()
    };
    let space_after = if raw_after.is_empty() {
      "".to_owned()
    } else {
      space.to_string()
    };

    if clause_list.is_empty() {
      let sql = "".to_owned();
      return format!("{query}{raw_before}{space_before}{sql}{raw_after}{space_after}");
    }

    let right_stmt = clause_list.iter().fold("".to_owned(), |acc, select| {
      let query = select.concat(&fmts);
      format!("{acc}{clause_name}{space}({lb}{query}){space}{lb}")
    });

    let query = query.trim_end();
    let space_before = space;
    let left_stmt = format!("({query}{raw_before}){space_before}");

    format!("{left_stmt}{right_stmt}{raw_after}{space_after}")
  }

  fn concat_limit(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._limit.is_empty() == false {
      let count = &self._limit;
      format!("LIMIT{space}{count}{space}{lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      SelectClause::Limit,
      sql,
    )
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
impl Select {
  fn concat_offset(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._offset.is_empty() == false {
      let start = &self._offset;
      format!("OFFSET{space}{start}{space}{lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      SelectClause::Offset,
      sql,
    )
  }
}
