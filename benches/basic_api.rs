#![feature(test)]
/// To run this benchmark you must use the nightly version:
/// rustup override set nightly
extern crate test;

#[cfg(test)]
mod any_feature {
  use sql_query_builder as sql;
  use test::{black_box, Bencher};

  #[bench]
  fn alter_table_builder(b: &mut Bencher) {
    b.iter(|| {
      sql::AlterTable::new()
        .alter_table(black_box("users"))
        .add(black_box("COLUMN id serial primary key"))
        .add(black_box("COLUMN login varchar(40) not null"))
        .drop(black_box("CONSTRAINT users_login_key"))
        .as_string()
    })
  }

  #[bench]
  fn create_table_builder(b: &mut Bencher) {
    b.iter(|| {
      sql::CreateTable::new()
        .create_table(black_box("users"))
        .column(black_box("id serial primary key"))
        .column(black_box("login varchar(40) not null"))
        .constraint(black_box("users_login_key unique(login)"))
        .as_string()
    })
  }

  #[bench]
  fn delete_builder(b: &mut Bencher) {
    b.iter(|| sql::DropTable::new().drop_table(black_box("users")).as_string())
  }

  #[bench]
  fn drop_table_builder(b: &mut Bencher) {
    b.iter(|| {
      sql::Delete::new()
        .delete_from(black_box("users"))
        .where_clause(black_box("id = $1"))
        .as_string()
    })
  }

  #[bench]
  fn insert_builder(b: &mut Bencher) {
    b.iter(|| {
      sql::Insert::new()
        .insert_into(black_box("users (login, name)"))
        .values(black_box("('foo', 'Foo')"))
        .values(black_box("('bar', 'Bar')"))
        .as_string()
    })
  }

  #[bench]
  fn select_builder(b: &mut Bencher) {
    b.iter(|| {
      sql::Select::new()
        .select(black_box("*"))
        .from(black_box("users"))
        .inner_join(black_box("orders ON users.login = orders.login"))
        .where_clause(black_box("user.login = $1"))
        .order_by(black_box("created_at desc"))
        .as_string()
    })
  }

  #[bench]
  fn update_builder(b: &mut Bencher) {
    b.iter(|| {
      sql::Update::new()
        .update(black_box("users"))
        .set(black_box("name = 'Bar'"))
        .where_clause(black_box("id = $1"))
        .as_string()
    })
  }

  #[bench]
  fn values_builder(b: &mut Bencher) {
    b.iter(|| {
      sql::Values::new()
        .values(black_box("('foo', 'Foo')"))
        .values(black_box("('bar', 'Bar')"))
        .as_string()
    })
  }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
#[cfg(test)]
mod postgresql_or_sqlite {
  use sql_query_builder as sql;
  use test::{black_box, Bencher};

  #[bench]
  fn create_index_builder(b: &mut Bencher) {
    b.iter(|| {
      sql::CreateIndex::new()
        .create_index(black_box("users_name_idx"))
        .on(black_box("users"))
        .column(black_box("name"))
        .as_string()
    })
  }

  #[bench]
  fn drop_index_builder(b: &mut Bencher) {
    b.iter(|| sql::DropIndex::new().drop_index("users_name_idx").as_string())
  }
}

#[cfg(not(feature = "sqlite"))]
#[cfg(test)]
mod not_sqlite {
  use sql_query_builder as sql;
  use test::{black_box, Bencher};

  #[bench]
  fn transaction_builder(b: &mut Bencher) {
    b.iter(|| {
      let insert_foo = sql::Insert::new()
        .insert_into(black_box("users (login, name)"))
        .values(black_box("('foo', 'Foo')"));

      let update_foo = sql::Update::new()
        .update(black_box("users"))
        .set(black_box("name = 'Bar'"))
        .where_clause(black_box("login = 'foo'"));

      sql::Transaction::new()
        .start_transaction(black_box("isolation level serializable"))
        .insert(insert_foo)
        .update(update_foo)
        .commit(black_box("transaction"))
        .as_string()
    })
  }
}
