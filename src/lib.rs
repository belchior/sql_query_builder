#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

mod alter_table;
mod behavior;
mod concat;
mod create_table;
mod delete;
mod drop_table;
mod fmt;
mod insert;
mod select;
mod structure;
mod transaction;
mod update;
mod utils;
mod values;

pub use crate::structure::{
  AlterTable, AlterTableAction, CreateTable, CreateTableParams, Delete, DeleteClause, DropTable, DropTableParams,
  Insert, InsertClause, Select, SelectClause, Transaction, Update, UpdateClause, Values, ValuesClause,
};

#[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
mod create_index;
#[cfg(any(feature = "postgresql", feature = "sqlite"))]
mod drop_index;

#[cfg(any(feature = "postgresql", feature = "sqlite", feature = "mysql"))]
pub use crate::structure::{CreateIndex, CreateIndexParams};
#[cfg(any(feature = "postgresql", feature = "sqlite"))]
pub use crate::structure::{DropIndex, DropIndexParams};
