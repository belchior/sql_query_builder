use pretty_assertions::assert_eq;
use sql_query_builder as sql;

#[test]
fn transaction_builder_should_be_displayable() {
  #[cfg(not(feature = "sqlite"))]
  {
    let tr = sql::Transaction::new().start_transaction("").commit("");

    println!("{}", tr);

    let query = tr.as_string();
    let expected_query = "START TRANSACTION; COMMIT;";

    assert_eq!(query, expected_query);
  }

  #[cfg(feature = "sqlite")]
  {
    let tr = sql::Transaction::new().begin("").commit("");

    println!("{}", tr);

    let query = tr.as_string();
    let expected_query = "BEGIN; COMMIT;";

    assert_eq!(query, expected_query);
  }
}

#[test]
fn transaction_builder_should_be_debuggable() {
  #[cfg(not(feature = "sqlite"))]
  {
    let tr = sql::Transaction::new().start_transaction("").commit("TRANSACTION");

    println!("{:?}", tr);

    let expected_query = "START TRANSACTION; COMMIT TRANSACTION;";
    let query = tr.as_string();

    assert_eq!(query, expected_query);
  }

  #[cfg(feature = "sqlite")]
  {
    let tr = sql::Transaction::new().begin("").commit("TRANSACTION");

    println!("{:?}", tr);

    let expected_query = "BEGIN; COMMIT TRANSACTION;";
    let query = tr.as_string();

    assert_eq!(query, expected_query);
  }
}

#[test]
fn transaction_builder_should_be_able_to_conditionally_add_clauses() {
  #[cfg(not(feature = "sqlite"))]
  {
    let mut tr = sql::Transaction::new().start_transaction("");

    if true {
      tr = tr.commit("WORK");
    }

    let query = tr.as_string();
    let expected_query = "START TRANSACTION; COMMIT WORK;";

    assert_eq!(query, expected_query);
  }

  #[cfg(feature = "sqlite")]
  {
    let mut tr = sql::Transaction::new().begin("");

    if true {
      tr = tr.commit("");
    }

    let query = tr.as_string();
    let expected_query = "BEGIN; COMMIT;";

    assert_eq!(query, expected_query);
  }
}

#[test]
fn transaction_builder_should_be_composable() {
  #[cfg(not(feature = "sqlite"))]
  {
    fn start_transaction(tr: sql::Transaction) -> sql::Transaction {
      tr.start_transaction("")
        .set_transaction("isolation level read committed")
    }

    fn commit(tr: sql::Transaction) -> sql::Transaction {
      tr.commit("")
    }

    fn as_string(tr: sql::Transaction) -> String {
      tr.as_string()
    }

    let query = Some(sql::Transaction::new())
      .map(start_transaction)
      .map(commit)
      .map(as_string)
      .unwrap();

    let expected_query = "\
      START TRANSACTION; \
      SET TRANSACTION isolation level read committed; \
      COMMIT;\
    ";

    assert_eq!(query, expected_query);
  }

  #[cfg(feature = "sqlite")]
  {
    fn begin(tr: sql::Transaction) -> sql::Transaction {
      tr.begin("")
    }

    fn commit(tr: sql::Transaction) -> sql::Transaction {
      tr.commit("")
    }

    fn as_string(tr: sql::Transaction) -> String {
      tr.as_string()
    }

    let query = Some(sql::Transaction::new())
      .map(begin)
      .map(commit)
      .map(as_string)
      .unwrap();

    let expected_query = "\
      BEGIN; \
      COMMIT;\
    ";

    assert_eq!(query, expected_query);
  }
}
