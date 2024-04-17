#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod chrom_fsmash;
mod chrom_dsmash;
mod chrom_fair;
mod chrom_dair;
mod chrom_uair;
mod chrom_bair;

#[skyline::main(name = "kain_moveset")]
pub fn main() {
    chrom_fsmash::install();
    chrom_dsmash::install();
    chrom_fair::install();
    chrom_bair::install();
    chrom_dair::install();
    chrom_uair::install();
}