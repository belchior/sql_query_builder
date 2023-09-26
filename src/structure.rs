use crate::behavior::TransactionQuery;
#[cfg(any(feature = "postgresql", feature = "sqlite"))]
use crate::behavior::WithQuery;
#[cfg(any(feature = "postgresql", feature = "sqlite"))]
use std::sync::Arc;

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
pub(crate) enum Combinator {
  Except,
  Intersect,
  Union,
}

/// Builder to contruct a [Delete] command
#[derive(Default, Clone)]
pub struct Delete {
  pub(crate) _delete_from: String,
  pub(crate) _raw_after: Vec<(DeleteClause, String)>,
  pub(crate) _raw_before: Vec<(DeleteClause, String)>,
  pub(crate) _raw: Vec<String>,
  pub(crate) _where: Vec<(LogicalOperator, String)>,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  pub(crate) _returning: Vec<String>,
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  pub(crate) _with: Vec<(String, std::sync::Arc<dyn crate::behavior::WithQuery>)>,
}

/// All available clauses to be used in `raw_before` and `raw_after` methods on [Delete] builder
///
/// # Example
/// ```
/// # use sql_query_builder as sql;
///
/// let raw = "where name = 'Foo'";
/// let delete_query = sql::Delete::new()
///   .delete_from("users")
///   .raw_after(sql::DeleteClause::DeleteFrom, raw)
///   .as_string();
/// ```
#[derive(PartialEq, Clone)]
pub enum DeleteClause {
  DeleteFrom,
  Where,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  Returning,
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  With,
}

/// Builder to contruct a [Insert] command
#[derive(Default, Clone)]
pub struct Insert {
  pub(crate) _on_conflict: String,
  pub(crate) _overriding: String,
  pub(crate) _raw_after: Vec<(InsertClause, String)>,
  pub(crate) _raw_before: Vec<(InsertClause, String)>,
  pub(crate) _raw: Vec<String>,
  pub(crate) _select: Option<Select>,
  pub(crate) _values: Vec<String>,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  pub(crate) _returning: Vec<String>,
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  pub(crate) _with: Vec<(String, std::sync::Arc<dyn crate::behavior::WithQuery>)>,

  #[cfg(not(feature = "sqlite"))]
  pub(crate) _insert_into: String,
  #[cfg(feature = "sqlite")]
  pub(crate) _insert: (InsertVars, String),

  #[cfg(feature = "sqlite")]
  pub(crate) _default_values: bool,
}

#[cfg(feature = "sqlite")]
#[derive(Default, Clone, PartialEq)]
pub(crate) enum InsertVars {
  #[default]
  InsertInto,
  InsertOr,
  ReplaceInto,
}

/// All available clauses to be used in `raw_before` and `raw_after` methods on [Insert] builder
///
/// # Example
/// ```
/// # use sql_query_builder as sql;
///
/// let raw = "values ('foo', 'Foo')";
/// let insert_query = sql::Insert::new()
///   .insert_into("users (login, name)")
///   .raw_after(sql::InsertClause::InsertInto, raw)
///   .as_string();
/// ```
#[derive(PartialEq, Clone)]
pub enum InsertClause {
  InsertInto,
  OnConflict,
  Overriding,
  Select,
  Values,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  Returning,
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  With,
  #[cfg(any(feature = "sqlite"))]
  InsertOr,
  #[cfg(any(feature = "sqlite"))]
  ReplaceInto,
  #[cfg(any(feature = "sqlite"))]
  DefaultValues,
}

#[derive(Clone, PartialEq)]
pub(crate) enum LogicalOperator {
  And,
  Or,
}

/// Builder to contruct a [Select] command
#[derive(Default, Clone)]
pub struct Select {
  pub(crate) _from: Vec<String>,
  pub(crate) _group_by: Vec<String>,
  pub(crate) _having: Vec<String>,
  pub(crate) _join: Vec<String>,
  pub(crate) _order_by: Vec<String>,
  pub(crate) _raw_after: Vec<(SelectClause, String)>,
  pub(crate) _raw_before: Vec<(SelectClause, String)>,
  pub(crate) _raw: Vec<String>,
  pub(crate) _select: Vec<String>,
  pub(crate) _where: Vec<(LogicalOperator, String)>,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  pub(crate) _except: Vec<Self>,
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  pub(crate) _intersect: Vec<Self>,
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  pub(crate) _limit: String,
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  pub(crate) _offset: String,
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  pub(crate) _union: Vec<Self>,
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  pub(crate) _with: Vec<(String, Arc<dyn WithQuery>)>,
}

