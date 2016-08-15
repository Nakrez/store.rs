//! This is the external API for store.rs

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![feature(custom_derive, plugin)]

#![cfg_attr(test, plugin(stainless))]
#![plugin(serde_macros)]

pub mod db;
