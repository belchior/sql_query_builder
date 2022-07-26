#[cfg(feature = "postgresql")]
pub enum Combinator {
  Except,
  Intersect,
  Union,
}

/// Builder to contruct a delete command
#[derive(Default, Clone)]
pub struct DeleteBuilder<'a> {
  pub(crate) _delete_from: &'a str,
  pub(crate) _raw_after: Vec<(DeleteClause, String)>,
  pub(crate) _raw_before: Vec<(DeleteClause, String)>,
  pub(crate) _raw: Vec<String>,
  pub(crate) _where: Vec<String>,

  #[cfg(feature = "postgresql")]
  pub(crate) _returning: Vec<String>,
  #[cfg(feature = "postgresql")]
  pub(crate) _with: Vec<(&'a str, std::sync::Arc<dyn crate::behavior::WithQuery>)>,
}

/// All available clauses to be used in `raw_before` and `raw_after` methods of the DeleteBuilder
#[derive(PartialEq, Clone)]
pub enum DeleteClause {
  DeleteFrom,
  Where,

  #[cfg(feature = "postgresql")]
  Returning,
  #[cfg(feature = "postgresql")]
  With,
}

/// Builder to contruct a insert command
#[derive(Default, Clone)]
pub struct InsertBuilder<'a> {
  pub(crate) _insert_into: &'a str,
  pub(crate) _on: Vec<String>,
  pub(crate) _overriding: &'a str,
  pub(crate) _raw_after: Vec<(InsertClause, String)>,
  pub(crate) _raw_before: Vec<(InsertClause, String)>,
  pub(crate) _raw: Vec<String>,
  pub(crate) _select: Option<SelectBuilder<'a>>,
  pub(crate) _values: Vec<String>,

  #[cfg(feature = "postgresql")]
  pub(crate) _returning: Vec<String>,
  #[cfg(feature = "postgresql")]
  pub(crate) _with: Vec<(&'a str, std::sync::Arc<dyn crate::behavior::WithQuery>)>,
}

/// All available clauses to be used in `raw_before` and `raw_after` methods of the InsertBuilder
#[derive(PartialEq, Clone)]
pub enum InsertClause {
  InsertInto,
  Overriding,
  Select,
  Values,

  #[cfg(feature = "postgresql")]
  Returning,
  #[cfg(feature = "postgresql")]
  With,
}

/// Builder to contruct a select command
#[derive(Default, Clone)]
pub struct SelectBuilder<'a> {
  pub(crate) _from: Vec<String>,
  pub(crate) _group_by: Vec<String>,
  pub(crate) _having: Vec<String>,
  pub(crate) _join: Vec<String>,
  pub(crate) _limit: &'a str,
  pub(crate) _offset: &'a str,
  pub(crate) _order_by: Vec<String>,
  pub(crate) _raw_after: Vec<(SelectClause, String)>,
  pub(crate) _raw_before: Vec<(SelectClause, String)>,
  pub(crate) _raw: Vec<String>,
  pub(crate) _select: Vec<String>,
  pub(crate) _where: Vec<String>,

  #[cfg(feature = "postgresql")]
  pub(crate) _except: Vec<Self>,
  #[cfg(feature = "postgresql")]
  pub(crate) _intersect: Vec<Self>,
  #[cfg(feature = "postgresql")]
  pub(crate) _union: Vec<Self>,
  #[cfg(feature = "postgresql")]
  pub(crate) _with: Vec<(&'a str, std::sync::Arc<dyn crate::behavior::WithQuery>)>,
}

/// All available clauses to be used in `raw_before` and `raw_after` methods of the SelectBuilder
#[derive(Clone, PartialEq)]
pub enum SelectClause {
  From,
  GroupBy,
  Having,
  Join,
  Limit,
  Offset,
  OrderBy,
  Select,
  Where,

  #[cfg(feature = "postgresql")]
  Except,
  #[cfg(feature = "postgresql")]
  Intersect,
  #[cfg(feature = "postgresql")]
  Union,
  #[cfg(feature = "postgresql")]
  With,
}

/// Builder to contruct a update command
#[derive(Default, Clone)]
pub struct UpdateBuilder<'a> {
  pub(crate) _raw_after: Vec<(UpdateClause, String)>,
  pub(crate) _raw_before: Vec<(UpdateClause, String)>,
  pub(crate) _raw: Vec<String>,
  pub(crate) _set: Vec<String>,
  pub(crate) _update: &'a str,
  pub(crate) _where: Vec<String>,

  #[cfg(feature = "postgresql")]
  pub(crate) _from: Vec<String>,
  #[cfg(feature = "postgresql")]
  pub(crate) _returning: Vec<String>,
  #[cfg(feature = "postgresql")]
  pub(crate) _with: Vec<(&'a str, std::sync::Arc<dyn crate::behavior::WithQuery>)>,
}

/// All available clauses to be used in `raw_before` and `raw_after` methods of the UpdateBuilder
#[derive(PartialEq, Clone)]
pub enum UpdateClause {
  Set,
  Update,
  Where,

  #[cfg(feature = "postgresql")]
  From,
  #[cfg(feature = "postgresql")]
  Returning,
  #[cfg(feature = "postgresql")]
  With,
}
