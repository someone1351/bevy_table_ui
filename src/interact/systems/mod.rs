mod focusing;
mod selecting;
// mod hovering;
mod cursoring;

pub use focusing::*;
pub use selecting::*;
// pub use hovering::*;
pub use cursoring::*;

/*
TODO

* look at parent query with layoutcomputed, some entities might be mistaken as root because it lacks a layoutcomputed component?

* handle entity moved to a dif root, by check it's root is the same as its cur stored root
*/