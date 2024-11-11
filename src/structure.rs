use crate::behavior::TransactionQuery;

#[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
use crate::behavior::WithQuery;

#[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
use std::sync::Arc;

/// Builder to contruct a [AlterTable] command.
///
/// Basic API
///
/// ```
/// use sql_query_builder as sql;
///
/// let query = sql::AlterTable::new()
///   .alter_table("users")
///   .add("COLUMN id serial primary key")
///   .as_string();
///
/// # let expected = "ALTER TABLE users ADD COLUMN id serial primary key";
/// # assert_eq!(expected, query);
/// ```
///
/// Output
///
/// ```sql
/// ALTER TABLE users ADD COLUMN id serial primary key
/// ```
#[derive(Default, Clone)]
pub struct AlterTable {
  pub(crate) _alter_table: String,
  pub(crate) _ordered_actions: Vec<AlterTableActionItem>,
  pub(crate) _raw_after: Vec<(AlterTableAction, String)>,
  pub(crate) _raw_before: Vec<(AlterTableAction, String)>,
  pub(crate) _raw: Vec<String>,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  pub(crate) _rename: String,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  pub(crate) _rename_to: String,
}

#[derive(PartialEq, Clone)]
pub(crate) struct AlterTableActionItem(pub(crate) AlterTableOrderedAction, pub(crate) String);

/// Actions used to build the sequencial part of [AlterTable]
#[derive(PartialEq, Clone)]
pub(crate) enum AlterTableOrderedAction {
  Add,
  Drop,

  #[cfg(any(feature = "postgresql"))]
  Alter,
}

/// All available params to be used in [AlterTable::raw_before] and [AlterTable::raw_after] methods on [AlterTable] builder
#[derive(PartialEq, Clone)]
pub enum AlterTableAction {
  AlterTable,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  #[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
  #[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
  Rename,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  #[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
  #[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
  RenameTo,

  #[cfg(not(any(feature = "postgresql")))]
  Add,

  #[cfg(not(any(feature = "postgresql")))]
  Drop,
}

#[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
pub(crate) enum Combinator {
  Except,
  Intersect,
  Union,
}

/// Builder to contruct a [CreateIndex] command. Available only for the crate features `postgresql` and `sqlite`.
///
/// Basic API
///
/// ```
/// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
/// # {
/// use sql_query_builder as sql;
///
/// let query = sql::CreateIndex::new()
///   .create_index("users_name_idx")
///   .on("users")
///   .column("name")
///   .as_string();
///
/// # let expected = "CREATE INDEX users_name_idx ON users (name)";
/// # assert_eq!(expected, query);
/// # }
/// ```
///
/// Output
///
/// ```sql
/// CREATE INDEX users_name_idx ON users (name)
/// ```
#[cfg(any(feature = "postgresql", feature = "sqlite"))]
#[derive(Default, Clone)]
pub struct CreateIndex {
  pub(crate) _column: Vec<String>,
  pub(crate) _index_name: String,
  pub(crate) _create_index: bool,
  pub(crate) _if_not_exists: bool,
  pub(crate) _on: String,
  pub(crate) _raw_after: Vec<(CreateIndexParams, String)>,
  pub(crate) _raw_before: Vec<(CreateIndexParams, String)>,
  pub(crate) _raw: Vec<String>,
  pub(crate) _unique: bool,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  pub(crate) _where: Vec<(LogicalOperator, String)>,

  #[cfg(feature = "postgresql")]
  pub(crate) _concurrently: bool,
  #[cfg(feature = "postgresql")]
  pub(crate) _include: Vec<String>,
  #[cfg(feature = "postgresql")]
  pub(crate) _only: bool,
  #[cfg(feature = "postgresql")]
  pub(crate) _using: String,
}

/// All available params to be used in [CreateIndex::raw_before] and [CreateIndex::raw_after] methods on [CreateIndex] builder
#[cfg(any(feature = "postgresql", feature = "sqlite"))]
#[derive(PartialEq, Clone)]
pub enum CreateIndexParams {
  Column,
  CreateIndex,
  On,
  Unique,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  Where,

  #[cfg(feature = "postgresql")]
  Concurrently,
  #[cfg(feature = "postgresql")]
  Only,
  #[cfg(feature = "postgresql")]
  Using,
  #[cfg(feature = "postgresql")]
  Include,
}

/// Builder to contruct a [CreateTable] command.
///
/// Basic API
///
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

