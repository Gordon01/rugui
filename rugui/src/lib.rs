#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![warn(clippy::all, rust_2018_idioms)]

pub mod coordinates;
pub mod framebuffer;
pub mod geometry;
pub mod widgets;
