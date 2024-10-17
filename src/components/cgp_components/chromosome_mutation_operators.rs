use std::fs::File;
use std::rc::Rc;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use crate::components::cgp_components::cgp_node_mutation_operators::NodeMutationOperatorTrait;
use crate::components::cgp_components::cgp_types::CGPType;
use crate::components::cgp_components::chromosome::Chromosome;
use std::io::{BufWriter, Write};
pub trait ChromosomeMutationTrait {
    fn new() -> Box<dyn ChromosomeMutationTrait>
    where
        Self: Sized;
    fn execute(&self, chromosome: &mut Chromosome, mutate_function: Rc<Box<dyn NodeMutationOperatorTrait>>, output_file: &mut BufWriter<File>);
}

pub struct ChromosomeMutationSingle;

pub struct ChromosomeMutationPoint;

pub struct ChromosomeMutationMultiN;

pub struct ChromosomeMutationSplit;


impl ChromosomeMutationTrait for ChromosomeMutationSingle {
    fn new() -> Box<dyn ChromosomeMutationTrait>
    where
        Self: Sized,
    {
        Box::new(Self)
    }

    fn execute(&self, chromosome: &mut Chromosome, mutate_function: Rc<Box<dyn NodeMutationOperatorTrait>>, output_file: &mut BufWriter<File>) {
        let start_id = chromosome.params.nbr_inputs;
        let end_id = chromosome.params.nbr_inputs + chromosome.params.graph_width + chromosome.params.nbr_outputs;

        let between = Uniform::from(start_id..end_id);
        let mut rng = rand::thread_rng();

        let mut mutated_nodes: Vec<usize> = Vec::with_capacity(128);

        loop {
            let random_node_id = between.sample(&mut rng);

            mutated_nodes.push(random_node_id);

            if chromosome.params.cgp_type == CGPType::DAG {
                mutate_function.mutate_dag(&mut chromosome.nodes_grid[random_node_id], chromosome.cgp_edges.as_mut().unwrap());
            } else {
                mutate_function.mutate_standard(&mut chromosome.nodes_grid[random_node_id]);
            }
            if chromosome.active_nodes.contains(&random_node_id) {
                break;
            }
        }
        writeln!(*output_file, "{:?}", mutated_nodes.len()).expect("write not okay??");

    }
}

impl ChromosomeMutationTrait for ChromosomeMutationPoint {
    fn new() -> Box<dyn ChromosomeMutationTrait>
    where
        Self: Sized,
    {
        Box::new(Self)
    }

    fn execute(&self, chromosome: &mut Chromosome, mutate_function: Rc<Box<dyn NodeMutationOperatorTrait>>, output_file: &mut BufWriter<File>) {
        let start_id = chromosome.params.nbr_inputs;
        let end_id = chromosome.params.nbr_inputs + chromosome.params.graph_width + chromosome.params.nbr_outputs;

        let mut rng = rand::thread_rng();
        let between = Uniform::new(0., 1.);

        for node_id in start_id..end_id {
            let rand_val = between.sample(&mut rng);
            if rand_val <= chromosome.params.mutation_rate {
                if chromosome.params.cgp_type == CGPType::DAG {
                    mutate_function.mutate_dag(&mut chromosome.nodes_grid[node_id], chromosome.cgp_edges.as_mut().unwrap());
                } else {
                    mutate_function.mutate_standard(&mut chromosome.nodes_grid[node_id]);
                }
            }
        }
    }
}

impl ChromosomeMutationTrait for ChromosomeMutationMultiN {
    fn new() -> Box<dyn ChromosomeMutationTrait>
    where
        Self: Sized,
    {
        Box::new(Self)
    }

    fn execute(&self, chromosome: &mut Chromosome, mutate_function: Rc<Box<dyn NodeMutationOperatorTrait>>, output_file: &mut BufWriter<File>) {
        let start_id = chromosome.params.nbr_inputs;
        let end_id = chromosome.params.nbr_inputs + chromosome.params.graph_width + chromosome.params.nbr_outputs;

        let between = Uniform::from(start_id..end_id);
        let mut rng = rand::thread_rng();

        let mut mutated_active_nodes_counter = 0;

        let mut mutated_nodes: Vec<usize> = Vec::with_capacity(128);

        loop {
            let random_node_id = between.sample(&mut rng);
            mutated_nodes.push(random_node_id);

            if chromosome.params.cgp_type == CGPType::DAG {
                mutate_function.mutate_dag(&mut chromosome.nodes_grid[random_node_id], chromosome.cgp_edges.as_mut().unwrap());
            } else {
                mutate_function.mutate_standard(&mut chromosome.nodes_grid[random_node_id]);
            }

            if chromosome.active_nodes.contains(&random_node_id) {
                mutated_active_nodes_counter += 1;
            }

            if mutated_active_nodes_counter >= chromosome.params.BIOMA_nbr_mutations {
                break;
            }
        }
        writeln!(output_file, "{:?}", mutated_nodes.len()).expect("write not okay??");

    }
}

impl ChromosomeMutationTrait for ChromosomeMutationSplit {
    fn new() -> Box<dyn ChromosomeMutationTrait>
    where
        Self: Sized,
    {
        Box::new(Self)
    }

    fn execute(&self, chromosome: &mut Chromosome, mutate_function: Rc<Box<dyn NodeMutationOperatorTrait>>, output_file: &mut BufWriter<File>) {
        let start_id = chromosome.params.nbr_inputs;
        let end_id = chromosome.params.nbr_inputs + chromosome.params.graph_width + chromosome.params.nbr_outputs;

        let mut rng = rand::thread_rng();
        let between = Uniform::new(0., 1.);

        for node_id in start_id..end_id {
            let rand_val = between.sample(&mut rng);

            let mut mutate_flag = false;
            // Check if a node is active or not
            if chromosome.active_nodes.contains(&node_id) {
                if rand_val <= chromosome.params.BIOMA_prob_active_mutation {
                    mutate_flag = true;
                }
            } else {
                if rand_val <= chromosome.params.BIOMA_prob_inactive_mutation {
                    mutate_flag = true
                }
            }

            if mutate_flag {
                if chromosome.params.cgp_type == CGPType::DAG {
                    mutate_function.mutate_dag(&mut chromosome.nodes_grid[node_id], chromosome.cgp_edges.as_mut().unwrap());
                } else {
                    mutate_function.mutate_standard(&mut chromosome.nodes_grid[node_id]);
                }
            }
        }
    }
}