/// Builder to contruct a [DropIndex] command. Available only for the crate features `postgresql` and `sqlite`.
///
/// Basic API
///
/// ```
/// # #[cfg(any(feature = "postgresql", feature = "sqlite"))]
/// # {
/// use sql_query_builder as sql;
///
/// let query = sql::DropIndex::new()
///   .drop_index("users_name_idx")
///   .as_string();
///
/// # let expected = "DROP INDEX users_name_idx";
/// # assert_eq!(expected, query);
/// # }
/// ```
///
///
/// Output
///
/// ```sql
/// DROP INDEX users_name_idx
/// ```
#[cfg(any(feature = "postgresql", feature = "sqlite"))]
#[derive(Default, Clone)]
pub struct DropIndex {
  pub(crate) _drop_index: Vec<String>,
  pub(crate) _if_exists: bool,
  pub(crate) _raw_after: Vec<(DropIndexParams, String)>,
  pub(crate) _raw_before: Vec<(DropIndexParams, String)>,
  pub(crate) _raw: Vec<String>,
}

/// All available params to be used in [DropIndex::raw_before] and [DropIndex::raw_after] methods on [DropIndex] builder
#[cfg(any(feature = "postgresql", feature = "sqlite"))]
#[derive(PartialEq, Clone)]
pub enum DropIndexParams {
  DropIndex,
}

/// Builder to contruct a [DropTable] command.
///
/// Basic API
///
/// ```
/// use sql_query_builder as sql;
///
/// let query = sql::DropTable::new()
///   .drop_table("users")
///   .as_string();
///
/// # let expected = "DROP TABLE users";
/// # assert_eq!(expected, query);
/// ```
///
///
/// Output
///
/// ```sql
/// DROP TABLE users
/// ```
#[derive(Default, Clone)]
pub struct DropTable {
  pub(crate) _drop_table: Vec<String>,
  pub(crate) _if_exists: bool,
  pub(crate) _raw_after: Vec<(DropTableParams, String)>,
  pub(crate) _raw_before: Vec<(DropTableParams, String)>,
  pub(crate) _raw: Vec<String>,
}

/// All available params to be used in [DropTable::raw_before] and [DropTable::raw_after] methods on [DropTable] builder
#[derive(PartialEq, Clone)]
pub enum DropTableParams {
  DropTable,
}

/// Builder to contruct a [Delete] command.
///
/// Basic API
///
/// ```
/// use sql_query_builder as sql;
///
/// let query = sql::Delete::new()
///   .delete_from("users")
///   .where_clause("id = $1")
///   .as_string();
///
/// # let expected = "DELETE FROM users WHERE id = $1";
/// # assert_eq!(expected, query);
/// ```
///
/// Output
///
/// ```sql
/// DELETE FROM users WHERE id = $1
/// ```
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
  pub(crate) _with: Vec<(String, std::sync::Arc<dyn crate::behavior::WithQuery + Send + Sync>)>,
}

/// All available clauses to be used in [Delete::raw_before] and [Delete::raw_after] methods on [Delete] builder
#[derive(PartialEq, Clone)]
pub enum DeleteClause {
  DeleteFrom,
  Where,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  #[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
  #[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
  Returning,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  #[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
  #[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
  With,
}

/// Builder to contruct a [Insert] command.
///
/// Basic API
///
/// ```
/// use sql_query_builder as sql;
///
/// let query = sql::Insert::new()
///   .insert_into("users (login, name)")
///   .values("('foo', 'Foo')")
///   .values("('bar', 'Bar')")
///   .as_string();
///
/// # let expected = "INSERT INTO users (login, name) VALUES ('foo', 'Foo'), ('bar', 'Bar')";
/// # assert_eq!(expected, query);
/// ```
///
/// Output
///
/// ```sql
/// INSERT INTO users (login, name) VALUES ('foo', 'Foo'), ('bar', 'Bar')
/// ```
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
  pub(crate) _with: Vec<(String, std::sync::Arc<dyn crate::behavior::WithQuery + Send + Sync>)>,

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
  #[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
  #[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
  Returning,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  #[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
  #[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
  With,

  #[cfg(feature = "sqlite")]
  #[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
  InsertOr,

  #[cfg(feature = "sqlite")]
  #[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
  ReplaceInto,
}

#[derive(Clone, PartialEq)]
pub(crate) enum LogicalOperator {
  And,
  Or,
}

impl std::fmt::Display for LogicalOperator {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let v = match self {
      LogicalOperator::And => "AND",
      LogicalOperator::Or => "OR",
    };
    write!(f, "{}", v)
  }
}

