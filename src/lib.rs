#![feature(plugin)]
#![plugin(peg_syntax_ext)]
#![feature(drop_types_in_const)]

// #![deny(missing_docs,
//         missing_debug_implementations, missing_copy_implementations,
//         trivial_casts, trivial_numeric_casts,
//         unsafe_code,
//         unstable_features,
//         unused_import_braces, unused_qualifications)]

extern crate cairo;
extern crate gtk;
extern crate gdk;
extern crate cgmath;
extern crate rand;
extern crate kiss3d;
extern crate nalgebra;
extern crate rustc_serialize;
extern crate toml;
extern crate xplicit_types;
extern crate xplicit_primitive;
extern crate xplicit_tessellation;

pub mod render;
pub mod xplicit_widget;
pub mod editor;
pub mod openscad;
pub mod menu;
pub mod mesh_view;
pub mod settings;
