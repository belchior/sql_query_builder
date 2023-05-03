use pretty_assertions::assert_eq;
use sql_query_builder as sql;

#[test]
fn all_clauses_concatenated_in_order() {
  let query = sql::Select::new()
    .raw("/* all clauses in order */")
    .select("*")
    .from("user_list")
    .inner_join("orders ON users.login = orders.login")
    .where_clause("user.login = $1")
    .group_by("login")
    .having("active = true")
    .order_by("created_at desc")
    .as_string();

  let expected_query = "\
    /* all clauses in order */ \
    SELECT * \
    FROM user_list \
    INNER JOIN orders ON users.login = orders.login \
    WHERE user.login = $1 \
    GROUP BY login \
    HAVING active = true \
    ORDER BY created_at desc\
  ";

  assert_eq!(query, expected_query);
}

mod builder_methods {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_new_should_initialize_as_empty_string() {
    let query = sql::Select::new().as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_as_string_should_convert_the_current_state_into_string() {
    let query = sql::Select::new().as_string();
    let expected_query = "";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_debug_should_print_at_console_in_a_human_readable_format() {
    let query = sql::Select::new().select("current_date").debug().as_string();
    let expected_query = "SELECT current_date";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_print_should_print_in_one_line_the_current_state_of_builder() {
    let query = sql::Select::new().select("1 + 2").print().as_string();
    let expected_query = "SELECT 1 + 2";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_add_raw_sql() {
    let query = sql::Select::new().raw("select id from users").as_string();
    let expected_query = "select id from users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Select::new().raw("select id").raw("from users").as_string();
    let expected_query = "select id from users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_be_the_first_to_be_concatenated() {
    let query = sql::Select::new().raw("select *").from("users").as_string();
    let expected_query = "select * FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_trim_space_of_the_argument() {
    let query = sql::Select::new().raw("  update users  ").as_string();
    let expected_query = "update users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Select::new()
      .raw("select login, name")
      .raw("select login, name")
      .as_string();
    let expected_query = "select login, name";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_trim_space_of_the_argument() {
    let query = sql::Select::new()
      .raw_after(sql::SelectClause::Select, "  from orders  ")
      .as_string();
    let expected_query = "from orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_trim_space_of_the_argument() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::Where, "  from address  ")
      .as_string();
    let expected_query = "from address";

    assert_eq!(query, expected_query);
  }
}

mod and_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_and_should_be_an_alias_to_where_clause() {
    let query = sql::Select::new().and("login = 'foo'").as_string();
    let expected_query = "WHERE login = 'foo'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_and_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Select::new().and("login = 'foo'").and("active = true").as_string();
    let expected_query = "WHERE login = 'foo' AND active = true";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_and_should_trim_space_of_the_argument() {
    let query = sql::Select::new().and("  name = 'Bar'  ").as_string();
    let expected_query = "WHERE name = 'Bar'";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_and_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Select::new()
      .and("status = 'success'")
      .and("status = 'success'")
      .as_string();
    let expected_query = "WHERE status = 'success'";

    assert_eq!(query, expected_query);
  }
}

mod from_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_from_should_add_the_from_clause() {
    let query = sql::Select::new().from("users").as_string();
    let expected_query = "FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_from_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Select::new().from("users").from("address").as_string();
    let expected_query = "FROM users, address";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_from_should_trim_space_of_the_argument() {
    let query = sql::Select::new().from("  users  ").as_string();
    let expected_query = "FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_from_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Select::new().from("address").from("address").as_string();
    let expected_query = "FROM address";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_from_should_be_after_select_clause() {
    let query = sql::Select::new().select("*").from("users").as_string();
    let expected_query = "SELECT * FROM users";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_from_clause() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::From, "select amount")
      .from("orders")
      .as_string();
    let expected_query = "select amount FROM orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_from_clause() {
    let query = sql::Select::new()
      .from("users")
      .raw_after(
        sql::SelectClause::From,
        "inner join address on users.login = address.login",
      )
      .as_string();
    let expected_query = "FROM users inner join address on users.login = address.login";

    assert_eq!(query, expected_query);
  }
}

mod group_by_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_group_by_should_add_the_group_by_clause() {
    let query = sql::Select::new().group_by("id, login").as_string();
    let expected_query = "GROUP BY id, login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_group_by_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Select::new()
      .group_by("id, login")
      .group_by("created_at")
      .as_string();
    let expected_query = "GROUP BY id, login, created_at";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_group_by_should_trim_space_of_the_argument() {
    let query = sql::Select::new().group_by("  id, login  ").as_string();
    let expected_query = "GROUP BY id, login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_group_by_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Select::new().group_by("status").group_by("status").as_string();
    let expected_query = "GROUP BY status";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_group_by_should_be_after_where_clause() {
    let query = sql::Select::new()
      .group_by("login")
      .where_clause("login = $1")
      .as_string();
    let expected_query = "\
    WHERE login = $1 \
    GROUP BY login\
  ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_group_by_clause() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::GroupBy, "where id = $1")
      .group_by("login")
      .as_string();
    let expected_query = "where id = $1 GROUP BY login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_group_by_clause() {
    let query = sql::Select::new()
      .group_by("login")
      .raw_after(sql::SelectClause::GroupBy, "LIMIT 10")
      .as_string();
    let expected_query = "GROUP BY login LIMIT 10";

    assert_eq!(query, expected_query);
  }
}

mod having_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_having_should_add_the_having_clause() {
    let query = sql::Select::new().having("active = true").as_string();
    let expected_query = "HAVING active = true";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_having_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Select::new()
      .having("active = true")
      .having("allow = true")
      .as_string();
    let expected_query = "HAVING active = true AND allow = true";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_having_should_trim_space_of_the_argument() {
    let query = sql::Select::new().having("  sum(amount) > 500  ").as_string();
    let expected_query = "HAVING sum(amount) > 500";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_having_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Select::new()
      .having("active = true")
      .having("active = true")
      .as_string();
    let expected_query = "HAVING active = true";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_having_should_be_after_group_by_clause() {
    let query = sql::Select::new().having("active = true").group_by("login").as_string();
    let expected_query = "\
    GROUP BY login \
    HAVING active = true\
  ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_having_clause() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::Having, "group by id")
      .having("active = true")
      .as_string();
    let expected_query = "group by id HAVING active = true";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_having_clause() {
    let query = sql::Select::new()
      .having("active = true")
      .raw_after(sql::SelectClause::Having, "LIMIT 10")
      .as_string();
    let expected_query = "HAVING active = true LIMIT 10";

    assert_eq!(query, expected_query);
  }
}

mod order_by_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_order_by_should_add_the_order_by_clause() {
    let query = sql::Select::new().order_by("id asc").as_string();
    let expected_query = "ORDER BY id asc";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_order_by_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Select::new()
      .order_by("login asc")
      .order_by("created_at desc")
      .as_string();
    let expected_query = "ORDER BY login asc, created_at desc";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_order_by_should_trim_space_of_the_argument() {
    let query = sql::Select::new().order_by("  id desc  ").as_string();
    let expected_query = "ORDER BY id desc";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_order_by_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Select::new().order_by("id desc").order_by("id desc").as_string();
    let expected_query = "ORDER BY id desc";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_order_by_should_be_after_having_clause() {
    let query = sql::Select::new()
      .having("active = true")
      .order_by("created_at desc")
      .as_string();
    let expected_query = "HAVING active = true ORDER BY created_at desc";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_order_by_clause() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::OrderBy, "where orders.user_login = $1")
      .order_by("id desc")
      .as_string();
    let expected_query = "where orders.user_login = $1 ORDER BY id desc";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_order_by_clause() {
    let query = sql::Select::new()
      .order_by("id desc")
      .raw_after(sql::SelectClause::OrderBy, "limit 20")
      .as_string();
    let expected_query = "ORDER BY id desc limit 20";

    assert_eq!(query, expected_query);
  }
}

mod select_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_select_should_add_the_select_clause() {
    let query = sql::Select::new().select("id, login").as_string();
    let expected_query = "SELECT id, login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_select_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Select::new().select("id, login").select("created_at").as_string();
    let expected_query = "SELECT id, login, created_at";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_select_by_should_trim_space_of_the_argument() {
    let query = sql::Select::new().select("  login, name  ").as_string();
    let expected_query = "SELECT login, name";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_select_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Select::new()
      .select("login, name")
      .select("login, name")
      .as_string();
    let expected_query = "SELECT login, name";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_select_clause() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::Select, "/* list orders */")
      .select("id, name")
      .as_string();
    let expected_query = "/* list orders */ SELECT id, name";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_select_clause() {
    let query = sql::Select::new()
      .select("id, name")
      .raw_after(sql::SelectClause::Select, "from address")
      .as_string();
    let expected_query = "SELECT id, name from address";

    assert_eq!(query, expected_query);
  }
}

mod where_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_where_should_add_the_where_clause() {
    let query = sql::Select::new()
      .where_clause("created_at::date = current_date")
      .as_string();
    let expected_query = "WHERE created_at::date = current_date";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_where_should_accumulate_values_on_consecutive_calls() {
    let query = sql::Select::new()
      .where_clause("created_at::date > current_date - INTERVAL '2 days'")
      .where_clause("created_at::date <= current_date")
      .as_string();
    let expected_query = "\
      WHERE \
        created_at::date > current_date - INTERVAL '2 days' \
        AND created_at::date <= current_date\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_where_by_should_trim_space_of_the_argument() {
    let query = sql::Select::new().where_clause("  id = $1  ").as_string();
    let expected_query = "WHERE id = $1";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_where_should_not_accumulate_arguments_with_the_same_content() {
    let query = sql::Select::new()
      .where_clause("active = true")
      .where_clause("active = true")
      .as_string();
    let expected_query = "WHERE active = true";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_where_should_be_after_any_of_the_joins_clauses() {
    let query = sql::Select::new()
      .inner_join("address ON users.login = address.login")
      .where_clause("user.login = $1")
      .as_string();
    let expected_query = "INNER JOIN address ON users.login = address.login WHERE user.login = $1";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_where_clause() {
    let query = sql::Select::new()
      .raw_before(sql::SelectClause::Where, "from orders")
      .where_clause("created_at::date = current_date")
      .as_string();
    let expected_query = "from orders WHERE created_at::date = current_date";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_where_clause() {
    let query = sql::Select::new()
      .where_clause("created_at::date = current_date")
      .raw_after(sql::SelectClause::Where, "limit 10")
      .as_string();
    let expected_query = "WHERE created_at::date = current_date limit 10";

    assert_eq!(query, expected_query);
  }
}
