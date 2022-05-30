use crate::fmt;

pub fn raw_queries<'a, T>(raw_list: &'a Vec<(T, String)>, clause: &'a T) -> Vec<String>
where
  T: PartialEq,
{
  raw_list
    .iter()
    .filter(|item| item.0 == *clause)
    .map(|item| item.1.clone())
    .collect::<Vec<_>>()
}

pub trait BuilderInner<'a, T> {
  fn concat(&self, fmts: &fmt::Formatter) -> String;

  fn raw_before(&self) -> &Vec<(T, String)>;

  fn raw_after(&self) -> &Vec<(T, String)>;

  fn concat_raw_before_after(&self, clause: T, query: String, fmts: &fmt::Formatter, sql: String) -> String
  where
    T: PartialEq,
  {
    let fmt::Formatter { space, .. } = fmts;
    let raw_before = raw_queries(self.raw_before(), &clause).join(space);
    let raw_after = raw_queries(self.raw_after(), &clause).join(space);
    let space_after = if raw_after.is_empty() == false { space } else { "" };
    let space_before = if raw_before.is_empty() == false { space } else { "" };

    format!("{query}{raw_before}{space_before}{sql}{raw_after}{space_after}")
  }
}
