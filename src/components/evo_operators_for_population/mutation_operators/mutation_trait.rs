use std::fs::File;
use std::io::BufWriter;
use std::rc::Rc;
use crate::components::cgp_components::cgp_node_mutation_operators::NodeMutationOperatorTrait;
use crate::components::cgp_components::chromosome_mutation_operators::ChromosomeMutationTrait;
use crate::utils::runner::Runner;

pub trait GeneralMutatePopulationTrait<T> where T: Clone
{
    fn new() -> Box<dyn GeneralMutatePopulationTrait<T>> where Self: Sized;

    fn execute(&mut self, runner: &mut Runner<T>,
               node_mutation_op: Rc<Box<dyn NodeMutationOperatorTrait>>,
               chromosome_mutation_op: Rc<Box<dyn ChromosomeMutationTrait>>,
               output_file: &mut BufWriter<File>
    );
}