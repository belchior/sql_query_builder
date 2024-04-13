use crate::{
  behavior::{concat_raw_before_after, Concat, ConcatSqlStandard},
  fmt,
  structure::{CreateTable, CreateTableParams},
};

impl ConcatSqlStandard<CreateTableParams> for CreateTable {}

impl CreateTable {
  fn concat_create_table(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { space, .. } = fmts;
    let sql = if self._create_table.is_empty() == false {
      let table_name = &self._create_table;
      format!("CREATE TABLE{space}{table_name}{space}")
    } else {
      "".to_string()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      CreateTableParams::CreateTable,
      sql,
    )
  }

  fn concat_column(&self, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, indent, .. } = fmts;

    let columns = self
      ._column
      .iter()
      .map(|column| format!("{lb}{indent}{column}"))
      .collect::<Vec<_>>()
      .join(comma)
      .trim_start()
      .to_string();

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      "".to_string(),
      fmts,
      CreateTableParams::Column,
      columns,
    )
  }

  fn concat_constraint(&self, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter {
      comma,
      lb,
      space,
      indent,
      ..
    } = fmts;

    let constraints = self
      ._constraint
      .iter()
      .map(|constraint| format!("{lb}{indent}CONSTRAINT{space}{constraint}"))
      .collect::<Vec<_>>()
      .join(comma);

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      "".to_string(),
      fmts,
      CreateTableParams::Constraint,
      constraints,
    )
  }

  fn concat_foreign_key(&self, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, indent, .. } = fmts;

    let foreign_keys = self
      ._foreign_key
      .iter()
      .map(|foreign_key| format!("{lb}{indent}FOREIGN KEY{foreign_key}"))
      .collect::<Vec<_>>()
      .join(comma);

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      "".to_string(),
      fmts,
      CreateTableParams::ForeignKey,
      foreign_keys,
    )
  }

  fn concat_primary_key(&self, fmts: &fmt::Formatter) -> String {
    let primary_key = match &self._primary_key {
      pk_exp if pk_exp.find('(').is_some() => format!("PRIMARY KEY{pk_exp}"),
      pk_exp if pk_exp.is_empty() == false => format!("PRIMARY KEY({pk_exp})"),
      _ => "".to_string(),
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      "".to_string(),
      fmts,
      CreateTableParams::PrimaryKey,
      primary_key,
    )
  }

  fn concat_parameters(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter {
      comma,
      lb,
      space,
      indent,
      ..
    } = fmts;

    let columns = self.concat_column(fmts);
    let primary_keys = self.concat_primary_key(fmts);
    let constraints = self.concat_constraint(fmts);
    let foreign_keys = self.concat_foreign_key(fmts);

    let params = [columns, primary_keys, constraints, foreign_keys]
      .into_iter()
      .filter(|item| item.is_empty() == false)
      .collect::<Vec<_>>()
      .join(comma)
      .trim_end()
      .to_string();

    if params.is_empty() == false {
      format!("{query}({lb}{indent}{params}{lb}){space}{lb}")
    } else {
      query
    }
  }
}

impl Concat for CreateTable {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_string();

    query = self.concat_raw(query, &fmts, &self._raw);
    query = self.concat_create_table(query, &fmts);
    query = self.concat_parameters(query, &fmts);

    query.trim_end().to_string()
  }
}
