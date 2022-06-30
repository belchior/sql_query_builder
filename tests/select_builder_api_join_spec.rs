use pretty_assertions::assert_eq;
use sql_query_builder::{SelectBuilder, SelectClause};

// Raw after method

#[test]
fn method_raw_after_should_add_raw_sql_after_join_clause() {
  let query = SelectBuilder::new()
    .inner_join("address ON users.login = address.login")
    .raw_after(SelectClause::Join, "where id = $1")
    .as_string();
  let expected_query = "INNER JOIN address ON users.login = address.login where id = $1";

  assert_eq!(query, expected_query);
}

// Raw before method

#[test]
fn method_raw_before_should_add_raw_sql_before_join_clause() {
  let query = SelectBuilder::new()
    .raw_before(SelectClause::Join, "from orders")
    .inner_join("address ON address.user_login = orders.user_login")
    .as_string();
  let expected_query = "from orders INNER JOIN address ON address.user_login = orders.user_login";

  assert_eq!(query, expected_query);
}

mod cross_join_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_cross_join_should_add_the_cross_join_clause() {
    let query = SelectBuilder::new().cross_join("address").as_string();
    let expected_query = "CROSS JOIN address";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_cross_join_should_accumulate_values_on_consecutive_calls() {
    let query = SelectBuilder::new()
      .cross_join("address")
      .cross_join("orders")
      .as_string();
    let expected_query = "\
      CROSS JOIN address \
      CROSS JOIN orders\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_cross_join_by_should_trim_space_of_the_argument() {
    let query = SelectBuilder::new().cross_join("  orders  ").as_string();
    let expected_query = "CROSS JOIN orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_cross_join_should_not_accumulate_arguments_with_the_same_content() {
    let query = SelectBuilder::new()
      .cross_join("address")
      .cross_join("address")
      .as_string();
    let expected_query = "CROSS JOIN address";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_cross_join_should_be_after_from_clause() {
    let query = SelectBuilder::new().from("users").cross_join("address").as_string();
    let expected_query = "FROM users CROSS JOIN address";

    assert_eq!(query, expected_query);
  }
}

mod inner_join_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_inner_join_should_add_the_inner_join_clause() {
    let query = SelectBuilder::new()
      .inner_join("address ON users.login = address.login")
      .as_string();
    let expected_query = "INNER JOIN address ON users.login = address.login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_inner_join_should_accumulate_values_on_consecutive_calls() {
    let query = SelectBuilder::new()
      .inner_join("address ON users.login = address.login")
      .inner_join("orders ON users.login = orders.login")
      .as_string();
    let expected_query = "\
      INNER JOIN address ON users.login = address.login \
      INNER JOIN orders ON users.login = orders.login\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_inner_join_by_should_trim_space_of_the_argument() {
    let query = SelectBuilder::new().inner_join("  orders  ").as_string();
    let expected_query = "INNER JOIN orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_inner_join_should_not_accumulate_arguments_with_the_same_content() {
    let query = SelectBuilder::new()
      .inner_join("address")
      .inner_join("address")
      .as_string();
    let expected_query = "INNER JOIN address";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_inner_join_should_be_after_from_clause() {
    let query = SelectBuilder::new()
      .from("users")
      .inner_join("address ON users.login = address.login")
      .as_string();
    let expected_query = "FROM users INNER JOIN address ON users.login = address.login";

    assert_eq!(query, expected_query);
  }
}

mod left_join_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_left_join_should_add_the_left_join_clause() {
    let query = SelectBuilder::new()
      .left_join("address ON users.login = address.login")
      .as_string();
    let expected_query = "LEFT JOIN address ON users.login = address.login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_left_join_should_accumulate_values_on_consecutive_calls() {
    let query = SelectBuilder::new()
      .left_join("address ON users.login = address.login")
      .left_join("orders ON users.login = orders.login")
      .as_string();
    let expected_query = "\
      LEFT JOIN address ON users.login = address.login \
      LEFT JOIN orders ON users.login = orders.login\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_left_join_by_should_trim_space_of_the_argument() {
    let query = SelectBuilder::new().left_join("  orders  ").as_string();
    let expected_query = "LEFT JOIN orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_left_join_should_not_accumulate_arguments_with_the_same_content() {
    let query = SelectBuilder::new()
      .left_join("address")
      .left_join("address")
      .as_string();
    let expected_query = "LEFT JOIN address";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_left_join_should_be_after_from_clause() {
    let query = SelectBuilder::new()
      .from("users")
      .left_join("address ON users.login = address.login")
      .as_string();
    let expected_query = "FROM users LEFT JOIN address ON users.login = address.login";

    assert_eq!(query, expected_query);
  }
}

mod right_join_clause {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn method_right_join_should_add_the_right_join_clause() {
    let query = SelectBuilder::new()
      .right_join("address ON users.login = address.login")
      .as_string();
    let expected_query = "RIGHT JOIN address ON users.login = address.login";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_right_join_should_accumulate_values_on_consecutive_calls() {
    let query = SelectBuilder::new()
      .right_join("address ON users.login = address.login")
      .right_join("orders ON users.login = orders.login")
      .as_string();
    let expected_query = "\
      RIGHT JOIN address ON users.login = address.login \
      RIGHT JOIN orders ON users.login = orders.login\
    ";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_right_join_by_should_trim_space_of_the_argument() {
    let query = SelectBuilder::new().right_join("  orders  ").as_string();
    let expected_query = "RIGHT JOIN orders";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_right_join_should_not_accumulate_arguments_with_the_same_content() {
    let query = SelectBuilder::new()
      .right_join("address")
      .right_join("address")
      .as_string();
    let expected_query = "RIGHT JOIN address";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_right_join_should_be_after_from_clause() {
    let query = SelectBuilder::new()
      .from("users")
      .right_join("address ON users.login = address.login")
      .as_string();
    let expected_query = "FROM users RIGHT JOIN address ON users.login = address.login";

    assert_eq!(query, expected_query);
  }
}
