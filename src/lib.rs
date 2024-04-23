#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
	unused_macros,
	unused_variables,
	unused_assignments,
	unused_unsafe,
	non_upper_case_globals,
	non_snake_case,
    clippy::borrow_interior_mutable_const
)]

mod win1a;
mod win2a;
mod win3a;

#[skyline::main(name = "robin_victoryfix_SL2")]
pub fn main() {
    win1a::install();
    win2a::install();
    win3a::install();
}