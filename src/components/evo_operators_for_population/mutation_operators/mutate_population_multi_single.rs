// use std::cmp::{max, min};
// use std::rc::Rc;
// use num::Float;
// use crate::components::cgp_components::cgp_node_mutation_operators::NodeMutationOperatorTrait;
// use crate::components::cgp_components::chromosome_mutation_operators::ChromosomeMutationTrait;
// use crate::components::evo_operators_for_population::mutation_operators::mutation_trait::GeneralMutatePopulationTrait;
// use crate::utils::runner::Runner;
//
// pub struct EAMutateDecreasingMultiN {
//     number_single_active_mutations: f32,
//     last_fitness_val: f32,
// }
//
// impl<T: Clone> GeneralMutatePopulationTrait<T> for EAMutateDecreasingMultiN {
//     fn new() -> Box<dyn GeneralMutatePopulationTrait<T>>
//     where
//         Self: Sized,
//     {
//         Box::new(Self {
//             number_single_active_mutations: 1.0,
//             last_fitness_val: f32::max_value(),
//         })
//     }
//
//     fn execute(&mut self,
//                runner: &mut Runner<T>,
//                node_mutation_op: Rc<Box<dyn NodeMutationOperatorTrait>>,
//                chromosome_mutation_op: Rc<Box<dyn ChromosomeMutationTrait>>) {
//         const CONST_VAL_F: f32 = 1.5;
//         const CONST_VAL_MAX_N: f32 = 5.0;
//
//         let best_fitness_val = runner.fitness_vals_sorted[0];
//
//         // update the correct number of single active mutations
//         if best_fitness_val < self.last_fitness_val {
//             let mut new_n = self.number_single_active_mutations;
//             new_n = (new_n / CONST_VAL_F).max(1.0);
//
//             self.number_single_active_mutations = new_n;
//         } else {
//             let mut new_n = self.number_single_active_mutations;
//             new_n = (new_n * CONST_VAL_F.powf(0.25)).min(CONST_VAL_MAX_N);
//
//             self.number_single_active_mutations = new_n;
//         }
//         self.last_fitness_val = best_fitness_val;
//
//         for id in &runner.child_ids {
//             for _ in 0..(self.number_single_active_mutations.round() as usize) {
//                 chromosome_mutation_op.execute(&mut runner.population[*id],
//                                                Rc::clone(&node_mutation_op));
//             }
//         }
//     }
// }