/// All available clauses to be used in `raw_before` and `raw_after` methods on [Select] builder
///
/// # Example
/// ```
/// # use sql_query_builder as sql;
///
/// let raw_join = "inner join addresses addr on u.login = addr.owner_login";
/// let select_query = sql::Select::new()
///   .select("*")
///   .from("users u")
///   .raw_after(sql::SelectClause::From, raw_join)
///   .where_clause("u.login = foo")
///   .as_string();
/// ```
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

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  Except,
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  Intersect,
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  Union,
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  With,
}

/// Builder to contruct a [Transaction] block
#[derive(Default)]
pub struct Transaction {
  pub(crate) _commit: Option<TransactionCommand>,
  pub(crate) _ordered_commands: Vec<Box<dyn TransactionQuery>>,
  pub(crate) _raw: Vec<String>,
  pub(crate) _set_transaction: Option<TransactionCommand>,
  pub(crate) _start_transaction: Option<TransactionCommand>,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  pub(crate) _begin: Option<TransactionCommand>,
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  pub(crate) _end: Option<TransactionCommand>,
}

/// Commands used in to build a [Transaction]
#[derive(PartialEq)]
pub(crate) enum TrCmd {
  Commit,
  ReleaseSavepoint,
  Rollback,
  Savepoint,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  Begin,
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  End,

  #[cfg(not(feature = "sqlite"))]
  SetTransaction,
  #[cfg(not(feature = "sqlite"))]
  StartTransaction,
}

#[derive(PartialEq)]
pub(crate) struct TransactionCommand(pub(crate) TrCmd, pub(crate) String);

/// Builder to contruct a [Update] command
#[derive(Default, Clone)]
pub struct Update {
  pub(crate) _raw_after: Vec<(UpdateClause, String)>,
  pub(crate) _raw_before: Vec<(UpdateClause, String)>,
  pub(crate) _raw: Vec<String>,
  pub(crate) _set: Vec<String>,
  pub(crate) _where: Vec<(LogicalOperator, String)>,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  pub(crate) _from: Vec<String>,
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  pub(crate) _returning: Vec<String>,
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  pub(crate) _with: Vec<(String, std::sync::Arc<dyn crate::behavior::WithQuery>)>,

  #[cfg(not(feature = "sqlite"))]
  pub(crate) _update: String,
  #[cfg(feature = "sqlite")]
  pub(crate) _update: (UpdateVars, String),
  #[cfg(feature = "sqlite")]
  pub(crate) _join: Vec<String>,
}

#[cfg(feature = "sqlite")]
#[derive(Default, Clone, PartialEq)]
pub(crate) enum UpdateVars {
  #[default]
  Update,
  UpdateOr,
}

/// All available clauses to be used in `raw_before` and `raw_after` methods on [Update] builder
///
/// # Example
/// ```
/// # use sql_query_builder as sql;
///
/// let raw = "set name = 'Foo'";
/// let update_query = sql::Update::new()
///   .update("users")
///   .raw_after(sql::UpdateClause::Update, raw)
///   .as_string();
/// ```
#[derive(PartialEq, Clone)]
pub enum UpdateClause {
  Set,
  Update,
  Where,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  From,
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  Returning,
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  With,

  #[cfg(feature = "sqlite")]
  UpdateOr,
  #[cfg(feature = "sqlite")]
  Join,
}

/// Builder to contruct a [Values] command
#[derive(Default, Clone)]
pub struct Values {
  pub(crate) _raw_after: Vec<(ValuesClause, String)>,
  pub(crate) _raw_before: Vec<(ValuesClause, String)>,
  pub(crate) _raw: Vec<String>,
  pub(crate) _values: Vec<String>,
}

/// All available clauses to be used in `raw_before` and `raw_after` methods on [Values] builder
///
/// # Example
/// ```
/// # use sql_query_builder as sql;
///
/// let raw_query = ", (3, 'three')";
/// let values = sql::Values::new()
///   .values("(1, 'one'), (2, 'two')")
///   .raw_after(sql::ValuesClause::Values, raw_query)
///   .debug();
/// ```
#[derive(PartialEq, Clone)]
pub enum ValuesClause {
  Values,
}
