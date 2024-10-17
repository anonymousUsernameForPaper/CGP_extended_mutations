use std::fmt::{Display, Formatter};
use crate::components::cgp_components::cgp_types::CGPType;
use crate::components::evo_operators_for_population::crossover_operators::crossover_types::CrossoverType;

#[derive(Clone)]
pub struct CgpParameters {
    pub cgp_type: CGPType,
    pub graph_width: usize,
    pub elitists: usize,
    pub population_size: usize,  // total pop-number: #elitsts + pop-size
    pub eval_after_iterations: usize,
    pub nbr_inputs: usize,
    pub nbr_outputs: usize,
    pub mutation_rate: f32,
    pub crossover_type: CrossoverType,
    pub crossover_rate: f32,
    pub multi_point_n: usize,
    pub tournament_size: usize,
    pub number_functions: usize,
    pub fitness_threshold: f32,
    pub multi_n_number_mutations: usize,
    pub split_mutation_rate_active: f32,
    pub split_mutation_rate_inactive: f32,
}


impl Display for CgpParameters {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "############ Parameters ############\n")?;
        write!(f, "CGP Type: {}\n", self.cgp_type)?;
        write!(f, "graph_width: {}\n", self.graph_width)?;
        write!(f, "mu: {}\n", self.elitists)?;
        write!(f, "lambda: {}\n", self.population_size)?;
        write!(f, "eval_after_iterations: {}\n", self.eval_after_iterations)?;
        write!(f, "nbr_inputs: {}\n", self.nbr_inputs)?;
        write!(f, "nbr_outputs: {}\n", self.nbr_outputs)?;
        write!(f, "mutation_rate: {}\n", self.mutation_rate)?;
        write!(f, "crossover_type: {}\n", self.crossover_type)?;
        write!(f, "crossover_rate: {}\n", self.crossover_rate)?;
        write!(f, "multi_point_n: {}\n", self.multi_point_n)?;
        write!(f, "fitness_threshold: {}\n", self.fitness_threshold)?;
        write!(f, "#########################\n")
    }
}