use crate::{
  concat::{concat_raw_before_after, sql_standard::ConcatWhere, Concat},
  fmt,
  structure::{CreateIndex, CreateIndexParams},
};

impl ConcatWhere<CreateIndexParams> for CreateIndex {}

impl Concat for CreateIndex {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_string();

    query = self.concat_raw(query, &fmts, &self._raw);

    #[cfg(feature = "postgresql")]
    {
      query = self.concat_create_index_postgres(query, &fmts);
      query = self.concat_on_postgres(query, &fmts);
      query = self.concat_using(query, &fmts);
    }

    #[cfg(feature = "sqlite")]
    {
      query = self.concat_create_index_sqlite(query, &fmts);
      query = self.concat_on_sqlite(query, &fmts);
    }

    query = self.concat_column(query, &fmts);

    #[cfg(feature = "postgresql")]
    {
      query = self.concat_include(query, &fmts);
    }

    query = self.concat_where(
      &self._raw_before,
      &self._raw_after,
      query,
      &fmts,
      CreateIndexParams::Where,
      &self._where,
    );

    query.trim_end().to_string()
  }
}

impl CreateIndex {
  fn concat_column(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, comma, space, .. } = fmts;

    let sql = if self._column.is_empty() == false {
      let column_names = self
        ._column
        .iter()
        .filter(|column| column.is_empty() == false)
        .map(|column| column.as_str())
        .collect::<Vec<_>>()
        .join(comma);

      if column_names.is_empty() == false {
        format!("({column_names}){space}{lb}")
      } else {
        "".to_string()
      }
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      CreateIndexParams::Column,
      sql,
    )
  }
}

#[cfg(feature = "postgresql")]
impl CreateIndex {
  fn concat_create_index_postgres(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;

    let unique = if self._unique {
      concat_raw_before_after(
        &self._raw_before,
        &self._raw_after,
        "".to_string(),
        fmts,
        CreateIndexParams::Unique,
        format!("UNIQUE{space}"),
      )
    } else {
      "".to_string()
    };

    let if_not_exists = if self._if_not_exists {
      format!("IF NOT EXISTS{space}")
    } else {
      "".to_string()
    };

    let index_name = if self._index_name.is_empty() == false {
      format!("{}{space}", &self._index_name)
    } else {
      "".to_string()
    };

    let concurrently = if self._concurrently {
      concat_raw_before_after(
        &self._raw_before,
        &self._raw_after,
        "".to_string(),
        fmts,
        CreateIndexParams::Concurrently,
        format!("CONCURRENTLY{space}"),
      )
    } else {
      "".to_string()
    };

    let modifiers_not_called = self._create_index == false && unique.is_empty() && concurrently.is_empty();
    let if_not_exists_without_index_name = self._if_not_exists && index_name.is_empty();

    let sql = if modifiers_not_called || if_not_exists_without_index_name {
      "".to_string()
    } else {
      format!("CREATE{space}{unique}INDEX{space}{concurrently}{if_not_exists}{index_name}{lb}")
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      CreateIndexParams::CreateIndex,
      sql,
    )
  }

  fn concat_include(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;

    let sql = if self._include.is_empty() == false {
      let column_names = self
        ._include
        .iter()
        .filter(|column| column.is_empty() == false)
        .map(|column| column.as_str())
        .collect::<Vec<_>>()
        .join(comma);

      if column_names.is_empty() == false {
        format!("INCLUDE{space}({column_names}){space}{lb}")
      } else {
        "".to_string()
      }
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      CreateIndexParams::Include,
      sql,
    )
  }

  fn concat_on_postgres(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { space, .. } = fmts;

    let sql = if self._on.is_empty() == false {
      let table_name = &self._on;

      let only = if self._only {
        concat_raw_before_after(
          &self._raw_before,
          &self._raw_after,
          "".to_string(),
          fmts,
          CreateIndexParams::Only,
          format!("ONLY{space}"),
        )
      } else {
        "".to_string()
      };

      format!("ON{space}{only}{table_name}{space}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      CreateIndexParams::On,
      sql,
    )
  }

  fn concat_using(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { space, .. } = fmts;

    let sql = if self._using.is_empty() == false {
      let index_method = &self._using;
      format!("USING{space}{index_method}{space}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      CreateIndexParams::Using,
      sql,
    )
  }
}

#[cfg(feature = "sqlite")]
impl CreateIndex {
  fn concat_create_index_sqlite(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;

    let unique = if self._unique {
      concat_raw_before_after(
        &self._raw_before,
        &self._raw_after,
        "".to_string(),
        fmts,
        CreateIndexParams::Unique,
        format!("UNIQUE{space}"),
      )
    } else {
      "".to_string()
    };

    let if_not_exists = if self._if_not_exists {
      format!("IF NOT EXISTS{space}")
    } else {
      "".to_string()
    };

    let index_name = if self._index_name.is_empty() == false {
      format!("{}{space}", &self._index_name)
    } else {
      "".to_string()
    };

    let sql = if index_name.is_empty() == false {
      format!("CREATE{space}{unique}INDEX{space}{if_not_exists}{index_name}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      CreateIndexParams::CreateIndex,
      sql,
    )
  }

  fn concat_on_sqlite(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { space, .. } = fmts;

    let sql = if self._on.is_empty() == false {
      let table_name = &self._on;

      format!("ON{space}{table_name}{space}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      CreateIndexParams::On,
      sql,
    )
  }
}
