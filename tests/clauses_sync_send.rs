use sql_query_builder::{Delete, Insert, Select, Update};

/* If any of these tests fail to compile with a message similar to:
*
* `(dyn sql_query_builder::behavior::WithQuery + 'static)` cannot be sent between threads safely
*
* Then it means that an introduced change caused a query type to lose its Send/Sync marker.
*/

#[test]
fn queries_must_be_sync_send() {
  must_be_sync_send(Select::new());
  must_be_sync_send(Delete::new());
  must_be_sync_send(Update::new());
  must_be_sync_send(Insert::new());
}

fn must_be_sync_send(_clause: impl Sync + Send) {}
