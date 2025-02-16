use crate::{
  concat::{concat_raw_before_after, Concat},
  fmt,
  structure::{AlterTable, AlterTableAction, AlterTableOrderedAction},
};

impl Concat for AlterTable {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_string();

    query = self.concat_raw(query, &fmts, &self._raw);
    query = self.concat_alter_table(query, &fmts);

    #[cfg(any(feature = "postgresql", feature = "sqlite"))]
    {
      query = self.concat_rename(query, &fmts);
      query = self.concat_rename_to(query, &fmts);
    }
    query = self.concat_ordered_actions(query, &fmts);

    query.trim_end().to_string()
  }
}

impl AlterTable {
  fn concat_alter_table(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { space, .. } = fmts;
    let sql = if self._alter_table.is_empty() == false {
      let table_name = &self._alter_table;
      format!("ALTER TABLE{space}{table_name}{space}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      AlterTableAction::AlterTable,
      sql,
    )
  }

  fn concat_ordered_actions(&self, query: String, fmts: &fmt::Formatter) -> String {
    let actions = self._ordered_actions.iter().filter(|item| item.1.is_empty() == false);

    #[cfg(any(feature = "postgresql", feature = "mysql"))]
    {
      use crate::structure::AlterTableActionItem;

      let fmt::Formatter {
        comma,
        lb,
        indent,
        space,
        ..
      } = fmts;

      let sql = actions
        .map(|item| {
          let AlterTableActionItem(action, content) = item;
          match action {
            AlterTableOrderedAction::Add => format!("{lb}{indent}ADD{space}{content}"),
            AlterTableOrderedAction::Drop => format!("{lb}{indent}DROP{space}{content}"),
            AlterTableOrderedAction::Alter => format!("{lb}{indent}ALTER{space}{content}"),
            #[cfg(feature = "mysql")]
            AlterTableOrderedAction::Rename => format!("{lb}{indent}RENAME{space}{content}"),
          }
        })
        .collect::<Vec<_>>()
        .join(comma)
        .to_string();

      format!("{query}{sql}{space}")
    }

    #[cfg(not(any(feature = "postgresql", feature = "mysql")))]
    {
      let fmt::Formatter { lb, space, .. } = fmts;

      if let Some(item) = actions.last() {
        let (sql, clause) = match item.0 {
          AlterTableOrderedAction::Add => (format!("ADD{space}{}{space}{lb}", item.1), AlterTableAction::Add),
          AlterTableOrderedAction::Drop => (format!("DROP{space}{}{space}{lb}", item.1), AlterTableAction::Drop),
        };

        return concat_raw_before_after(&self._raw_before, &self._raw_after, query, fmts, clause, sql);
      }

      query
    }
  }

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  fn concat_rename(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._rename.is_empty() == false {
      let action = &self._rename;

      format!("RENAME{space}{action}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      AlterTableAction::Rename,
      sql,
    )
  }

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  fn concat_rename_to(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._rename_to.is_empty() == false {
      let table_name = &self._rename_to;

      format!("RENAME TO{space}{table_name}{space}{lb}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      AlterTableAction::RenameTo,
      sql,
    )
  }
}
