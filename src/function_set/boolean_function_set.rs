use std::rc::Rc;
use crate::function_set::function_trait::FunctionTrait;


pub fn get_boolean_function_set() -> Rc<Vec<Box<dyn FunctionTrait<bool>>>> {
    let mut function_set: Vec<Box<dyn FunctionTrait<bool>>> = Vec::new();

    function_set.push(Box::new(BoolAND));
    function_set.push(Box::new(BoolOR));
    function_set.push(Box::new(BoolNAND));
    function_set.push(Box::new(BoolNOR));

    return Rc::new(function_set);
}


pub struct BoolAND;

pub struct BoolOR;

pub struct BoolNAND;

pub struct BoolNOR;

impl FunctionTrait<bool> for BoolAND {
    fn get_number_inputs_needed(&self) -> usize {
        return 2;
    }

    fn execute_function(&self, inputs: &[&Vec<bool>]) -> Vec<bool> {
        let input0 = inputs[0];
        let input1 = inputs[1];
        return input0
            .iter()
            .zip(input1.iter())
            .map(|(a, b)| *a & *b)
            .collect();
    }
}

impl FunctionTrait<bool> for BoolOR {
    fn get_number_inputs_needed(&self) -> usize {
        return 2;
    }

    fn execute_function(&self, inputs: &[&Vec<bool>]) -> Vec<bool> {
        let input0 = inputs[0];
        let input1 = inputs[1];
        return input0
            .iter()
            .zip(input1.iter())
            .map(|(a, b)| *a | *b)
            .collect();
    }
}

impl FunctionTrait<bool> for BoolNAND {

    fn get_number_inputs_needed(&self) -> usize {
        return 2;
    }

    fn execute_function(&self, inputs: &[&Vec<bool>]) -> Vec<bool> {
        let input0 = inputs[0];
        let input1 = inputs[1];
        return input0
            .iter()
            .zip(input1.iter())
            .map(|(a, b)| !(*a & *b))
            .collect();
    }
}

impl FunctionTrait<bool> for BoolNOR {

    fn get_number_inputs_needed(&self) -> usize {
        return 2;
    }

    fn execute_function(&self, inputs: &[&Vec<bool>]) -> Vec<bool> {
        let input0 = inputs[0];
        let input1 = inputs[1];
        return input0
            .iter()
            .zip(input1.iter())
            .map(|(a, b)| !(*a | *b))
            .collect();
    }
}


