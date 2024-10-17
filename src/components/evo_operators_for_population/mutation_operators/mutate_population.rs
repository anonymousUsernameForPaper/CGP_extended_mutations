use std::fs::File;
use std::rc::Rc;
use crate::components::cgp_components::cgp_node_mutation_operators::NodeMutationOperatorTrait;
use crate::components::cgp_components::chromosome_mutation_operators::ChromosomeMutationTrait;
use crate::components::evo_operators_for_population::mutation_operators::mutation_trait::GeneralMutatePopulationTrait;
use crate::utils::runner::Runner;
use std::io::BufWriter;
pub struct EAMutateStandard;

impl<T: Clone> GeneralMutatePopulationTrait<T> for EAMutateStandard {
    fn new() -> Box<dyn GeneralMutatePopulationTrait<T>> where Self: Sized {
        Box::new(Self)
    }

    fn execute(&mut self, runner: &mut Runner<T>,
               node_mutation_op: Rc<Box<dyn NodeMutationOperatorTrait>>,
               chromosome_mutation_op: Rc<Box<dyn ChromosomeMutationTrait>>,
               output_file: &mut BufWriter<File>
    ) {
        for id in &runner.child_ids {
            chromosome_mutation_op.execute(&mut runner.population[*id],
                                           Rc::clone(&node_mutation_op),
                                            output_file,
            );
        }
    }
}