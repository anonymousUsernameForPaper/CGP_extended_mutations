#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use std::fs;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;
use clap::Parser;
use std::io::{BufWriter, Write};
use cgp_master::function_set::boolean_function_set;
use cgp_master::function_set::regression_function_set;
use cgp_master::components::cgp_components::cgp_node::CGPNode;
use cgp_master::components::cgp_components::cgp_node_mutation_operators::*;
use cgp_master::global_params::CgpParameters;
use cgp_master::components::cgp_components::cgp_node_types::NodeType;
use cgp_master::components::cgp_components::cgp_types::CGPType;
use cgp_master::components::cgp_components::chromosome_evaluator_operators::*;
use cgp_master::components::cgp_components::chromosome_find_active_node_operators::*;
use cgp_master::components::cgp_components::chromosome_mutation_operators::*;
use cgp_master::components::cgp_components::chromosome_reorder_operators::*;
use cgp_master::components::evo_operators_for_population::crossover_operators::crossover_mulambda_elitist::CrossoverMuLambdaElitist;
use cgp_master::components::evo_operators_for_population::crossover_operators::crossover_tournament::CrossoverTournament;
use cgp_master::components::evo_operators_for_population::crossover_operators::crossover_trait::GeneralCrossoverTrait;
use cgp_master::components::evo_operators_for_population::crossover_operators::crossover_types::CrossoverType;
use cgp_master::components::evo_operators_for_population::evaluation_operators::eval_population_mupluslambda::{ForwardPassPopulationMuPlusLambda, ForwardPassPopulationTournament};
use cgp_master::components::evo_operators_for_population::evaluation_operators::eval_population_oneplusfour::{EAForwardPassPopulationOnePlusFour};
use cgp_master::components::evo_operators_for_population::evaluation_operators::eval_population_trait::{GeneralForwardPassPopulationTrait};
use cgp_master::components::evo_operators_for_population::general_operators::clone_parent_to_child::{CloneParentToChild, ClonePopulationTrait};
use cgp_master::components::evo_operators_for_population::general_operators::reorder_population::{GeneralReorderPopulationTrait, ReorderPopulation};
use cgp_master::components::evo_operators_for_population::mutation_operators::mutate_population::EAMutateStandard;
use cgp_master::components::evo_operators_for_population::selection_operators::elitist_selection_oneplusfour::EAElitistSelectionOnePlusFour;
use cgp_master::components::evo_operators_for_population::mutation_operators::mutation_trait::GeneralMutatePopulationTrait;
use cgp_master::components::evo_operators_for_population::selection_operators::elitist_selection_mupluslambda::ElitistSelectionMuPlusLambda;
use cgp_master::components::evo_operators_for_population::selection_operators::elitist_selection_tournament::ElitistSelectionWithTournament;
use cgp_master::components::evo_operators_for_population::selection_operators::selection_trait::GeneralSelectionTrait;

use cgp_master::utils::runner::Runner;
use cgp_master::utils::utility_funcs;
use cgp_master::datasets::boolean_datasets;
use cgp_master::datasets::regression_benchmarks;

use cgp_master::utils::txt_writer::*;
use cgp_master::function_set::function_trait::FunctionTrait;

#[derive(Parser)]
#[clap(author, version, about, name = "testname")]
struct Args {
    #[arg(long, default_value_t = 1)]
    run_id: usize,

    #[arg(long, default_value_t = 0)]
    dataset: usize,

    #[arg(long, default_value_t = 500)]
    nbr_nodes: usize,

    #[arg(long, default_value = "f32")]
    dataset_type: String,

    #[arg(long, default_value_t = 3)]
    mutation_multi_n: usize,

    #[arg(long, default_value_t = 0.0)]
    split_mutation_rate_active: f32,

    #[arg(long, default_value_t = 0.0)]
    split_mutation_rate_inactive: f32,

    #[arg(long, default_value_t = 0.0)]
    mutation_rate: f32,

    // allowed values:
    // - single
    // - point
    // - multi
    // - complexpoint
    #[arg(long, default_value = "single")]
    bioma_mutation_type: String,
}


