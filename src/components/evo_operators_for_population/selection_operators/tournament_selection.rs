// use rand::prelude::IteratorRandom;
// use crate::components::evo_operators_for_population::selection_operators::selection_trait::GeneralSelectionTrait;
// use crate::utils::runner::Runner;
//
// pub struct TournamentSelection;
//
// impl<T: Clone> GeneralSelectionTrait<T> for TournamentSelection {
//     fn new() -> Box<dyn GeneralSelectionTrait<T>> where Self: Sized {
//         Box::new(Self)
//     }
//
//     fn execute(&self, runner: &mut Runner<T>) {
//         let mut selection = vec![];
//
//         for _ in 0..runner.params.population_size {
//             let winner_id = runner.fitness_vals
//                 .clone()
//                 .into_iter()
//                 .enumerate() // get tuples: (i, fitness_val) with i := chromosome id
//                 .choose_multiple(&mut runner.rng, runner.params.tournament_size)
//                 .into_iter()
//                 .min_by(|i, j| i.1.partial_cmp(&j.1).unwrap())  // Sort by fitness val
//                 .map(|(i, _)| i)// get id of chromosome
//                 .unwrap();
//
//             selection.push(winner_id)
//         }
//
//         runner.tournament_selected = Some(selection);
//     }
// }