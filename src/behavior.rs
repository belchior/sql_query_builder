use crate::concat::Concat;

/// Represents all commands that can be used in a transaction
pub trait TransactionQuery: Concat {}

/// Represents all commands that can be used inside the with method
#[cfg(any(feature = "postgresql", feature = "sqlite"))]
pub trait WithQuery: Concat {}
