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

/// Builder to contruct a [CreateTable] command
///
/// Basic API
/// ```
/// use sql_query_builder as sql;
///
/// let query = sql::CreateTable::new()
///   .create_table("users")
///   .column("id serial primary key")
///   .column("login varchar(40) not null")
///   .constraint("users_login_key unique(login)")
///   .as_string();
///
/// # let expected = "\
/// #   CREATE TABLE users (\
/// #     id serial primary key, \
/// #     login varchar(40) not null, \
/// #     CONSTRAINT users_login_key unique(login)\
/// #   )\
/// # ";
/// # assert_eq!(expected, query);
/// ```
///
///
/// Output (indented for readability)
///
/// ```sql
/// CREATE TABLE users (
///   id serial primary key,
///   login varchar(40) not null,
///   created_at timestamp not null,
///   CONSTRAINT users_login_key unique(login)
/// )
/// ```
#[derive(Default, Clone)]
pub struct CreateTable {
  pub(crate) _column: Vec<String>,
  pub(crate) _constraint: Vec<String>,
  pub(crate) _create_table: String,
  pub(crate) _foreign_key: Vec<String>,
  pub(crate) _primary_key: String,
  pub(crate) _raw_after: Vec<(CreateTableParams, String)>,
  pub(crate) _raw_before: Vec<(CreateTableParams, String)>,
  pub(crate) _raw: Vec<String>,
}

/// All available params to be used in [CreateTable::raw_before] and [CreateTable::raw_after] methods on [CreateTable] builder
#[derive(PartialEq, Clone)]
pub enum CreateTableParams {
  Column,
  Constraint,
  CreateTable,
  ForeignKey,
  PrimaryKey,
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

/// All available clauses to be used in [Delete::raw_before] and [Delete::raw_after] methods on [Delete] builder
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
  pub(crate) _default_values: bool,
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
}

#[cfg(feature = "sqlite")]
#[derive(Default, Clone, PartialEq)]
pub(crate) enum InsertVars {
  #[default]
  InsertInto,
  InsertOr,
  ReplaceInto,
}

/// All available clauses to be used in [Insert::raw_before] and [Insert::raw_after] methods on [Insert] builder
#[derive(PartialEq, Clone)]
pub enum InsertClause {
  DefaultValues,
  InsertInto,
  OnConflict,
  Overriding,
  Select,
  Values,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  Returning,
  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  With,

  #[cfg(feature = "sqlite")]
  InsertOr,
  #[cfg(feature = "sqlite")]
  ReplaceInto,
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
  pub(crate) _window: Vec<String>,

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

/// All available clauses to be used in [Select::raw_before] and [Select::raw_after] methods on [Select] builder
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
  Window,

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

/// All available clauses to be used in [Update::raw_before] and [Update::raw_after] methods on [Update] builder
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

/// All available clauses to be used in [Values::raw_before] and [Values::raw_after] methods on [Values] builder
#[derive(PartialEq, Clone)]
pub enum ValuesClause {
  Values,
}
