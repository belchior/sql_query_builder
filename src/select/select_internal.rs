use crate::{
  concat::{
    concat_raw_before_after,
    sql_standard::{ConcatFrom, ConcatJoin, ConcatOrderBy, ConcatWhere},
    Concat,
  },
  fmt,
  structure::{Select, SelectClause},
  utils,
};

impl ConcatFrom<SelectClause> for Select {}
impl ConcatWhere<SelectClause> for Select {}
impl ConcatJoin<SelectClause> for Select {}
impl ConcatOrderBy<SelectClause> for Select {}

impl Concat for Select {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_string();

    #[cfg(not(any(feature = "postgresql", feature = "sqlite", feature = "mysql")))]
    {
      query = self.concat_raw(query, &fmts, &self._raw);
      query = self.concat_select(query, &fmts);
      query = self.concat_from(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        SelectClause::From,
        &self._from,
      );
      query = self.concat_join(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        SelectClause::Join,
        &self._join,
      );
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
      query = self.concat_window(query, &fmts);
      query = self.concat_order_by(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        SelectClause::OrderBy,
        &self._order_by,
      );
    }

    #[cfg(feature = "postgresql")]
    {
      use crate::structure::Combinator;

      query = self.concat_raw(query, &fmts, &self._raw);
      query = self.concat_with(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        SelectClause::With,
        &self._with,
      );
      query = self.concat_select(query, &fmts);
      query = self.concat_from(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        SelectClause::From,
        &self._from,
      );
      query = self.concat_join(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        SelectClause::Join,
        &self._join,
      );
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
      query = self.concat_window(query, &fmts);
      query = self.concat_order_by(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        SelectClause::OrderBy,
        &self._order_by,
      );
      query = self.concat_limit(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        SelectClause::Limit,
        &self._limit,
      );
      query = self.concat_offset(query, &fmts);
      query = self.concat_combinator(query, &fmts, Combinator::Except);
      query = self.concat_combinator(query, &fmts, Combinator::Intersect);
      query = self.concat_combinator(query, &fmts, Combinator::Union);
    }

    #[cfg(feature = "sqlite")]
    {
      use crate::structure::Combinator;

      query = self.concat_raw(query, &fmts, &self._raw);
      query = self.concat_with(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        SelectClause::With,
        &self._with,
      );
      query = self.concat_select(query, &fmts);
      query = self.concat_from(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        SelectClause::From,
        &self._from,
      );
      query = self.concat_join(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        SelectClause::Join,
        &self._join,
      );
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
      query = self.concat_window(query, &fmts);
      query = self.concat_order_by(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        SelectClause::OrderBy,
        &self._order_by,
      );
      query = self.concat_limit(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        SelectClause::Limit,
        &self._limit,
      );
      query = self.concat_offset(query, &fmts);
      query = self.concat_combinator(query, &fmts, Combinator::Except);
      query = self.concat_combinator(query, &fmts, Combinator::Intersect);
      query = self.concat_combinator(query, &fmts, Combinator::Union);
    }

    #[cfg(feature = "mysql")]
    {
      use crate::structure::Combinator;

      query = self.concat_raw(query, &fmts, &self._raw);
      query = self.concat_with(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        SelectClause::With,
        &self._with,
      );
      query = self.concat_select(query, &fmts);
      query = self.concat_from(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        SelectClause::From,
        &self._from,
      );
      query = self.concat_join(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        SelectClause::Join,
        &self._join,
      );
      query = self.concat_partition(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        SelectClause::Partition,
        &self._partition,
      );
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
      query = self.concat_window(query, &fmts);
      query = self.concat_order_by(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        SelectClause::OrderBy,
        &self._order_by,
      );
      query = self.concat_limit(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        SelectClause::Limit,
        &self._limit,
      );
      query = self.concat_offset(query, &fmts);
      query = self.concat_combinator(query, &fmts, Combinator::Except);
      query = self.concat_combinator(query, &fmts, Combinator::Intersect);
      query = self.concat_combinator(query, &fmts, Combinator::Union);
    }

    query.trim_end().to_string()
  }
}

impl Select {
  fn concat_group_by(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._group_by.is_empty() == false {
      let columns = utils::join(&self._group_by, comma);
      format!("GROUP BY{space}{columns}{space}{lb}")
    } else {
      "".to_string()
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
      let conditions = utils::join(&self._having, " AND ");
      format!("HAVING{space}{conditions}{space}{lb}")
    } else {
      "".to_string()
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

  fn concat_select(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._select.is_empty() == false {
      let columns = utils::join(&self._select, comma);
      format!("SELECT{space}{columns}{space}{lb}")
    } else {
      "".to_string()
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

  fn concat_window(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._window.is_empty() == false {
      let columns = utils::join(&self._window, comma);
      format!("WINDOW{space}{columns}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      SelectClause::Window,
      sql,
    )
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
use crate::concat::non_standard::{ConcatLimit, ConcatWith};

#[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
impl ConcatWith<SelectClause> for Select {}
#[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
impl ConcatLimit<SelectClause> for Select {}

#[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
impl Select {
  fn concat_combinator(
    &self,
    query: String,
    fmts: &fmt::Formatter,
    combinator: crate::structure::Combinator,
  ) -> String {
    use crate::{concat::raw_queries, structure::Combinator};

    let fmt::Formatter { lb, space, .. } = fmts;
    let (clause, clause_name, clause_list) = match combinator {
      Combinator::Except => (SelectClause::Except, "EXCEPT", &self._except),
      Combinator::Intersect => (SelectClause::Intersect, "INTERSECT", &self._intersect),
      Combinator::Union => (SelectClause::Union, "UNION", &self._union),
    };

    let raw_before = raw_queries(&self._raw_before, &clause).join(space).trim().to_string();
    let raw_after = raw_queries(&self._raw_after, &clause).join(space).trim().to_string();

    let space_before = if raw_before.is_empty() {
      "".to_string()
    } else {
      space.to_string()
    };
    let space_after = if raw_after.is_empty() {
      "".to_string()
    } else {
      space.to_string()
    };

    if clause_list.is_empty() {
      let sql = "".to_string();
      return format!("{query}{raw_before}{space_before}{sql}{raw_after}{space_after}");
    }

    let right_stmt = clause_list.iter().fold("".to_string(), |acc, select| {
      let query = select.concat(&fmts);
      format!("{acc}{clause_name}{space}({lb}{query}){space}{lb}")
    });

    let query = query.trim_end();
    let space_before = space;
    let left_stmt = format!("({query}{raw_before}){space_before}");

    format!("{left_stmt}{right_stmt}{raw_after}{space_after}")
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
impl Select {
  fn concat_offset(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._offset.is_empty() == false {
      let start = &self._offset;
      format!("OFFSET{space}{start}{space}{lb}")
    } else {
      "".to_string()
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

#[cfg(feature = "mysql")]
use crate::concat::mysql::ConcatPartition;

#[cfg(feature = "mysql")]
impl ConcatPartition<SelectClause> for Select {}
