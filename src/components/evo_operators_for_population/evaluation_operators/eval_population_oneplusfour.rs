use std::rc::Rc;
use crate::components::cgp_components::chromosome_evaluator_operators::{EvaluateChromosomeTrait, ChromosomeEvaluator};
use crate::components::cgp_components::chromosome_find_active_node_operators::ChromosomeActiveNodeTrait;
use crate::components::evo_operators_for_population::evaluation_operators::eval_population_trait::{GeneralForwardPassPopulationTrait, GeneralTestPopulationTrait};
use crate::function_set::function_trait::FunctionTrait;
use crate::utils::runner::Runner;

pub struct EAForwardPassPopulationOnePlusFour;
// pub struct EATestPopulationOnePlusFour;


impl<T: Clone> GeneralForwardPassPopulationTrait<T> for EAForwardPassPopulationOnePlusFour
where ChromosomeEvaluator: EvaluateChromosomeTrait<T> {
    fn new() -> Box<dyn GeneralForwardPassPopulationTrait<T>> where Self: Sized {
        Box::new(Self)
    }

    fn execute(&self, runner: &mut Runner<T>,
               evaluator_function: Rc<Box<dyn EvaluateChromosomeTrait<T>>>,
               active_node_func: Rc<Box<dyn ChromosomeActiveNodeTrait<T>>>,
               function_set: Rc<Vec<Box<dyn FunctionTrait<T>>>>) {
        // because of (1+4), there should only be one elitist
        assert_eq!(1, runner.elitist_ids.len());

        let parent_id = runner.elitist_ids[0];
        for id in 0..(runner.params.elitists + runner.params.population_size) {
            if id != parent_id {
                let fitness: f32 = evaluator_function.evaluate(&mut runner.population[id],
                                                                   Rc::clone(&active_node_func),
                                                                   &runner.data,
                                                                   &runner.label,
                                                                   Rc::clone(&function_set));
                runner.fitness_vals[id] = fitness;
            }
        }
        runner.sort_fitness_vals();
    }

    fn execute_test_set(&self, runner: &mut Runner<T>, evaluator_function: Rc<Box<dyn EvaluateChromosomeTrait<T>>>, active_node_func: Rc<Box<dyn ChromosomeActiveNodeTrait<T>>>, function_set: Rc<Vec<Box<dyn FunctionTrait<T>>>>) -> f32 {
        // because of (1+4), there should only be one elitist
        assert_eq!(1, runner.elitist_ids.len());
        let mut best_fitness = f32::MAX;
        assert!(runner.eval_data.is_some());

        for id in 0..(runner.params.elitists + runner.params.population_size) {
            let fitness: f32 = evaluator_function.evaluate(&mut runner.population[id],
                                                           Rc::clone(&active_node_func),
                                                           &runner.eval_data.as_ref().unwrap(),
                                                           &runner.eval_label.as_ref().unwrap(),
                                                           Rc::clone(&function_set));
            if fitness < best_fitness {
                best_fitness = fitness;
            }
        }

        return best_fitness;    }
}

// impl<T: Clone> GeneralTestPopulationTrait<T> for EATestPopulationOnePlusFour
// where ChromosomeEvaluator: EvaluateChromosomeTrait<T> {
//     fn new() -> Box<dyn GeneralTestPopulationTrait<T>> where Self: Sized {
//         Box::new(Self)
//     }
//
//     fn execute(&self, runner: &mut Runner<T>,
//                evaluator_function: Rc<Box<dyn EvaluateChromosomeTrait<T>>>,
//                active_node_func: Rc<Box<dyn ChromosomeActiveNodeTrait<T>>>,
//                function_set: Rc<Vec<Box<dyn FunctionTrait<T>>>>) -> f32{
//         // because of (1+4), there should only be one elitist
//         assert_eq!(1, runner.elitist_ids.len());
//         let mut best_fitness = f32::MAX;
//         assert!(runner.eval_data.is_some());
//
//         for id in 0..(runner.params.elitists + runner.params.population_size) {
//             let fitness: f32 = evaluator_function.evaluate(&mut runner.population[id],
//                                                            Rc::clone(&active_node_func),
//                                                            &runner.eval_data.as_ref().unwrap(),
//                                                            &runner.eval_label.as_ref().unwrap(),
//                                                            Rc::clone(&function_set));
//             if fitness < best_fitness {
//                 best_fitness = fitness;
//             }
//         }
//
//         return best_fitness;
//     }
// }

