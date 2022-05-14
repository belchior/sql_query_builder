#[derive(Default, Clone)]
pub struct SelectBuilder<'a> {
  pub(crate) _from: Vec<String>,
  pub(crate) _join: Vec<String>,
  pub(crate) _limit: &'a str,
  pub(crate) _order_by: Vec<String>,
  pub(crate) _raw_after: Vec<(Clause, String)>,
  pub(crate) _raw_before: Vec<(Clause, String)>,
  pub(crate) _raw: Vec<String>,
  pub(crate) _select: Vec<String>,
  pub(crate) _union: Vec<Self>,
  pub(crate) _where: Vec<String>,
  pub(crate) _with: Vec<(&'a str, Self)>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Clause {
  From,
  Join,
  Limit,
  OrderBy,
  Select,
  Union,
  Where,
  With,
}
