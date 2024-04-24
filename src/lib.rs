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
mod win1b;
mod win1c;
mod win1d;
mod win1e;
mod win1f;

mod win2a;
mod win2b;
mod win2c;
mod win2d;
mod win2e;
mod win2f;
mod win2g;

mod win3a;
mod win3b;
mod win3c;
mod win3d;
mod win3e;

#[skyline::main(name = "wft_victoryfix_SL2")]
pub fn main() {
    win1a::install();
    win1b::install();
    win1c::install();
    win1d::install();
    win1e::install();
    win1f::install();

    win2a::install();
    win2b::install();
    win2c::install();
    win2d::install();
    win2e::install();
    win2f::install();
    win2g::install();

    win3a::install();
    win3b::install();
    win3c::install();
    win3d::install();
    win3e::install();
    
}