use std::rc::Rc;
use crate::components::cgp_components::chromosome_find_active_node_operators::ChromosomeActiveNodeTrait;
use crate::function_set::function_trait::FunctionTrait;
use crate::utils::runner::Runner;

pub trait GeneralCrossoverTrait<T> where T: Clone
{
    fn new() -> Box<dyn GeneralCrossoverTrait<T>> where Self: Sized;

    fn execute(&self, runner: &mut Runner<T>,
               active_node_function: Rc<Box<dyn ChromosomeActiveNodeTrait<T>>>,
               function_set: Rc<Vec<Box<dyn FunctionTrait<T>>>>,);
}