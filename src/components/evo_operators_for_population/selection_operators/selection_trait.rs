use crate::utils::runner::Runner;

pub trait GeneralSelectionTrait<T>
{
    fn new() -> Box<dyn GeneralSelectionTrait<T>> where Self: Sized;

    fn execute(&self, runner: &mut Runner<T>);
}