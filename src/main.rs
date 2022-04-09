
extern crate optimigation;
use optimigation::GenomeList;

use crate::optimigation::ga_algorithm::Generation;
use crate::optimigation::ga_algorithm::System;

fn sum(genome: &Vec<f32>) -> f32{

    let mut a: f32 = 0.0;
    for i in 0..genome.len(){
        a += genome[i];
    }
    a
}

fn main() {

    let c1: fn(&Vec<f32>) -> f32 = |x| {(sum(x)/x.len() as f32 - 10.0).abs()};
    let mut functions = Vec::new();
    functions.push(c1);


    let mut world = GenomeList::new(1000, 10, 0.0, 10.0, &functions);
    GenomeList::ga_loop(&mut world, 200, 1, 0.05, 0.0, 10.0);

}