fn bioma_bool(args: Args) {
    let (data, label) = match args.dataset {
        0 => boolean_datasets::parity::get_dataset(),
        1 => boolean_datasets::encode::get_dataset(),
        2 => boolean_datasets::decode::get_dataset(),
        3 => boolean_datasets::multiply::get_dataset(),
        _ => { panic!("Wrong Dataset Number") }
    };
    let function_set = boolean_function_set::get_boolean_function_set();

    let fitness_threshold = 0.0001;  // Bool

    let mut params = CgpParameters {
        cgp_type: CGPType::Standard,
        graph_width: args.nbr_nodes,
        elitists: 1,
        population_size: 4,  // can also be lambda
        eval_after_iterations: 0,
        nbr_inputs: data[0].len(),
        nbr_outputs: label[0].len(),
        mutation_rate: args.mutation_rate,
        crossover_type: CrossoverType::NoCrossover,
        crossover_rate: 0.0,
        multi_point_n: 0,
        tournament_size: 0,
        number_functions: function_set.len(),
        fitness_threshold,
        BIOMA_nbr_mutations: args.mutation_multi_n,
        BIOMA_prob_active_mutation: args.split_mutation_rate_active,
        BIOMA_prob_inactive_mutation: args.split_mutation_rate_inactive,
    };

    let node_mutation_op = Rc::new(NodeMutationStandard::new());
    let chromosome_active_op = Rc::new(ChromosomeFindActiveNodesStandard::new());

    let chromosome_mutation_op: Rc<Box<dyn ChromosomeMutationTrait>>;
    if args.bioma_mutation_type == "single" {
        chromosome_mutation_op = Rc::new(ChromosomeMutationSingle::new());
    } else if args.bioma_mutation_type == "point" {
        chromosome_mutation_op = Rc::new(ChromosomeMutationPoint::new());
    } else if args.bioma_mutation_type == "multi" {
        chromosome_mutation_op = Rc::new(ChromosomeMutationMultiN::new());
    } else {
        chromosome_mutation_op = Rc::new(ChromosomeMutationSplit::new());
    }

    let chromosome_eval_op = Rc::new(ChromosomeEvaluator::new());

    let clone_parent2child = CloneParentToChild::new();
    // let mutation_operator = EAMutateStandard::new();
    let  mut mutation_operator = EAMutateStandard::new();
    let eval_operator = EAForwardPassPopulationOnePlusFour::new();
    let selection_operator = EAElitistSelectionOnePlusFour::new();

    let mut runner = Runner::new(params, data, label, None, None, Rc::clone(&function_set), Rc::clone(&chromosome_active_op));

    let save_path = Path::new("")
        .join("Experiments_Output_boolean")
        .join(format!("dataset_{}", args.dataset))
        .join(format!("{}", args.bioma_mutation_type))
        .join(format!("number_nodes_{}_prob_active_{}_inactive_{}_multi_{}_point_{}", args.nbr_nodes, args.split_mutation_rate_active, args.split_mutation_rate_inactive, args.mutation_multi_n, args.mutation_rate));

    fs::create_dir_all(save_path.clone()).unwrap();
    let save_file_iteration = format!("run_{}_iteration.txt", args.run_id);
    let mut output_file = File::create(save_path.join(save_file_iteration))
        .expect("cannot create file");

    let save_file_iteration = format!("mutated_nodes_{}.txt", args.run_id);
    let mut mutationfile = BufWriter::new(File::create(save_path.join(save_file_iteration))
        .expect("cannot create file"));

    let mut iteration_number = 0;
    for i in 0..500_000 {
        writeln!(output_file, "Iteration: {iteration_number}, Fitness: {:?}", runner.get_best_fitness()).expect("write not okay??");
        if i % 500 == 0 {
            println!("i: {}, fitness: {}", i, runner.get_best_fitness());
        }
        iteration_number += 1;

        clone_parent2child.execute(&mut runner);
        mutation_operator.execute(&mut runner, Rc::clone(&node_mutation_op), Rc::clone(&chromosome_mutation_op), &mut mutationfile);
        eval_operator.execute(&mut runner, Rc::clone(&chromosome_eval_op), Rc::clone(&chromosome_active_op), Rc::clone(&function_set));
        selection_operator.execute(&mut runner);

        if runner.get_best_fitness() < fitness_threshold {
            break;
        }
    }
    println!("{}", iteration_number);

    write!(output_file, "End at iteration: {}", iteration_number).expect("cannot write");
    active_nodes_writer(&mut runner, &save_path, args.run_id, Rc::clone(&chromosome_active_op), Rc::clone(&function_set));
}

