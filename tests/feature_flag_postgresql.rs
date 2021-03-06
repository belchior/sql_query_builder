#[cfg(feature = "postgresql")]
mod returning_clause {
  mod delete_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder::{DeleteBuilder, DeleteClause};

    #[test]
    fn method_returning_should_add_the_returning_clause() {
      let query = DeleteBuilder::new().returning("*").as_string();
      let expected_query = "RETURNING *";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_returning_should_accumulate_values_on_consecutive_calls() {
      let query = DeleteBuilder::new().returning("login").returning("name").as_string();
      let expected_query = "RETURNING login, name";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_returning_should_not_accumulate_arguments_with_the_same_content() {
      let query = DeleteBuilder::new().returning("id").returning("id").as_string();
      let expected_query = "RETURNING id";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_returning_should_trim_space_of_the_argument() {
      let query = DeleteBuilder::new().returning("  login  ").as_string();
      let expected_query = "RETURNING login";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_returning_should_be_after_where_clause() {
      let query = DeleteBuilder::new()
        .returning("id")
        .where_clause("name = $1")
        .as_string();
      let expected_query = "WHERE name = $1 RETURNING id";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_returning_clause() {
      let query = DeleteBuilder::new()
        .raw_before(DeleteClause::Returning, "delete from users")
        .returning("login")
        .as_string();
      let expected_query = "delete from users RETURNING login";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_returning_clause() {
      let query = DeleteBuilder::new()
        .returning("id")
        .raw_after(DeleteClause::Returning, ", login, name")
        .as_string();
      let expected_query = "RETURNING id , login, name";

      assert_eq!(query, expected_query);
    }
  }

  mod insert_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder::{InsertBuilder, InsertClause};

    #[test]
    fn method_returning_should_add_the_returning_clause() {
      let query = InsertBuilder::new().returning("*").as_string();
      let expected_query = "RETURNING *";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_returning_should_accumulate_values_on_consecutive_calls() {
      let query = InsertBuilder::new().returning("login").returning("name").as_string();
      let expected_query = "RETURNING login, name";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_returning_should_not_accumulate_arguments_with_the_same_content() {
      let query = InsertBuilder::new().returning("id").returning("id").as_string();
      let expected_query = "RETURNING id";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_returning_should_trim_space_of_the_argument() {
      let query = InsertBuilder::new().returning("  login  ").as_string();
      let expected_query = "RETURNING login";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_returning_should_be_after_values_clause() {
      let query = InsertBuilder::new()
        .insert_into("(login, name)")
        .returning("login")
        .values("('foo', 'Foo')")
        .as_string();
      let expected_query = "INSERT INTO (login, name) VALUES ('foo', 'Foo') RETURNING login";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_returning_clause() {
      let query = InsertBuilder::new()
        .raw_before(InsertClause::Returning, "values ('foo')")
        .returning("login")
        .as_string();
      let expected_query = "values ('foo') RETURNING login";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_returning_clause() {
      let query = InsertBuilder::new()
        .returning("id")
        .raw_after(InsertClause::Returning, ", login, name")
        .as_string();
      let expected_query = "RETURNING id , login, name";

      assert_eq!(query, expected_query);
    }
  }

  mod update_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder::{UpdateBuilder, UpdateClause};

    #[test]
    fn method_returning_should_add_the_returning_clause() {
      let query = UpdateBuilder::new().returning("*").as_string();
      let expected_query = "RETURNING *";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_returning_should_accumulate_values_on_consecutive_calls() {
      let query = UpdateBuilder::new().returning("login").returning("name").as_string();
      let expected_query = "RETURNING login, name";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_returning_should_not_accumulate_arguments_with_the_same_content() {
      let query = UpdateBuilder::new().returning("id").returning("id").as_string();
      let expected_query = "RETURNING id";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_returning_should_trim_space_of_the_argument() {
      let query = UpdateBuilder::new().returning("  login  ").as_string();
      let expected_query = "RETURNING login";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_returning_should_be_after_where_clause() {
      let query = UpdateBuilder::new()
        .returning("id")
        .where_clause("name = $1")
        .as_string();
      let expected_query = "WHERE name = $1 RETURNING id";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_returning_clause() {
      let query = UpdateBuilder::new()
        .raw_before(UpdateClause::Returning, "where login = $1")
        .returning("login")
        .as_string();
      let expected_query = "where login = $1 RETURNING login";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_returning_clause() {
      let query = UpdateBuilder::new()
        .returning("id")
        .raw_after(UpdateClause::Returning, ", login, name")
        .as_string();
      let expected_query = "RETURNING id , login, name";

      assert_eq!(query, expected_query);
    }
  }
}

#[cfg(feature = "postgresql")]
mod with_clause {
  mod delete_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder::{DeleteBuilder, DeleteClause};

    #[test]
    fn method_with_should_add_the_with_clause() {
      let deleted_users = DeleteBuilder::new()
        .delete_from("users")
        .where_clause("ative = false")
        .returning("id");
      let query = DeleteBuilder::new().with("id_list", deleted_users).as_string();
      let expected_query = "WITH id_list AS (DELETE FROM users WHERE ative = false RETURNING id)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_accept_inline_argument() {
      let query = DeleteBuilder::new()
        .with(
          "id_list",
          DeleteBuilder::new()
            .delete_from("users")
            .where_clause("ative = false")
            .returning("id"),
        )
        .as_string();
      let expected_query = "WITH id_list AS (DELETE FROM users WHERE ative = false RETURNING id)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_accumulate_values_on_consecutive_calls() {
      let deleted_users = DeleteBuilder::new().delete_from("users");
      let deleted_orders = DeleteBuilder::new().delete_from("orders");
      let query = DeleteBuilder::new()
        .with("deleted_users", deleted_users)
        .with("deleted_orders", deleted_orders)
        .as_string();
      let expected_query = "\
        WITH deleted_users AS (DELETE FROM users), \
             deleted_orders AS (DELETE FROM orders)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_trim_space_of_the_argument() {
      let query = DeleteBuilder::new()
        .with("  deleted_users  ", DeleteBuilder::new().delete_from("users"))
        .as_string();
      let expected_query = "WITH deleted_users AS (DELETE FROM users)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_with_should_be_after_raw() {
      let query = DeleteBuilder::new()
        .raw("/* the with clause */")
        .with("deleted_users", DeleteBuilder::new().delete_from("users"))
        .as_string();
      let expected_query = "\
        /* the with clause */ \
        WITH deleted_users AS (DELETE FROM users)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_with_clause() {
      let query = DeleteBuilder::new()
        .raw_before(DeleteClause::With, "/* the with clause */")
        .with("deleted_orders", DeleteBuilder::new().delete_from("orders"))
        .as_string();
      let expected_query = "\
        /* the with clause */ \
        WITH deleted_orders AS (DELETE FROM orders)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_with_clause() {
      let query = DeleteBuilder::new()
        .with("deleted_address", DeleteBuilder::new().delete_from("address"))
        .raw_after(DeleteClause::With, "select name, login")
        .as_string();
      let expected_query = "\
        WITH deleted_address AS (DELETE FROM address) \
        select name, login\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_delete_from_should_be_after_with_clause() {
      let query = DeleteBuilder::new()
        .with("deleted_address", DeleteBuilder::new().delete_from("address"))
        .delete_from("orders")
        .as_string();
      let expected_query = "\
        WITH deleted_address AS (DELETE FROM address) \
        DELETE FROM orders\
      ";

      assert_eq!(query, expected_query);
    }
  }

  mod insert_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder::{InsertBuilder, InsertClause};

    #[test]
    fn method_with_should_add_the_with_clause() {
      let inserted_users = InsertBuilder::new()
        .insert_into("users(login)")
        .values("('foo')")
        .returning("id");
      let query = InsertBuilder::new().with("id_list", inserted_users).as_string();
      let expected_query = "WITH id_list AS (INSERT INTO users(login) VALUES ('foo') RETURNING id)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_accept_inline_argument() {
      let query = InsertBuilder::new()
        .with(
          "id_list",
          InsertBuilder::new()
            .insert_into("users(login)")
            .values("('foo')")
            .returning("id"),
        )
        .as_string();
      let expected_query = "WITH id_list AS (INSERT INTO users(login) VALUES ('foo') RETURNING id)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_accumulate_values_on_consecutive_calls() {
      let inserted_users = InsertBuilder::new().insert_into("users");
      let inserted_orders = InsertBuilder::new().insert_into("orders");
      let query = InsertBuilder::new()
        .with("inserted_users", inserted_users)
        .with("inserted_orders", inserted_orders)
        .as_string();
      let expected_query = "\
        WITH inserted_users AS (INSERT INTO users), \
             inserted_orders AS (INSERT INTO orders)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_trim_space_of_the_argument() {
      let query = InsertBuilder::new()
        .with("  inserted_users  ", InsertBuilder::new().insert_into("users"))
        .as_string();
      let expected_query = "WITH inserted_users AS (INSERT INTO users)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_with_should_be_after_raw() {
      let query = InsertBuilder::new()
        .raw("/* the with clause */")
        .with("inserted_users", InsertBuilder::new().insert_into("users"))
        .as_string();
      let expected_query = "\
        /* the with clause */ \
        WITH inserted_users AS (INSERT INTO users)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_with_clause() {
      let query = InsertBuilder::new()
        .raw_before(InsertClause::With, "/* the with clause */")
        .with("inserted_orders", InsertBuilder::new().insert_into("orders"))
        .as_string();
      let expected_query = "\
        /* the with clause */ \
        WITH inserted_orders AS (INSERT INTO orders)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_with_clause() {
      let query = InsertBuilder::new()
        .with("inserted_address", InsertBuilder::new().insert_into("address"))
        .raw_after(InsertClause::With, "select name, login")
        .as_string();
      let expected_query = "\
        WITH inserted_address AS (INSERT INTO address) \
        select name, login\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_insert_into_should_be_after_with_clause() {
      let query = InsertBuilder::new()
        .with("inserted_address", InsertBuilder::new().insert_into("address"))
        .insert_into("orders")
        .as_string();
      let expected_query = "\
        WITH inserted_address AS (INSERT INTO address) \
        INSERT INTO orders\
      ";

      assert_eq!(query, expected_query);
    }
  }

  mod select_builder_with_clause {
    use pretty_assertions::assert_eq;
    use sql_query_builder::{SelectBuilder, SelectClause};

    #[test]
    fn method_with_should_add_the_with_clause() {
      let select_users = SelectBuilder::new().select("login").from("users");
      let query = SelectBuilder::new().with("user_list", select_users).as_string();
      let expected_query = "WITH user_list AS (SELECT login FROM users)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_accept_inline_argument() {
      let query = SelectBuilder::new()
        .with("user_list", SelectBuilder::new().select("login").from("users"))
        .as_string();
      let expected_query = "WITH user_list AS (SELECT login FROM users)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_accumulate_values_on_consecutive_calls() {
      let select_users = SelectBuilder::new().select("id, login").from("users");
      let select_users_id = SelectBuilder::new().select("id").from("user_list");
      let query = SelectBuilder::new()
        .with("user_list", select_users)
        .with("user_ids", select_users_id)
        .as_string();
      let expected_query = "\
      WITH user_list AS (SELECT id, login FROM users), user_ids AS (SELECT id FROM user_list)\
    ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_trim_space_of_the_argument() {
      let query = SelectBuilder::new()
        .with("  date  ", SelectBuilder::new().select("current_date"))
        .as_string();
      let expected_query = "WITH date AS (SELECT current_date)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_with_should_be_after_raw() {
      let select_base = SelectBuilder::new()
        .raw("select 123 as id union")
        .with("user_list", SelectBuilder::new().select("*").from("users"))
        .select("id");
      let query = select_base.as_string();
      let expected_query = "\
    select 123 as id union \
    WITH user_list AS (SELECT * FROM users) \
    SELECT id\
  ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_with_clause() {
      let query = SelectBuilder::new()
        .raw_before(SelectClause::With, "/* the users orders */")
        .with("orders_list", SelectBuilder::new().select("*").from("orders"))
        .as_string();
      let expected_query = "/* the users orders */ WITH orders_list AS (SELECT * FROM orders)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_with_clause() {
      let query = SelectBuilder::new()
        .with("address_list", SelectBuilder::new().select("*").from("address"))
        .raw_after(SelectClause::With, "select name, login")
        .as_string();
      let expected_query = "WITH address_list AS (SELECT * FROM address) select name, login";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_select_should_be_after_with_clause() {
      let select_users = SelectBuilder::new().select("*").from("users");
      let select_base = SelectBuilder::new().with("user_list", select_users).select("id");
      let query = select_base.as_string();
      let expected_query = "\
      WITH user_list AS (SELECT * FROM users) \
      SELECT id\
    ";

      assert_eq!(query, expected_query);
    }
  }

  mod update_builder {
    use pretty_assertions::assert_eq;
    use sql_query_builder::{UpdateBuilder, UpdateClause};

    #[test]
    fn method_with_should_add_the_with_clause() {
      let update_users = UpdateBuilder::new()
        .update("users")
        .where_clause("ative = false")
        .returning("id");
      let query = UpdateBuilder::new().with("id_list", update_users).as_string();
      let expected_query = "WITH id_list AS (UPDATE users WHERE ative = false RETURNING id)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_accept_inline_argument() {
      let query = UpdateBuilder::new()
        .with(
          "id_list",
          UpdateBuilder::new()
            .update("users")
            .where_clause("ative = false")
            .returning("id"),
        )
        .as_string();
      let expected_query = "WITH id_list AS (UPDATE users WHERE ative = false RETURNING id)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_accumulate_values_on_consecutive_calls() {
      let updated_users = UpdateBuilder::new().update("users");
      let updated_orders = UpdateBuilder::new().update("orders");
      let query = UpdateBuilder::new()
        .with("updated_users", updated_users)
        .with("updated_orders", updated_orders)
        .as_string();
      let expected_query = "\
        WITH updated_users AS (UPDATE users), \
             updated_orders AS (UPDATE orders)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_with_should_trim_space_of_the_argument() {
      let query = UpdateBuilder::new()
        .with("  updated_users  ", UpdateBuilder::new().update("users"))
        .as_string();
      let expected_query = "WITH updated_users AS (UPDATE users)";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_with_should_be_after_raw() {
      let query = UpdateBuilder::new()
        .raw("/* the with clause */")
        .with("updated_users", UpdateBuilder::new().update("users"))
        .as_string();
      let expected_query = "\
        /* the with clause */ \
        WITH updated_users AS (UPDATE users)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_before_should_add_raw_sql_before_with_clause() {
      let query = UpdateBuilder::new()
        .raw_before(UpdateClause::With, "/* the with clause */")
        .with("updated_orders", UpdateBuilder::new().update("orders"))
        .as_string();
      let expected_query = "\
        /* the with clause */ \
        WITH updated_orders AS (UPDATE orders)\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn method_raw_after_should_add_raw_sql_after_with_clause() {
      let query = UpdateBuilder::new()
        .with("updated_address", UpdateBuilder::new().update("address"))
        .raw_after(UpdateClause::With, "select name, login")
        .as_string();
      let expected_query = "\
        WITH updated_address AS (UPDATE address) \
        select name, login\
      ";

      assert_eq!(query, expected_query);
    }

    #[test]
    fn clause_update_should_be_after_with_clause() {
      let query = UpdateBuilder::new()
        .with("updated_address", UpdateBuilder::new().update("address"))
        .update("orders")
        .as_string();
      let expected_query = "\
        WITH updated_address AS (UPDATE address) \
        UPDATE orders\
      ";

      assert_eq!(query, expected_query);
    }
  }
}
