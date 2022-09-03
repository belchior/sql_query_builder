#[cfg(feature = "postgresql")]
pub enum Combinator {
  Except,
  Intersect,
  Union,
}

/// Builder to contruct a [Delete] command
#[derive(Default, Clone)]
pub struct Delete<'a> {
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

/// All available clauses to be used in `raw_before` and `raw_after` methods on [Delete] builder
///
/// # Examples
/// ```
/// use sql_query_builder as sql;
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

  #[cfg(feature = "postgresql")]
  Returning,
  #[cfg(feature = "postgresql")]
  With,
}

/// Builder to contruct a [Insert] command
#[derive(Default, Clone)]
pub struct Insert<'a> {
  pub(crate) _insert_into: &'a str,
  pub(crate) _on_conflict: &'a str,
  pub(crate) _overriding: &'a str,
  pub(crate) _raw_after: Vec<(InsertClause, String)>,
  pub(crate) _raw_before: Vec<(InsertClause, String)>,
  pub(crate) _raw: Vec<String>,
  pub(crate) _select: Option<Select<'a>>,
  pub(crate) _values: Vec<String>,

  #[cfg(feature = "postgresql")]
  pub(crate) _returning: Vec<String>,
  #[cfg(feature = "postgresql")]
  pub(crate) _with: Vec<(&'a str, std::sync::Arc<dyn crate::behavior::WithQuery>)>,
}

/// All available clauses to be used in `raw_before` and `raw_after` methods on [Insert] builder
///
/// # Examples
/// ```
/// use sql_query_builder as sql;
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

  #[cfg(feature = "postgresql")]
  Returning,
  #[cfg(feature = "postgresql")]
  With,
}

/// Builder to contruct a [Select] command
#[derive(Default, Clone)]
pub struct Select<'a> {
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

/// All available clauses to be used in `raw_before` and `raw_after` methods on [Select] builder
///
/// # Examples
/// ```
/// use sql_query_builder as sql;
///
/// let raw_join = "inner join address addr on u.login = addr.owner_login";
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

  #[cfg(feature = "postgresql")]
  Except,
  #[cfg(feature = "postgresql")]
  Intersect,
  #[cfg(feature = "postgresql")]
  Union,
  #[cfg(feature = "postgresql")]
  With,
}

/// Builder to contruct a [Update] command
#[derive(Default, Clone)]
pub struct Update<'a> {
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

/// All available clauses to be used in `raw_before` and `raw_after` methods on [Update] builder
///
/// # Examples
/// ```
/// use sql_query_builder as sql;
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

  #[cfg(feature = "postgresql")]
  From,
  #[cfg(feature = "postgresql")]
  Returning,
  #[cfg(feature = "postgresql")]
  With,
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
/// # Examples
/// ```
/// use sql_query_builder as sql;
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
