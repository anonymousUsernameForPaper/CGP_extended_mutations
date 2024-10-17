use std::rc::Rc;
use crate::components::cgp_components::chromosome_evaluator_operators::{EvaluateChromosomeTrait, ChromosomeEvaluator};
use crate::components::cgp_components::chromosome_find_active_node_operators::ChromosomeActiveNodeTrait;
use crate::components::evo_operators_for_population::evaluation_operators::eval_population_trait::{GeneralForwardPassPopulationTrait, GeneralTestPopulationTrait};
use crate::function_set::function_trait::FunctionTrait;
use crate::utils::runner::Runner;

pub struct ForwardPassPopulationMuPlusLambda;
pub struct ForwardPassPopulationTournament;

// pub struct TestPopulationMuPlusLambda;
// pub struct TestPopulationTournament;

impl<T: Clone> GeneralForwardPassPopulationTrait<T> for ForwardPassPopulationMuPlusLambda where ChromosomeEvaluator: EvaluateChromosomeTrait<T> {
    fn new() -> Box<dyn GeneralForwardPassPopulationTrait<T>> where Self: Sized {
        Box::new(Self)
    }

    fn execute(&self, runner: &mut Runner<T>,
               evaluator_function: Rc<Box<dyn EvaluateChromosomeTrait<T>>>,
               active_node_func: Rc<Box<dyn ChromosomeActiveNodeTrait<T>>>,
               function_set: Rc<Vec<Box<dyn FunctionTrait<T>>>>) {

        // for id in eval_set {
        for id in &runner.child_ids {
            let fitness: f32 = evaluator_function.evaluate(&mut runner.population[*id],
                                                               Rc::clone(&active_node_func),
                                                               &runner.data,
                                                               &runner.label,
                                                               Rc::clone(&function_set));

            runner.fitness_vals[*id] = fitness;
        }

        runner.sort_fitness_vals();
    }

    fn execute_test_set(&self, runner: &mut Runner<T>, evaluator_function: Rc<Box<dyn EvaluateChromosomeTrait<T>>>, active_node_func: Rc<Box<dyn ChromosomeActiveNodeTrait<T>>>, function_set: Rc<Vec<Box<dyn FunctionTrait<T>>>>) -> f32 {
        let mut best_fitness = f32::MAX;
        // for id in eval_set {
        for id in &runner.child_ids {
            let fitness: f32 = evaluator_function.evaluate(&mut runner.population[*id],
                                                           Rc::clone(&active_node_func),
                                                           &runner.eval_data.as_ref().unwrap(),
                                                           &runner.eval_label.as_ref().unwrap(),
                                                           Rc::clone(&function_set));
            if fitness < best_fitness {
                best_fitness = fitness
            }

        }

        return best_fitness    }
}

impl<T: Clone> GeneralForwardPassPopulationTrait<T> for ForwardPassPopulationTournament where ChromosomeEvaluator: EvaluateChromosomeTrait<T> {
    fn new() -> Box<dyn GeneralForwardPassPopulationTrait<T>> where Self: Sized {
        ForwardPassPopulationMuPlusLambda::new()
    }

    fn execute(&self, _runner: &mut Runner<T>,
               _evaluator_function: Rc<Box<dyn EvaluateChromosomeTrait<T>>>,
               _active_node_func: Rc<Box<dyn ChromosomeActiveNodeTrait<T>>>,
               _function_set: Rc<Vec<Box<dyn FunctionTrait<T>>>>) {
        panic!("How did I get here? ")

    }

    fn execute_test_set(&self, runner: &mut Runner<T>, evaluator_function: Rc<Box<dyn EvaluateChromosomeTrait<T>>>, active_node_func: Rc<Box<dyn ChromosomeActiveNodeTrait<T>>>, function_set: Rc<Vec<Box<dyn FunctionTrait<T>>>>) -> f32 {
        panic!("How did I get here? ")
    }
}

// impl<T: Clone> GeneralTestPopulationTrait<T> for TestPopulationMuPlusLambda where ChromosomeEvaluator: EvaluateChromosomeTrait<T> {
//     fn new() -> Box<dyn GeneralTestPopulationTrait<T>> where Self: Sized {
//         Box::new(Self)
//     }
//
//     fn execute(&self, runner: &mut Runner<T>,
//                evaluator_function: Rc<Box<dyn EvaluateChromosomeTrait<T>>>,
//                active_node_func: Rc<Box<dyn ChromosomeActiveNodeTrait<T>>>,
//                function_set: Rc<Vec<Box<dyn FunctionTrait<T>>>>) -> f32 {
//
//         let mut best_fitness = f32::MAX;
//         // for id in eval_set {
//         for id in &runner.child_ids {
//             let fitness: f32 = evaluator_function.evaluate(&mut runner.population[*id],
//                                                            Rc::clone(&active_node_func),
//                                                            &runner.eval_data.as_ref().unwrap(),
//                                                            &runner.eval_label.as_ref().unwrap(),
//                                                            Rc::clone(&function_set));
//             if fitness < best_fitness {
//                 best_fitness = fitness
//             }
//
//         }
//
//         return best_fitness
//     }
// }

// impl<T: Clone> GeneralTestPopulationTrait<T> for TestPopulationTournament where ChromosomeEvaluator: EvaluateChromosomeTrait<T> {
//     fn new() -> Box<dyn GeneralTestPopulationTrait<T>> where Self: Sized {
//         TestPopulationMuPlusLambda::new()
//     }
//
//     fn execute(&self, _runner: &mut Runner<T>,
//                _evaluator_function: Rc<Box<dyn EvaluateChromosomeTrait<T>>>,
//                _active_node_func: Rc<Box<dyn ChromosomeActiveNodeTrait<T>>>,
//                _function_set: Rc<Vec<Box<dyn FunctionTrait<T>>>>) -> f32 {
//         panic!("How did I get here? ")
//
//     }
// }