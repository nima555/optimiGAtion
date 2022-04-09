# optimiGAtion

## Abstract

This is Rust tools for evolutionary computation, especialy GA. The tools focuses high level of versatility, so we can optimaze so much types of mutivariable problem via this.

## What is GA?

GA(Genetic Algorithm) is one of the most basic theory of evolutionary computation, which is used for optimazation mutivariable problem in the situation of reserch, design, and so on. This logic mainly contains four steps to get the answer you want. This is below,

- Create group of genomes
- Evaluate the avility of each genome via evaluate function
- Crossover
- Change group of genomes

In general, the next generation of genomes is greater than the last one because weak genome would be exchanged to new child genome which is expected as strong. After many evolution of gneomes group, the most strong genome in the group is the values you want to know. Of couece, the evaluate function would be set by you.

## How to use

If you want to use this tools, you should add optimiGAtion to dependencies in cargo.toml like

```Cargo.toml
[dependencies]
optimigation = "0.1.0"
```

Then, main.rs should be like this.

```main.rs
extern crate gene;
use gene::GenomeList;
use crate::gene::ga_algorithm::Generation;
use crate::gene::ga_algorithm::System;


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



    let mut world = GenomeList::new(1000, 10, 0.0, 10.0, &functions);               // initiating group of genome.
    GenomeList::ga_loop(&mut world, 200, 3, 0.05, 0.0, 10.0);                       // starting GA loop.

}
```

### GenomeList::new(usize, usize, f32, f32, &Vec< fn(&Vec) -> f32 >)

  these are arguments of this function.

1. The number of genomes in group.
2. The number of components of each genome.
3. Min of values.
4. Max of values.
5. Closure list of evaluation function.

### GenomeList::ga_loop(& mut Self, usize, i8, f32, f32, f32)

  these are arguments of this function.

1. The genome lists initiated by GenomeList::new()
2. The number of elite genomes selected as parent for crossover.
3. Key decides way of crossover.
4. Min of values.
5. Max of values.

Especialy third argument, the correspondence table of key and way of crossover is

|   key | way of crossover                        |
| ----: | :-------------------------------------- |
|    -1 | inherited average of parents component  |
|     0 | inherited components in units of random |
| n > 0 | inherited components in units of n      |

## Sample of Result

The result of the above main.rs would be this.

```
Note        : Creating first generation...
Note        : GA loop start.
Result      : Genome { dna: [9.99994, 9.9998865, 9.999953, 9.9998865, 9.999953, 9.9998865, 9.99994, 9.9998865, 9.999953, 9.9998865], eval: 8.2969666e-5 }
Generations : 13953.0
```

the all of theoritical values are 1.0, and GA can get quasi‐optimum solution.

## Next Release

- apply any types of values
- multi evaluate function
