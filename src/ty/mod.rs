//! Default configurations and factories.

#[cfg(feature = "null")]
pub mod null;

#[cfg(feature = "plain")]
pub mod plain;

#[cfg(feature = "term")]
pub mod term;
