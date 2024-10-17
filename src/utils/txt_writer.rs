use std::fs::File;
use std::path::Path;
use std::rc::Rc;
use crate::components::cgp_components::chromosome_find_active_node_operators::ChromosomeActiveNodeTrait;
use crate::function_set::function_trait::FunctionTrait;
use crate::utils::runner::{get_runner_parent, Runner};
use std::io::Write;

pub fn active_nodes_writer<T>(runner: &mut Runner<T>,
                              save_path: &Path,
                              run_id: usize,
                              active_node_func: Rc<Box<dyn ChromosomeActiveNodeTrait<T>>>,
                              function_set: Rc<Vec<Box<dyn FunctionTrait<T>>>>)
{
    let save_file_active_node = format!("run_{}_active_node.txt", run_id);
    let mut output = File::create(save_path.join(save_file_active_node))
        .expect("cannot create file");

    let mut parent = get_runner_parent(&runner);

    active_node_func.execute(&mut parent, Rc::clone(&function_set));

    write!(output, "{:?}", parent.active_nodes).expect("cannot write");
}