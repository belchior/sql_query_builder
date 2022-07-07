Write SQL queries in a simple and composable way.

The main goal is to find the best balance between write idiomatic SQL queries and manage cenarios
of complex query composition mixed with conditional clauses.


## Quick Start

```rust
use sql_query_builder::SelectBuilder;

let mut select = SelectBuilder::new()
  .select("id, login")
  .from("users")
  .where_clause("login = $1");

let is_admin = true;

if is_admin {
  select = select.and("is_admin = true");
}

let query = select.as_string();

println!("{}", query);
```

Output

```sql
SELECT id, login FROM users WHERE login = $1 AND is_admin = true
```

In simple terms this library will not try to understand what you are writing inside the arguments, this is good
because it's removes a lot complexity and verbosity that other libraries needs to generate a SQL query,
in contrast debugging tends to be more difficult and silly error can araise.
The lib has `.debug()` method with a nice output to minimize the effort to debug a complex query.


## How it's works

Consecutive calls to the same clause will accumulates values respecting the order of the calls,
the two select produce the same SQL query

```rust
use sql_query_builder::SelectBuilder;

let select = SelectBuilder::new()
  .select("id, login");

let select = SelectBuilder::new()
  .select("id")
  .select("login");
```

Methods like `limit` and `offset` will override the previous value, the two select produce the same SQL query

```rust
use sql_query_builder::SelectBuilder;

let select = SelectBuilder::new()
  .limit("123");

let select = SelectBuilder::new()
  .limit("1000")
  .limit("123");
```

The library ignores the order between clauses so the two selects produce the same SQL query

```rust
use sql_query_builder::SelectBuilder;

let select = SelectBuilder::new()
  .select("id, login")
  .from("users")
  .where_clause("login = $1");

let select = SelectBuilder::new()
  .from("users")
  .where_clause("login = $1")
  .select("id, login");
```

You can conditionally add a clause mutating the select

```rust
use sql_query_builder::SelectBuilder;

let mut select = SelectBuilder::new()
  .select("id, login")
  .from("users")
  .where_clause("login = $1");

let shouldIncludesAddress = true;

if shouldIncludesAddress {
  select = select.inner_join("address on user.login = address.owner_login");
}
```


## Composition

Composition is very welcome to write complex queries, this feature makes the library shine

```rust
use sql_query_builder::SelectBuilder;

fn project(select: SelectBuilder) -> SelectBuilder {
select
    .select("u.id, u.name as user_name, u.login")
    .select("a.name as address_name")
    .select("o.name as product_name")
}

fn relations(select: SelectBuilder) -> SelectBuilder {
  select
    .from("users u")
    .inner_join("address a ON a.user_login = u.login")
    .inner_join("orders o ON o.user_login = u.login")
}

fn conditions(select: SelectBuilder) -> SelectBuilder {
  select
    .where_clause("u.login = $1")
    .and("o.id = $2")
}

fn as_string(select: SelectBuilder) -> String {
  select.as_string()
}

let query = Some(SelectBuilder::new())
  .map(project)
  .map(relations)
  .map(conditions)
  .map(as_string)
  .unwrap();

println!("{}", query);
```

Output (indented for redability)

```sql
SELECT u.id, u.name as user_name, u.login, a.name as address_name, o.name as product_name
FROM users u
INNER JOIN address a ON a.user_login = u.login
INNER JOIN orders o ON o.user_login = u.login
WHERE u.login = $1 AND o.id = $2
```


## Raw queries

You can use the raw method to accomplish some edge cases that are hard to rewrite into the SelectBuilder syntax.
The `select.raw()` method will put any SQL you define at top of the output

```rust
use sql_query_builder::SelectBuilder;

let raw_query = "\
  select u.id as user_id, addr.* \
  from users u \
  inner join address addr on u.login = addr.owner_login\
";
let select = SelectBuilder::new()
  .raw(raw_query)
  .where_clause("login = $1");
```

To a more precisely use case your can use the `select.raw_before()` and `select.raw_after()`

```rust
use sql_query_builder::{SelectBuilder, SelectClause};

let raw_query = "\
  from users u \
  inner join address addr on u.login = addr.owner_login\
";
let select = SelectBuilder::new()
  .select("u.id as user_id, addr.*")
  .raw_before(SelectClause::Where, raw_query)
  .where_clause("login = $1");
```

```rust
use sql_query_builder::{SelectBuilder, SelectClause};

let raw_query = "\
  from users u \
  inner join address addr on u.login = addr.owner_login\
";
let select = SelectBuilder::new()
  .select("u.id as user_id, addr.*")
  .raw_after(SelectClause::Select, raw_query)
  .where_clause("login = $1");
```


## Crate features

SQL Query Builder comes with the following optional features:
- `postgresql` enable Postgres syntax

You can enable features like

```toml
# Cargo.toml

sql_query_builder = { version = "0.x.x", features = ["postgresql"] }
```
