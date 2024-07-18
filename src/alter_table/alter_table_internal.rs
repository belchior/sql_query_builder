use crate::{
  behavior::{concat_raw_before_after, Concat, ConcatSqlStandard},
  fmt,
  structure::{AlterTable, AlterTableAction, AlterTableActionItem, AlterTableOrderedAction},
};

impl ConcatSqlStandard<AlterTableAction> for AlterTable {}

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
    let fmt::Formatter {
      comma,
      lb,
      indent,
      space,
      ..
    } = fmts;

    let actions = self
      ._ordered_actions
      .iter()
      .map(|item| {
        let AlterTableActionItem(action, content) = item;
        match action {
          AlterTableOrderedAction::Add => format!("{lb}{indent}ADD {content}"),
          AlterTableOrderedAction::Drop => format!("{lb}{indent}DROP {content}"),
          #[cfg(any(feature = "postgresql", feature = "sqlite"))]
          AlterTableOrderedAction::Rename => format!("{lb}{indent}RENAME {content}"),
          #[cfg(any(feature = "postgresql"))]
          AlterTableOrderedAction::Alter => format!("{lb}{indent}ALTER {content}"),
        }
      })
      .collect::<Vec<_>>()
      .join(comma)
      .to_string();

    format!("{query}{actions}{space}")
  }

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  fn concat_rename_to(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { space, .. } = fmts;
    let sql = if self._rename_to.is_empty() == false {
      let table_name = &self._rename_to;
      format!("RENAME TO{space}{table_name}{space}")
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

impl Concat for AlterTable {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_string();

    query = self.concat_raw(query, &fmts, &self._raw);
    query = self.concat_alter_table(query, &fmts);

    #[cfg(any(feature = "postgresql", feature = "sqlite"))]
    {
      query = self.concat_rename_to(query, &fmts);
    }
    query = self.concat_ordered_actions(query, &fmts);

    query.trim_end().to_string()
  }
}
