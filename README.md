Write SQL queries in a simple and composable way.

The main goal of this library is to find the best balance between write idiomatic SQL queries and manage cenarios
of complex query composition. Try in a large query and you will understand what this lib can do for you.

## Quick Start

```rust
use sql_query_builder::SelectBuilder;

let mut select = SelectBuilder::new()
  .select("id, login")
  .from("users")
  .where_clause("login = $1");

let id = Some(123);

if let Some(id) = id {
  select = select.and("id = $2");
}

let query = select.as_string();

println!("{}", query);
```

Output

```sql
SELECT id, login FROM users WHERE login = $1 AND id = $2
```

In simple terms this library will not try to understand what you are writing inside the arguments, this is good
because it's removes a lot complexity and verbosity that other libraries needs to generate a SQL query,
whereas debugging tends to be more difficult and silly error can araise.
The lib has `.debug()` method to minimize the effort to debug a complex query.

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

You can dynamically add a clause mutating the select

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

fn joins(select: SelectBuilder) -> SelectBuilder {
  select
    .from("users u")
    .inner_join("address a ON a.user_login = u.login")
    .inner_join("orders o ON o.user_login = u.login")
}

fn conditions(select: SelectBuilder) -> SelectBuilder {
  select.where_clause("u.login = $1").and("o.id = $2")
}

fn as_string(select: SelectBuilder) -> String {
  select.as_string()
}

let query = Some(SelectBuilder::new())
  .map(project)
  .map(joins)
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
