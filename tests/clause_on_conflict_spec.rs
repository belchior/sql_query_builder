mod insert_command {
  use pretty_assertions::assert_eq;
  use sql_query_builder as sql;

  #[test]
  fn method_on_conflict_should_add_a_on_conflict_clause() {
    let query = sql::Insert::new().on_conflict("DO NOTHING").as_string();
    let expected_query = "ON CONFLICT DO NOTHING";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_on_conflict_should_override_value_on_consecutive_calls() {
    let query = sql::Insert::new()
      .on_conflict("do nothing")
      .on_conflict("on constraint users_name_key do nothing")
      .as_string();
    let expected_query = "ON CONFLICT on constraint users_name_key do nothing";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_on_conflict_should_trim_space_of_the_argument() {
    let query = sql::Insert::new().on_conflict("  DO NOTHING  ").as_string();
    let expected_query = "ON CONFLICT DO NOTHING";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_before_should_add_raw_sql_before_on_conflict_clause() {
    let query = sql::Insert::new()
      .raw_before(sql::InsertClause::OnConflict, "values ('foo', 'Foo')")
      .on_conflict("DO NOTHING")
      .as_string();
    let expected_query = "values ('foo', 'Foo') ON CONFLICT DO NOTHING";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn method_raw_after_should_add_raw_sql_after_on_conflict_clause() {
    let query = sql::Insert::new()
      .on_conflict("do nothing")
      .raw_after(sql::InsertClause::OnConflict, "/* raw after */")
      .as_string();
    let expected_query = "ON CONFLICT do nothing /* raw after */";

    assert_eq!(query, expected_query);
  }

  #[test]
  fn clause_on_conflict_should_be_after_values_clause() {
    let query = sql::Insert::new()
      .values("('foo', 'Foo')")
      .on_conflict("DO NOTHING")
      .as_string();
    let expected_query = "VALUES ('foo', 'Foo') ON CONFLICT DO NOTHING";

    assert_eq!(query, expected_query);
  }
}
