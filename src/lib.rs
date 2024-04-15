#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod chrom_fsmash;
mod chrom_fair;
mod chrom_dair;
mod chrom_uair;

#[skyline::main(name = "kain_moveset")]
pub fn main() {
    chrom_fsmash::install();
    chrom_fair::install();
    chrom_dair::install();
    chrom_uair::install();
}