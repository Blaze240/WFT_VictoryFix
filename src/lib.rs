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

mod win1;
mod win2;
mod win3;

#[skyline::main(name = "byleth_victoryfix_SL2")]
pub fn main() {
    win1::install();
    win2::install();
    win3::install();
}