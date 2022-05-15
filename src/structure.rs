/// Builder to contruct a Select query
#[derive(Default, Clone)]
pub struct SelectBuilder<'a> {
  pub(crate) _except: Vec<Self>,
  pub(crate) _from: Vec<String>,
  pub(crate) _group_by: Vec<String>,
  pub(crate) _having: Vec<String>,
  pub(crate) _intersect: Vec<Self>,
  pub(crate) _join: Vec<String>,
  pub(crate) _limit: &'a str,
  pub(crate) _offset: &'a str,
  pub(crate) _order_by: Vec<String>,
  pub(crate) _raw_after: Vec<(Clause, String)>,
  pub(crate) _raw_before: Vec<(Clause, String)>,
  pub(crate) _raw: Vec<String>,
  pub(crate) _select: Vec<String>,
  pub(crate) _union: Vec<Self>,
  pub(crate) _where: Vec<String>,
  pub(crate) _with: Vec<(&'a str, Self)>,
}

/// All available clauses to be used in `raw_before` and `raw_after` methods
#[derive(Clone, Copy, PartialEq)]
pub enum Clause {
  Except,
  From,
  GroupBy,
  Having,
  Intersect,
  Join,
  Limit,
  Offset,
  OrderBy,
  Select,
  Union,
  Where,
  With,
}