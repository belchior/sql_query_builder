#![doc = include_str!("../README.md")]

mod behavior;
mod delete;
mod fmt;
mod insert;
mod select;
mod structure;
mod update;

pub use crate::structure::{
  DeleteBuilder, DeleteClause, InsertBuilder, InsertClause, SelectBuilder, SelectClause, UpdateBuilder, UpdateClause,
};
