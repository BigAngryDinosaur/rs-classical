mod fib;
mod nucleotide_compressor;

use nucleotide_compressor::NucleotideCompressor;

fn main() {
    //println!("{}", fib::fib_iter(10))
    //
    let n = NucleotideCompressor::new(String::from("ACT")).unwrap();
    println!("{:?} = {}", n, n);
}
