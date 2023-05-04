Write SQL queries in a simple and composable way.

The main goal is to find the best balance between write idiomatic SQL queries and manage scenarios
of complex query composition mixed with conditional clauses.


## Quick Start

```rust
use sql_query_builder as sql;

let mut select = sql::Select::new()
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


## Feature Flags

SQL Query Builder comes with the following optional features:
- `postgresql` enable Postgres syntax

You can enable features like

```toml
## Cargo.toml

sql_query_builder = { version = "1.x.x", features = ["postgresql"] }
```


## How it's works

In simple terms this library will not try to understand what you are writing inside the arguments, this is good
because it's removes a lot of complexity and verbosity to generate a SQL query, in contrast debugging tends to be more difficult and silly error can araise.
The lib has the [debug()](https://docs.rs/sql_query_builder/latest/sql_query_builder/struct.Select.html#method.debug) method with a nice output to minimize the effort to debug a complex query.

Consecutive calls to the same clause will accumulates values respecting the order of the calls,
the two select produce the same SQL query

```rust
use sql_query_builder as sql;

let select = sql::Select::new()
  .select("id, login");

let select = sql::Select::new()
  .select("id")
  .select("login");
```

Methods like `limit` and `offset` will override the previous value, the two select is equivalent

```text
use sql_query_builder as sql;

let select = sql::Select::new()
  .limit("1000")
  .limit("123");

let select = sql::Select::new()
  .limit("123");
```

The library ignores the order between clauses so the two selects are the same

```rust
use sql_query_builder as sql;

let select = sql::Select::new()
  .select("id, login")
  .from("users")
  .where_clause("login = $1");

let select = sql::Select::new()
  .from("users")
  .where_clause("login = $1")
  .select("id, login");
```

You can conditionally add a clause mutating the select

```rust
use sql_query_builder as sql;

let mut select = sql::Select::new()
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
use sql_query_builder as sql;

fn project(select: sql::Select) -> sql::Select {
select
    .select("u.id, u.name as user_name, u.login")
    .select("a.name as address_name")
    .select("o.name as product_name")
}

fn relations(select: sql::Select) -> sql::Select {
  select
    .from("users u")
    .inner_join("address a ON a.user_login = u.login")
    .inner_join("orders o ON o.user_login = u.login")
}

fn conditions(select: sql::Select) -> sql::Select {
  select
    .where_clause("u.login = $1")
    .and("o.id = $2")
}

fn as_string(select: sql::Select) -> String {
  select.as_string()
}

let query = Some(sql::Select::new())
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

You can use the raw method to accomplish some edge cases that are hard to rewrite into the Select syntax.
The `select.raw()` method will put any SQL you define on top of the output

```rust
use sql_query_builder as sql;

let raw_query = "\
  select u.id as user_id, addr.* \
  from users u \
  inner join address addr on u.login = addr.owner_login\
";
let select = sql::Select::new()
  .raw(raw_query)
  .where_clause("login = $1");
```

To a more precisely use case your can use the `select.raw_before()` and `select.raw_after()`

```rust
use sql_query_builder as sql;

let raw_query = "\
  from users u \
  inner join address addr on u.login = addr.owner_login\
";
let select = sql::Select::new()
  .select("u.id as user_id, addr.*")
  .raw_before(sql::SelectClause::Where, raw_query)
  .where_clause("login = $1");
```

```rust
use sql_query_builder as sql;

let raw_query = "\
  from users u \
  inner join address addr on u.login = addr.owner_login\
";
let select = sql::Select::new()
  .select("u.id as user_id, addr.*")
  .raw_after(sql::SelectClause::Select, raw_query)
  .where_clause("login = $1");
```

## Debugging queries

Sometimes it's more ease just print de current state of the query builder, to do this just add the .debug() method at any part of the builder, note that the where clause will not be printed because the debug was added before

```rust
use sql_query_builder as sql;

let mut select = sql::Select::new()
  .select("id, login")
  .from("users")
  .debug()
  .where_clause("login = $1");
```

Output

```sql
SELECT id, login
FROM users
```

See the [documentation](https://docs.rs/sql_query_builder/) for more builders like [Insert](https://docs.rs/sql_query_builder/latest/sql_query_builder/struct.Insert.html), [Update](https://docs.rs/sql_query_builder/latest/sql_query_builder/struct.Update.html) and [Delete](https://docs.rs/sql_query_builder/latest/sql_query_builder/struct.Delete.html)
