# sql_query_builder

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

In simple terms this library will not try to undestand what you are writing, for one side this is good
because it's removes a lot complexity and verbosity that other libraries needs to generate a SQL query,
for another side debug tends to be more dificult based on the same arguments. The lib has `.debug()` method
to minimize the effort to debug a complex query.

## How it's works

See the documentation
