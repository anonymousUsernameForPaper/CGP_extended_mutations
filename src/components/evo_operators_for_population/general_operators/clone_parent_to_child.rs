use rand::prelude::SliceRandom;
use crate::utils::runner::Runner;

pub trait ClonePopulationTrait<T> where T: Clone
{
    fn new() -> Box<dyn ClonePopulationTrait<T>> where Self: Sized;

    fn execute(&self, runner: &mut Runner<T>);
}

pub struct CloneParentToChild;

/// Must be used when no crossover is applied
/// Similar outcome can be achieved by setting `crossover_rate = 0`. But that's more efficient I think
/// And more readable
impl<T: Clone> ClonePopulationTrait<T> for CloneParentToChild {
    fn new() -> Box<dyn ClonePopulationTrait<T>> where Self: Sized {
        Box::new(Self)
    }


    fn execute(&self, runner: &mut Runner<T>) {
        for id in &runner.child_ids {
            // get the parent Id, so it can be cloned
            let parent_id: usize;
            if runner.params.elitists == 1 {
                // case: (1+4) ES; only one parent available to clone
                parent_id = runner.elitist_ids[0];
            } else {
                parent_id = *runner.elitist_ids.choose(&mut runner.rng).unwrap();
            }
            runner.population[*id] = runner.population[parent_id].clone();
        }
    }
}