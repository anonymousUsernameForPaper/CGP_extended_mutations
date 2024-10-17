use std::rc::Rc;
use crate::components::cgp_components::chromosome_find_active_node_operators::ChromosomeActiveNodeTrait;
use crate::components::cgp_components::chromosome_reorder_operators::ChromosomeReorderTrait;
use crate::function_set::function_trait::FunctionTrait;
use crate::utils::runner::Runner;

pub trait GeneralReorderPopulationTrait<T> where T: Clone
{
    fn new() -> Box<dyn GeneralReorderPopulationTrait<T>> where Self: Sized;

    fn execute(&self,
               runner: &mut Runner<T>,
               chromosome_reorder_op: Rc<Box<dyn ChromosomeReorderTrait<T>>>,
               active_node_func: Rc<Box<dyn ChromosomeActiveNodeTrait<T>>>,
               function_set: Rc<Vec<Box<dyn FunctionTrait<T>>>>);
}

pub struct ReorderPopulation;

impl<T: Clone> GeneralReorderPopulationTrait<T> for ReorderPopulation {
    fn new() -> Box<dyn GeneralReorderPopulationTrait<T>> where Self: Sized {
        Box::new(Self)
    }

    fn execute(&self,
               runner: &mut Runner<T>,
               chromosome_reorder_op: Rc<Box<dyn ChromosomeReorderTrait<T>>>,
               active_node_func: Rc<Box<dyn ChromosomeActiveNodeTrait<T>>>,
               function_set: Rc<Vec<Box<dyn FunctionTrait<T>>>>) {
        for id in &runner.child_ids {
            chromosome_reorder_op.execute(&mut runner.population[*id],
                                          Rc::clone(&active_node_func),
                                          Rc::clone(&function_set));
        }
    }
}