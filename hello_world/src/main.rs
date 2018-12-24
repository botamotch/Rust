extern crate num;

use num::Complex;

fn main() {
    let v = vec![1, 2, 3];
    let x = Complex::new(1, 3);

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
        println!("    norm : {}", xx.norm_sqr());
        println!("    conj : {}", xx.conj());
    }
    println!(" ==================================================");
}
