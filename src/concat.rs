use crate::{fmt::Formatter, SelectBuilder};

impl SelectBuilder<'_> {
  pub(crate) fn concat(&self, fmts: &Formatter) -> String {
    let mut query = "".to_owned();

    query = self.concat_raw(query, &fmts);
    query = self.concat_with(query, &fmts);
    query = self.concat_select(query, &fmts);
    query = self.concat_from(query, &fmts);
    query = self.concat_join(query, &fmts);
    query = self.concat_where(query, &fmts);
    query = self.concat_order_by(query, &fmts);
    query = self.concat_limit(query, &fmts);
    query = self.concat_union(query, &fmts);

    query.trim_end().to_owned()
  }

  fn concat_from(&self, query: String, fmts: &Formatter) -> String {
    if self._from.is_empty() {
      return query;
    }
    let Formatter { comma, sep, .. } = fmts;
    let tables = self._from.join(comma);

    format!("{query}FROM {tables}{sep}")
  }

  fn concat_join(&self, query: String, fmts: &Formatter) -> String {
    if self._join.is_empty() {
      return query;
    }
    let Formatter { sep, .. } = fmts;
    let joins = self._join.join(sep);
    format!("{query}{joins}{sep}")
  }

  fn concat_limit(&self, query: String, fmts: &Formatter) -> String {
    if self._limit.is_empty() {
      return query;
    }
    let limit = self._limit;
    let Formatter { sep, .. } = fmts;

    format!("{query}LIMIT {limit}{sep}")
  }

  fn concat_order_by(&self, query: String, fmts: &Formatter) -> String {
    if self._order_by.is_empty() {
      return query;
    }
    let Formatter { sep, comma, .. } = fmts;
    let columns = self._order_by.join(comma);

    format!("{query}ORDER BY {columns}{sep}")
  }

  fn concat_raw(&self, query: String, fmts: &Formatter) -> String {
    if self._raw.is_empty() {
      return query;
    }
    let Formatter { sep, .. } = fmts;
    let raw_sql = self._raw.join(sep);

    format!("{query}{raw_sql}{sep}")
  }

  fn concat_select(&self, query: String, fmts: &Formatter) -> String {
    if self._select.is_empty() {
      return query;
    }
    let Formatter { sep, comma, .. } = fmts;
    let columns = self._select.join(comma);

    format!("{query}SELECT {columns}{sep}")
  }

  fn concat_union(&self, query: String, fmts: &Formatter) -> String {
    if self._union.is_empty() {
      return query;
    }

    let Formatter { sep, .. } = fmts;
    let unions_string = self._union.iter().fold("".to_owned(), |acc, select| {
      let select_string = select.concat(&fmts);

      format!("{acc}UNION{sep}{select_string}{sep}")
    });

    format!("{query}{unions_string}{sep}")
  }

  fn concat_where(&self, query: String, fmts: &Formatter) -> String {
    if self._where.is_empty() {
      return query;
    }
    let Formatter { sep, .. } = fmts;
    let clauses = self._where.join(" AND ");

    format!("{query}WHERE {clauses}{sep}")
  }

  fn concat_with(&self, query: String, fmts: &Formatter) -> String {
    if self._with.is_empty() {
      return query;
    }

    let Formatter { sep, indent, comma } = fmts;
    let with = self._with.iter().fold("".to_owned(), |acc, item| {
      let (name, select) = item;
      let inner_sep = format!("{sep}{indent}");
      let inner_fmts = Formatter {
        indent,
        sep: inner_sep.as_str(),
        comma,
      };
      let select_string = select.concat(&inner_fmts);

      format!("{acc}{name} AS ({sep}{indent}{select_string}{sep}){comma}")
    });
    let with = &with[..with.len() - comma.len()];

    format!("{query}WITH {with}{sep}")
  }
}
