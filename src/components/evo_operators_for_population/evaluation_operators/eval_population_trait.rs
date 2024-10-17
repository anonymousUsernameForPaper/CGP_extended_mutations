use std::rc::Rc;
use crate::components::cgp_components::chromosome_evaluator_operators::EvaluateChromosomeTrait;
use crate::components::cgp_components::chromosome_find_active_node_operators::ChromosomeActiveNodeTrait;
use crate::function_set::function_trait::FunctionTrait;
use crate::utils::runner::Runner;

pub trait GeneralForwardPassPopulationTrait<T> where T: Clone
{
    fn new() -> Box<dyn GeneralForwardPassPopulationTrait<T>> where Self: Sized;

    fn execute(&self, runner: &mut Runner<T>,
               evaluator_function: Rc<Box<dyn EvaluateChromosomeTrait<T>>>,
               active_node_func: Rc<Box<dyn ChromosomeActiveNodeTrait<T>>>,
               function_set: Rc<Vec<Box<dyn FunctionTrait<T>>>>);

    fn execute_test_set(&self, runner: &mut Runner<T>,
                                       evaluator_function: Rc<Box<dyn EvaluateChromosomeTrait<T>>>,
                                       active_node_func: Rc<Box<dyn ChromosomeActiveNodeTrait<T>>>,
                                       function_set: Rc<Vec<Box<dyn FunctionTrait<T>>>>) -> f32;
}

// pub trait GeneralTestPopulationTrait<T> where T: Clone
// {
//     fn new() -> Box<dyn GeneralTestPopulationTrait<T>> where Self: Sized;
//
//     fn execute(&self, runner: &mut Runner<T>,
//                evaluator_function: Rc<Box<dyn EvaluateChromosomeTrait<T>>>,
//                active_node_func: Rc<Box<dyn ChromosomeActiveNodeTrait<T>>>,
//                function_set: Rc<Vec<Box<dyn FunctionTrait<T>>>>) -> f32;
// }