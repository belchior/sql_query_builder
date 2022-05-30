pub enum Combinator {
  Except,
  Intersect,
  Union,
}

/// Builder to contruct a Insert query
#[derive(Default)]
pub struct InsertBuilder<'a> {
  pub(crate) _insert_into: &'a str,
  pub(crate) _on: Vec<String>,
  pub(crate) _overriding: Vec<String>,
  pub(crate) _raw_after: Vec<(InsertClause, String)>,
  pub(crate) _raw_before: Vec<(InsertClause, String)>,
  pub(crate) _raw: Vec<String>,
  pub(crate) _select: Vec<SelectBuilder<'a>>,
  pub(crate) _values: Vec<String>,
}

/// All available clauses to be used in `raw_before` and `raw_after` methods of the InsertBuilder
#[derive(PartialEq)]
pub enum InsertClause {
  InsertInto,
  Values,
  Select,
  Overriding,
  On,
}
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
  pub(crate) _raw_after: Vec<(SelectClause, String)>,
  pub(crate) _raw_before: Vec<(SelectClause, String)>,
  pub(crate) _raw: Vec<String>,
  pub(crate) _select: Vec<String>,
  pub(crate) _union: Vec<Self>,
  pub(crate) _where: Vec<String>,
  pub(crate) _with: Vec<(&'a str, Self)>,
}

/// All available clauses to be used in `raw_before` and `raw_after` methods of the SelectBuilder
#[derive(Clone, PartialEq)]
pub enum SelectClause {
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
