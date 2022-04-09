use rand::Rng;
use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};

use text_colorizer::*;


#[derive(Debug, Clone)]
pub struct Genome{
    
    dna : Vec<f32>,
    eval : f32,
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct GenomeList{

    list: Vec<Genome>,
    times: Vec<i32>,
    phase: usize,
    eval_max_history: Vec<f32>,
    eval_min_history: Vec<f32>,
    eval_ave_history: Vec<f32>,
    function: Vec< fn(&Vec<f32>) -> f32 >,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct GAvalues {
    genome_length : i32,          //遺伝子情報の長さ
    max_genome_list : i32,        //遺伝子集団の大きさ
    select_genome : usize,        //エリート遺伝子選択数
    individual_mutation : f32,    //個体突然変異確率
    genome_mutation : f32,        //遺伝子突然変異確率
    max_generation : i32,         //繰り返す世代数
}

#[allow(dead_code)]
fn get_type<T>(_: T) -> & 'static str{
    std::any::type_name::<T>()
}


fn sum(genome: &Vec<f32>) -> f32{

    let mut a: f32 = 0.0;
    for i in 0..genome.len(){
        a += genome[i];
    }
    a
}

fn get_ave(vec: &Vec<f32>) -> f32{

    sum(vec)/(vec.len() as f32)
}



pub mod ga_algorithm{

    use crate::Genome;
    pub trait Gene{

        fn new(length: usize, min: f32, max: f32, f: &Vec< fn(&Vec<f32>) -> f32 > ) -> Genome;
        fn mutate(gene:& mut Self, x: f32, min: f32, max: f32, f: &Vec< fn(&Vec<f32>) -> f32 >, phase: usize);
    }


    pub trait Func{

        fn add(f: fn(&Vec<f32>) -> f32, list: &mut Self);
    }


    pub trait Crossover{

        fn step_n(dna1: &Self, dna2: &Genome, n: usize, f: &Vec< fn(&Vec<f32>) -> f32 >, phase: usize) -> Genome;
        fn randomly(dna1: &Self, dna2: &Genome, f: &Vec< fn(&Vec<f32>) -> f32 >, phase: usize) -> Genome;
        fn average(dna1: &Self, dna2: &Genome, f: &Vec< fn(&Vec<f32>) -> f32 >, phase: usize) -> Genome;

    }


    pub trait Generation{

        fn new(amount: usize, length: usize, min: f32, max: f32, f: &Vec< fn(&Vec<f32>) -> f32 >) ->Self;
        fn couple(genomes:&Self, parents_number: usize) -> Vec<(&Genome, &Genome)>;
        fn order(genomes:& mut Self);
        fn create_next_generation(genomes: &mut Self, children: Vec<Genome>, gene_mutate: f32, min: f32, max: f32);

    //println!("{}", genomes.to_vec().len());
    }


    pub trait System{

        fn ga_loop(all: & mut Self, number: usize, cross_type: i8, genome_mutation: f32, min: f32, max: f32);

    }
}


use ga_algorithm::Gene;
use ga_algorithm::Func;
use ga_algorithm::Crossover;
use ga_algorithm::Generation;
use ga_algorithm::System;


impl Gene for Genome{

    fn new(length: usize, min: f32, max: f32, f: &Vec< fn(&Vec<f32>) -> f32 >) -> Genome{

        let mut v: Vec<f32> = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..length{
            let mut a: f32 = rng.gen();
            a = (max - min) * a + min;
            v.push(a);
        }

        if f.len() == 0{
            eprintln!("{} : {}", "Error".red(), "function is not given before");
            std::process::exit(1);
        }

        let ini_value = f[0](&v);

        let init = Genome{
            dna: v,
            eval: ini_value,
        };

        init
    }


    fn mutate(gene:& mut Self, x: f32, min: f32, max: f32, f: &Vec< fn(&Vec<f32>) -> f32 >, phase: usize){

        let mut rng = rand::thread_rng();
        let mut recount= false;

        let a : f32 =rng.gen();

        if a < x{

            for i in 0..gene.dna.len(){

                let b : f32 =rng.gen();
                
                if b < x{

                    let mut a: f32 = rng.gen();
                    a = (max - min) * a + min;
                    gene.dna[i] = a;
                    if recount == false{recount = true;}
                    
                }
            }
        }

        if recount == true{
            let val = f[phase-1](&gene.dna);
            gene.eval = val;
            //println!("{:?}", gene);
        }
    }
}


impl Func for GenomeList{

    fn add(f: fn(&Vec<f32>) -> f32, list: &mut Self){
        list.function.push(f);

    }

}



impl Crossover for Genome{

    fn step_n(dna1: &Self, dna2: &Genome, n: usize, f: &Vec< fn(&Vec<f32>) -> f32 >, phase: usize) -> Genome{

        let mut child_dna: Vec<f32> = Vec::new();
        let mut count: usize = 0;
        
        while count < dna1.dna.len(){

            for i in 0..n{
                if count == dna1.dna.len(){break;}
                child_dna.push(dna1.dna[i]);
                count+=1;
                
            }
            for i in 0..n{
                if count == dna1.dna.len(){break;}
                child_dna.push(dna2.dna[i+1]);
                count+=1;

            }
        }

        let childe_values: f32 = f[phase-1](&child_dna);

        let result = Genome{
            dna: child_dna,
            eval: childe_values,
        };

        result
    }

 
    fn randomly(dna1: &Self, dna2: &Genome, f: &Vec< fn(&Vec<f32>) -> f32 >, phase: usize) -> Genome{

        let mut child_dna: Vec<f32> = Vec::new();
        
        let mut rng = rand::thread_rng();
        
        for i in 0..dna1.dna.len(){

            let mut a: i32 = rng.gen();
            if a >= 0 {a = 1;}
            else {a = 0;}

            if a == 0{child_dna.push(dna1.dna[i]);}
            else if a == 1{child_dna.push(dna2.dna[i]);}

        }

        let childe_values: f32 = f[phase-1](&child_dna);

        Genome{
            dna: child_dna,
            eval: childe_values,
        }
    }

