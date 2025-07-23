use crate::{
  concat::{concat_raw_before_after, Concat},
  fmt,
  structure::{Insert, InsertClause},
};

#[cfg(feature = "mysql")]
use crate::structure::MySqlVariance;

impl Concat for Insert {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_string();

    #[cfg(not(any(feature = "postgresql", feature = "sqlite", feature = "mysql")))]
    {
      query = self.concat_raw(query, &fmts, &self._raw);
      query = self.concat_insert_into(query, &fmts);
      query = self.concat_overriding(query, &fmts);
      query = self.concat_default_values(query, &fmts);
      query = self.concat_values(query, &fmts);
      query = self.concat_select(query, &fmts);
    }

    #[cfg(feature = "postgresql")]
    {
      query = self.concat_raw(query, &fmts, &self._raw);
      query = self.concat_with(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        InsertClause::With,
        &self._with,
      );
      query = self.concat_insert_into(query, &fmts);
      query = self.concat_overriding(query, &fmts);
      query = self.concat_default_values(query, &fmts);
      query = self.concat_values(query, &fmts);
      query = self.concat_select(query, &fmts);
      query = self.concat_on_conflict(query, &fmts);
      query = self.concat_returning(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        InsertClause::Returning,
        &self._returning,
      );
    }

    #[cfg(feature = "sqlite")]
    {
      query = self.concat_raw(query, &fmts, &self._raw);
      query = self.concat_with(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        InsertClause::With,
        &self._with,
      );
      query = self.concat_insert_into(query, &fmts);
      query = self.concat_insert_or(query, &fmts);
      query = self.concat_replace_into(query, &fmts);
      query = self.concat_default_values(query, &fmts);
      query = self.concat_values(query, &fmts);
      query = self.concat_select(query, &fmts);
      query = self.concat_on_conflict(query, &fmts);
      query = self.concat_returning(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        InsertClause::Returning,
        &self._returning,
      );
    }

    #[cfg(feature = "mysql")]
    {
      query = self.concat_raw(query, &fmts, &self._raw);
      query = self.concat_insert_into(query, &fmts);
      query = self.concat_insert(query, &fmts);
      query = self.concat_into(query, &fmts);
      query = self.concat_partition(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        InsertClause::Partition,
        &self._partition,
      );

      match self._mysql_variance {
        MySqlVariance::InsertSelect => {
          query = self.concat_column(
            &self._raw_before,
            &self._raw_after,
            query,
            &fmts,
            InsertClause::Column,
            &self._column,
          );
          query = self.concat_select(query, &fmts);
        }
        MySqlVariance::InsertSet => {
          query = self.concat_set(
            &self._raw_before,
            &self._raw_after,
            query,
            &fmts,
            InsertClause::Set,
            &self._set,
          );
        }
        MySqlVariance::InsertValues | MySqlVariance::InsertValuesRow => {
          query = self.concat_column(
            &self._raw_before,
            &self._raw_after,
            query,
            &fmts,
            InsertClause::Column,
            &self._column,
          );
          query = self.concat_values(query, &fmts);
        }
      }

      query = self.concat_on_duplicate_key_update(query, &fmts);
    }

    query.trim_end().to_string()
  }
}

impl Insert {
  fn concat_insert_into(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._insert_into.is_empty() == false {
      let expression = &self._insert_into;
      format!("INSERT INTO{space}{expression}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      InsertClause::InsertInto,
      sql,
    )
  }

  #[cfg(not(any(feature = "sqlite", feature = "mysql")))]
  fn concat_overriding(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._overriding.is_empty() == false {
      let overriding = &self._overriding;
      format!("OVERRIDING{space}{overriding}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      InsertClause::Overriding,
      sql,
    )
  }

  fn concat_select(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if let Some(select) = &self._select {
      let select_string = select.concat(fmts);
      format!("{select_string}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      InsertClause::Select,
      sql,
    )
  }

  #[cfg(not(feature = "mysql"))]
  fn concat_default_values(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._default_values {
      format!("DEFAULT VALUES{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      InsertClause::DefaultValues,
      sql,
    )
  }

  fn concat_values(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._values.is_empty() == false {
      let sep = format!("{comma}{lb}");
      let rows = self
        ._values
        .iter()
        .filter(|item| item.is_empty() == false)
        .map(|item| {
          #[cfg(not(feature = "mysql"))]
          {
            item.clone()
          }

          #[cfg(feature = "mysql")]
          {
            if self._mysql_variance == MySqlVariance::InsertValuesRow {
              format!("ROW{item}")
            } else {
              item.clone()
            }
          }
        })
        .collect::<Vec<_>>()
        .join(&sep);

      if rows.is_empty() == true {
        return "".to_string();
      }

      format!("VALUES{space}{lb}{rows}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      InsertClause::Values,
      sql,
    )
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
use crate::concat::non_standard::{ConcatReturning, ConcatWith};

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
impl ConcatReturning<InsertClause> for Insert {}
#[cfg(any(feature = "postgresql", feature = "sqlite"))]
impl ConcatWith<InsertClause> for Insert {}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
impl Insert {
  fn concat_on_conflict(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._on_conflict.is_empty() == false {
      let overriding = &self._on_conflict;
      format!("ON CONFLICT{space}{overriding}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      InsertClause::OnConflict,
      sql,
    )
  }
}

#[cfg(feature = "sqlite")]
impl Insert {
  fn concat_insert_or(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._insert_or.is_empty() == false {
      let expression = &self._insert_or;
      format!("INSERT OR{space}{expression}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      InsertClause::InsertOr,
      sql,
    )
  }

  fn concat_replace_into(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._replace_into.is_empty() == false {
      let table_name = &self._replace_into;
      format!("REPLACE INTO{space}{table_name}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      InsertClause::ReplaceInto,
      sql,
    )
  }
}

#[cfg(feature = "mysql")]
use crate::{
  concat::{mysql::ConcatPartition, non_standard::ConcatColumn, sql_standard::ConcatSet},
  utils,
};

#[cfg(feature = "mysql")]
impl ConcatColumn<InsertClause> for Insert {}
#[cfg(feature = "mysql")]
impl ConcatPartition<InsertClause> for Insert {}
#[cfg(feature = "mysql")]
impl ConcatSet<InsertClause> for Insert {}

#[cfg(feature = "mysql")]
impl Insert {
  fn concat_insert(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { space, .. } = fmts;
    let sql = if self._insert.is_empty() == false {
      let modifiers = &self._insert;
      format!("INSERT{space}{modifiers}{space}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      InsertClause::Insert,
      sql,
    )
  }

  fn concat_into(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { space, .. } = fmts;
    let sql = if self._into.is_empty() == false {
      let table_name = &self._into;
      format!("INTO{space}{table_name}{space}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      InsertClause::Into,
      sql,
    )
  }

  fn concat_on_duplicate_key_update(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._on_duplicate_key_update.is_empty() == false {
      let values = utils::join(&self._on_duplicate_key_update, comma);
      format!("ON DUPLICATE KEY UPDATE{space}{values}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      InsertClause::OnDuplicateKeyUpdate,
      sql,
    )
  }
}
