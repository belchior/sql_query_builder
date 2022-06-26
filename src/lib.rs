#![doc = include_str!("../README.md")]

mod behavior;
mod fmt;
mod insert;
mod select;
mod structure;
mod update;

pub use crate::structure::{InsertBuilder, InsertClause, SelectBuilder, SelectClause, UpdateBuilder, UpdateClause};