/// Builder to contruct a [Select] command.
///
/// Basic API
///
/// ```
/// use sql_query_builder as sql;
///
/// let query = sql::Select::new()
///   .select("*")
///   .from("users")
///   .inner_join("orders ON users.login = orders.login")
///   .where_clause("user.login = $1")
///   .order_by("created_at desc")
///   .as_string();
///
/// # let expected = "\
/// #   SELECT * \
/// #   FROM users \
/// #   INNER JOIN orders ON users.login = orders.login \
/// #   WHERE user.login = $1 \
/// #   ORDER BY created_at desc\
/// # ";
/// # assert_eq!(expected, query);
/// ```
///
/// Output (indented for readability)
///
/// ```sql
/// SELECT *
/// FROM users
/// INNER JOIN orders ON users.login = orders.login
/// WHERE user.login = $1
/// ORDER BY created_at desc
/// ```
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

  #[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
  pub(crate) _except: Vec<Self>,

  #[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
  pub(crate) _intersect: Vec<Self>,

  #[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
  pub(crate) _limit: String,

  #[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
  pub(crate) _offset: String,

  #[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
  pub(crate) _union: Vec<Self>,

  #[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
  pub(crate) _with: Vec<(String, Arc<dyn WithQuery + Send + Sync>)>,

  #[cfg(feature = "mysql")]
  pub(crate) _partition: Vec<String>,
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

  #[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
  #[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
  #[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
  #[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
  Except,

  #[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
  #[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
  #[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
  #[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
  Intersect,

  #[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
  #[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
  #[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
  #[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
  Union,

  #[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
  #[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
  #[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
  #[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
  With,

  #[cfg(feature = "mysql")]
  #[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
  Partition,
}

/// Builder to contruct a [Transaction] block.
///
/// Basic API
///
/// ```
/// # #[cfg(not(feature = "sqlite"))]
/// # {
/// use sql_query_builder as sql;
///
/// let insert_foo = sql::Insert::new()
///   .insert_into("users (login, name)")
///   .values("('foo', 'Foo')");
///
/// let update_foo = sql::Update::new()
///   .update("users")
///   .set("name = 'Bar'")
///   .where_clause("login = 'foo'");
///
/// let query = sql::Transaction::new()
///   .start_transaction("isolation level serializable")
///   .insert(insert_foo)
///   .update(update_foo)
///   .commit("transaction")
///   .as_string();
///
/// # let expected = "\
/// # START TRANSACTION isolation level serializable; \
/// # INSERT INTO users (login, name) VALUES ('foo', 'Foo'); \
/// # UPDATE users SET name = 'Bar' WHERE login = 'foo'; \
/// # COMMIT transaction;\
/// # ";
/// # assert_eq!(expected, query);
/// # }
/// ```
///
/// Output (indented for readability)
///
/// ```sql
/// START TRANSACTION isolation level serializable;
/// INSERT INTO users (login, name) VALUES ('foo', 'Foo');
/// UPDATE users SET name = 'Bar' WHERE login = 'foo';
/// COMMIT transaction;
/// ```
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
  #[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
  #[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
  Begin,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  #[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
  #[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
  End,

  #[cfg(not(feature = "sqlite"))]
  #[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
  SetTransaction,

  #[cfg(not(feature = "sqlite"))]
  #[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
  StartTransaction,
}

#[derive(PartialEq)]
pub(crate) struct TransactionCommand(pub(crate) TrCmd, pub(crate) String);

/// Builder to contruct a [Update] command.
///
/// Basic API
///
/// ```
/// use sql_query_builder as sql;
///
/// let query = sql::Update::new()
///   .update("users")
///   .set("name = 'Bar'")
///   .where_clause("id = $1")
///   .as_string();
///
/// # let expected = "UPDATE users SET name = 'Bar' WHERE id = $1";
/// # assert_eq!(expected, query);
/// ```
///
/// Output
///
/// ```sql
/// UPDATE users SET name = 'Bar' WHERE id = $1
/// ```
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
  pub(crate) _with: Vec<(String, std::sync::Arc<dyn crate::behavior::WithQuery + Send + Sync>)>,

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
  #[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
  #[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
  From,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  #[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
  #[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
  Returning,

  #[cfg(any(feature = "postgresql", feature = "sqlite"))]
  #[cfg_attr(docsrs, doc(cfg(feature = "postgresql")))]
  #[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
  With,

  #[cfg(feature = "sqlite")]
  #[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
  UpdateOr,

  #[cfg(feature = "sqlite")]
  #[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
  Join,
}

/// Builder to contruct a [Values] command.
///
/// Basic API
///
/// ```
/// use sql_query_builder as sql;
///
/// let query = sql::Values::new()
///   .values("('foo', 'Foo')")
///   .values("('bar', 'Bar')")
///   .as_string();
///
/// # let expected = "VALUES ('foo', 'Foo'), ('bar', 'Bar')";
/// # assert_eq!(expected, query);
/// ```
///
/// Output
///
/// ```sql
/// VALUES ('foo', 'Foo'), ('bar', 'Bar')
/// ```
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
