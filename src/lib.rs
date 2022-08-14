#![doc = include_str!("../README.md")]

mod behavior;
mod delete;
mod fmt;
mod insert;
mod select;
mod structure;
mod update;
mod values;

pub use crate::structure::{
  Delete, DeleteClause, Insert, InsertClause, Select, SelectClause, Update, UpdateClause, Values, ValuesClause,
};
