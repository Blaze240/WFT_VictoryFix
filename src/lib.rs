#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod chrom_fsmash;
mod chrom_fair;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    chrom_fsmash::install();
    chrom_fair::install();
}