    fn average(dna1: &Self, dna2: &Genome, f: &Vec< fn(&Vec<f32>) -> f32 >, phase: usize) -> Genome{

        let mut child_dna: Vec<f32> = Vec::new();
        
        for i in 0..dna1.dna.len(){
            
            child_dna.push((dna1.dna[i]+dna2.dna[i])/2.0);
            
        }

        let childe_values: f32 = f[phase-1](&child_dna);

        Genome{
            dna: child_dna,
            eval: childe_values,
        }
    }
}






impl Generation for GenomeList{

    fn new(amount: usize, length: usize, min: f32, max: f32, f: &Vec< fn(&Vec<f32>) -> f32 >) ->Self{

        let mut list: Vec<Genome> = Vec::new();
        let mut values: Vec<f32> = Vec::new();

        let mut times: Vec<i32> = Vec::new();
        let mut val_max: Vec<f32> = Vec::new();
        let mut val_min: Vec<f32> = Vec::new();
        let mut val_ave: Vec<f32> = Vec::new();
        let mut func: Vec< fn(&Vec<f32>) -> f32 > = Vec::new();

        println!("{}        : {}", "Note".green(), "Creating first generation...");

        for _i in 0..amount{

            let genome = Genome::new(length, min, max, f);
            
            list.push(genome.clone());
            values.push(genome.eval);
        }

        for i in 0..f.len(){

            func.push(f[i]);
        }

        times.push(1);
        val_max.push(values.iter().fold(0.0/0.0, |m, v| v.max(m)));
        val_min.push(values.iter().fold(0.0/0.0, |m, v| v.min(m)));
        val_ave.push(get_ave(&values));

        GenomeList{

            list: list,
            times: times,
            phase: 1,
            eval_max_history: val_max,
            eval_min_history: val_min,
            eval_ave_history: val_ave,
            function: func,
        }
    }


    fn order(genomes:& mut Self){

        genomes.list.sort_by(|a, b| (a.eval).partial_cmp(&(b.eval)).unwrap());
    }
    


    fn couple(genomes:&Self, parents_number: usize) -> Vec<(&Genome, &Genome)>{

        let mut parents: Vec<(&Genome, &Genome)> = Vec::new();
        let mut rng = rand::thread_rng();

        for i in 0..parents_number{

            //think the case that parents_number is larger than genomes.len()
            parents.push((genomes.list.choose(&mut rng).unwrap(), &genomes.list[i]));
        }

        parents
    }

    fn create_next_generation(genomes: &mut Self, children: Vec<Genome>, gene_mutate: f32, min: f32, max: f32){

        let (a,b) = (genomes.list.len(), children.len());
        let mut _list: Vec<Genome> = genomes.list.splice(a-b..a, children).collect::<Vec<Genome>>();

        for i in 0..genomes.list.len(){

            Genome::mutate(&mut genomes.list[i], gene_mutate, min, max, &genomes.function, genomes.phase);
        }
    }

}

impl System for GenomeList{

    fn ga_loop(all: & mut Self, number: usize, cross_type: i8, genome_mutation: f32, min: f32, max: f32){

        let mut time: f32 = 0.0;

        println!("{}        : {}", "Note".green(), "GA loop start.");

        loop{

            GenomeList::order(all);
            let parents: Vec<(&Genome, &Genome)> = GenomeList::couple(all, number);
            let mut children: Vec<Genome> = Vec::new();
            for i in 0..parents.len(){

                if cross_type < -1 || cross_type > (all.list[0].dna.len() as usize).try_into().unwrap(){

                    println!("{}       : {}", "Error".red(), "The crossover type key is not corrrect.");
                    std::process::exit(1);
                }

                if cross_type > 0 {

                    children.push(Genome::step_n(parents[i].0, parents[i].1, cross_type as usize, &all.function, all.phase));
                }

                else if cross_type == 0{

                    children.push(Genome::randomly(parents[i].0, parents[i].1, &all.function, all.phase));
                }

                else if cross_type == -1{

                    children.push(Genome::average(parents[i].0, parents[i].1, &all.function, all.phase));
                }
            }

            GenomeList::create_next_generation(all,children, genome_mutation, min, max);

            if all.list[0].eval < 1e-4{

                eprintln!("{}      : {:?}", "Result".blue(), all.list[0]);
                println!("{} : {:?}","Generations".blue(), time);
                break;

            }

            time += 1.0;
            

            if time > 1e8{

                println!("{} : {}", "Warning : ".yellow(), "GA was stopped because generation times got over 1e10.");
                eprintln!("{}   :   {:?}", "Result".blue(), all.list[0]);
                break;

            }

            //println!("{:?}", all.list[0])

            
        }
    }
}