fn bioma_f32(args: Args) {
    let (data, label) = match args.dataset {
        0 => regression_benchmarks::keijzer::get_dataset(),
        1 => regression_benchmarks::koza_3::get_dataset(),
        2 => regression_benchmarks::nguyen_7::get_dataset(),
        3 => regression_benchmarks::pagie_1::get_dataset(),
        _ => { panic!("Wrong Dataset Number") }
    };
    let (eval_data, eval_label) = match args.dataset {
        0 => regression_benchmarks::keijzer::get_eval_dataset(),
        1 => regression_benchmarks::koza_3::get_eval_dataset(),
        2 => regression_benchmarks::nguyen_7::get_eval_dataset(),
        3 => regression_benchmarks::pagie_1::get_eval_dataset(),
        _ => { panic!("Wrong Dataset Number") }
    };

    let function_set = regression_function_set::get_regression_function_set();

    let fitness_threshold = 0.01;  // Regression

    let mut params = CgpParameters {
        cgp_type: CGPType::Standard,
        graph_width: args.nbr_nodes,
        elitists: 1,
        population_size: 4,  // can also be lambda
        eval_after_iterations: 0,
        nbr_inputs: data[0].len(),
        nbr_outputs: label[0].len(),
        mutation_rate: args.mutation_rate,
        crossover_type: CrossoverType::NoCrossover,
        crossover_rate: 0.0,
        multi_point_n: 0,
        tournament_size: 0,
        number_functions: function_set.len(),
        fitness_threshold,
        BIOMA_nbr_mutations: args.mutation_multi_n,
        BIOMA_prob_active_mutation: args.split_mutation_rate_active,
        BIOMA_prob_inactive_mutation: args.split_mutation_rate_inactive,
    };

    let node_mutation_op = Rc::new(NodeMutationStandard::new());
    let chromosome_active_op = Rc::new(ChromosomeFindActiveNodesStandard::new());

    let chromosome_mutation_op: Rc<Box<dyn ChromosomeMutationTrait>>;
    if args.bioma_mutation_type == "single" {
        chromosome_mutation_op = Rc::new(ChromosomeMutationSingle::new());
    } else if args.bioma_mutation_type == "point" {
        chromosome_mutation_op = Rc::new(ChromosomeMutationPoint::new());
    } else if args.bioma_mutation_type == "multi" {
        chromosome_mutation_op = Rc::new(ChromosomeMutationMultiN::new());
    } else {
        chromosome_mutation_op = Rc::new(ChromosomeMutationSplit::new());
    }

    let chromosome_eval_op = Rc::new(ChromosomeEvaluator::new());

    let clone_parent2child = CloneParentToChild::new();
    let mut mutation_operator = EAMutateStandard::new();

    let eval_operator = EAForwardPassPopulationOnePlusFour::new();
    let selection_operator = EAElitistSelectionOnePlusFour::new();

    let mut runner = Runner::new(params, data, label, Some(eval_data), Some(eval_label), Rc::clone(&function_set), Rc::clone(&chromosome_active_op));

    let save_path = Path::new("")
        .join("Experiments_Output_regression")
        .join(format!("dataset_{}", args.dataset))
        .join(format!("{}", args.bioma_mutation_type))
        .join(format!("number_nodes_{}_prob_active_{}_inactive_{}_multi_{}_point_{}", args.nbr_nodes, args.split_mutation_rate_active, args.split_mutation_rate_inactive, args.mutation_multi_n, args.mutation_rate));

    fs::create_dir_all(save_path.clone()).unwrap();
    let save_file_iteration = format!("run_{}_iteration.txt", args.run_id);
    let mut output_file = File::create(save_path.join(save_file_iteration))
        .expect("cannot create file");

    let save_file_iteration = format!("mutated_nodes_{}.txt", args.run_id);
    let mut mutationfile = BufWriter::new(File::create(save_path.join(save_file_iteration))
        .expect("cannot create file"));

    let mut iteration_number = 0;

    for _ in 0..500_000 {
        writeln!(output_file, "Iteration: {iteration_number}, Fitness: {:?}", runner.get_best_fitness()).expect("write not okay??");

        if iteration_number % 500 == 0 {
            println!("i: {}, fitness: {}", iteration_number, runner.get_best_fitness());
        }

        iteration_number += 1;

        clone_parent2child.execute(&mut runner);
        mutation_operator.execute(&mut runner, Rc::clone(&node_mutation_op), Rc::clone(&chromosome_mutation_op), &mut mutationfile);
        eval_operator.execute(&mut runner, Rc::clone(&chromosome_eval_op), Rc::clone(&chromosome_active_op), Rc::clone(&function_set));
        selection_operator.execute(&mut runner);

        if runner.get_best_fitness() < fitness_threshold {
            break;
        }
    }

    let fitness_train = eval_operator.execute_test_set(&mut runner, Rc::clone(&chromosome_eval_op), Rc::clone(&chromosome_active_op), Rc::clone(&function_set));

    println!("{}", runner.get_best_fitness());
    writeln!(output_file, "End at iteration: {}", iteration_number).expect("cannot write");
    writeln!(output_file, "Fitness Eval: {}", runner.get_best_fitness()).expect("cannot write");
    writeln!(output_file, "Fitness Train: {}", fitness_train).expect("cannot write");

    active_nodes_writer(&mut runner, &save_path, args.run_id, Rc::clone(&chromosome_active_op), Rc::clone(&function_set));
}

fn main() {
    let mut args = Args::parse();

    if args.dataset_type == "bool" {
        bioma_bool(args);
    } else {
        bioma_f32(args);
    }
}