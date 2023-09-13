#[cfg(feature = "sqlite")]
mod default_values_clause {
  mod insert_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_default_values_should_add_the_default_values_clause() {
      let query = sql::Insert::new()
        .insert_into("users (login, name)")
        .default_values()
        .as_string();
      let expected_query = "INSERT INTO users (login, name) DEFAULT VALUES";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_default_values_should_overrides_the_values_clause() {
      let query = sql::Insert::new()
        .insert_into("orders (product_name, price)")
        .default_values()
        .values("('Foo', 1234)")
        .as_string();
      let expected_query = "INSERT INTO orders (product_name, price) DEFAULT VALUES";

      assert_eq!(query, expected_query);
    }
  }
}

#[cfg(feature = "sqlite")]
mod insert_or_clause {
  mod insert_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_insert_or_should_add_the_insert_or_clause() {
      let query = sql::Insert::new()
        .insert_or("ABORT INTO users (login, name)")
        .as_string();
      let expected_query = "INSERT OR ABORT INTO users (login, name)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_insert_or_should_override_value_on_consecutive_calls() {
      let query = sql::Insert::new()
        .insert_or("FAIL INTO users (login, name)")
        .insert_or("FAIL INTO orders (product_name, price)")
        .as_string();
      let expected_query = "INSERT OR FAIL INTO orders (product_name, price)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_insert_or_should_trim_space_of_the_argument() {
      let query = sql::Insert::new().insert_or("  IGNORE INTO users (name)  ").as_string();
      let expected_query = "INSERT OR IGNORE INTO users (name)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_insert_or_clause() {
      let query = sql::Insert::new()
        .raw_before(sql::InsertClause::InsertOr, "/* insert or replace */")
        .insert_or("REPLACE INTO users (login)")
        .as_string();
      let expected_query = "/* insert or replace */ INSERT OR REPLACE INTO users (login)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_insert_or_clause() {
      let query = sql::Insert::new()
        .insert_or("ROLLBACK INTO users (name)")
        .raw_after(sql::InsertClause::InsertOr, "values ('foo')")
        .as_string();
      let expected_query = "INSERT OR ROLLBACK INTO users (name) values ('foo')";

      assert_eq!(query, expected_query);
    }
  }
}

#[cfg(feature = "sqlite")]
mod insert_variances {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn when_more_than_one_insert_variances_are_defined_the_last_one_should_overrides_the_previous_ones() {
    let query = sql::Insert::new()
      .insert_into("users (login, name)")
      .insert_or("ABORT INTO users (login, name)")
      .replace_into("users (login, name)")
      .as_string();
    let expected_query = "REPLACE INTO users (login, name)";
    assert_eq!(query, expected_query);

    let query = sql::Insert::new()
      .replace_into("users (login, name)")
      .insert_into("users (login, name)")
      .insert_or("ABORT INTO users (login, name)")
      .as_string();
    let expected_query = "INSERT OR ABORT INTO users (login, name)";
    assert_eq!(query, expected_query);

    let query = sql::Insert::new()
      .insert_or("ABORT INTO users (login, name)")
      .replace_into("users (login, name)")
      .insert_into("users (login, name)")
      .as_string();
    let expected_query = "INSERT INTO users (login, name)";
    assert_eq!(query, expected_query);
  }
}

#[cfg(feature = "sqlite")]
mod update_or_clause {
  mod update_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder as sql;

    #[test]
    fn method_update_or_should_add_the_update_or_clause() {
      let query = sql::Update::new().update_or("ABORT users (login, name)").as_string();
      let expected_query = "UPDATE OR ABORT users (login, name)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_update_or_should_override_value_on_consecutive_calls() {
      let query = sql::Update::new()
        .update_or("FAIL users")
        .update_or("IGNORE orders")
        .as_string();
      let expected_query = "UPDATE OR IGNORE orders";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn methods_update_and_update_or_should_override_each_other() {
      let query = sql::Update::new()
        .update("users")
        .update_or("IGNORE orders")
        .as_string();
      let expected_query = "UPDATE OR IGNORE orders";

      assert_eq!(query, expected_query);

      let query = sql::Update::new()
        .update_or("IGNORE orders")
        .update("users")
        .as_string();
      let expected_query = "UPDATE users";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_update_or_should_trim_space_of_the_argument() {
      let query = sql::Update::new().update_or("  REPLACE orders  ").as_string();
      let expected_query = "UPDATE OR REPLACE orders";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_update_or_clause() {
      let query = sql::Update::new()
        .raw_before(sql::UpdateClause::UpdateOr, "/* update_or users */")
        .update_or("ROLLBACK users")
        .as_string();
      let expected_query = "/* update_or users */ UPDATE OR ROLLBACK users";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_update_or_clause() {
      let query = sql::Update::new()
        .update_or("ABORT users")
        .raw_after(sql::UpdateClause::UpdateOr, "set login = 'foo'")
        .as_string();
      let expected_query = "UPDATE OR ABORT users set login = 'foo'";

      assert_eq!(query, expected_query);
    }
  }
}
