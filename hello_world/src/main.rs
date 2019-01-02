extern crate num;
extern crate gnuplot;

use num::Complex;
// use gnuplot::{Figure, Caption, Color};

fn main() {
    let v = vec![1.1, 1.9, 3.2];
    let x = Complex::new(1.1, 3.1);

    println!(" ==================================================");
    println!("  Hello, world!");
    println!("  Goodbye, honey");
    for &i in v.iter() {
        println!("  > {}", i)
    }

    println!(" ==================================================");
    for &i in v.iter() {
        let xx = x.scale(i);
        println!("  x      : {}", xx);
        println!("    norm : {}", xx.norm());
        println!("    conj : {}", xx.conj());
    }
    println!(" ==================================================");
}
