use sea_query::{tests_cfg::*, *};

mod foreign_key;
mod index;
#[cfg(feature = "postgres-interval")]
mod interval;
mod query;
mod table;
// mod types;

#[path = "../common.rs"]
mod common;
use common::*;
