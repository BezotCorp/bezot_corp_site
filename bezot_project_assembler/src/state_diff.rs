use std::collections::BTreeMap;

use crate::assembler_state::{AssemblerState, InputFile, OutputFile};

#[derive(Debug)]
pub struct StateDiff {
    pub added_inputs: Vec<InputFile>,
    pub changed_inputs: Vec<InputFile>,
    pub removed_inputs: Vec<InputFile>,
    pub outputs_changed: bool,
}

impl StateDiff {
    pub fn has_changes(&self) -> bool {
        !self.added_inputs.is_empty()
            || !self.changed_inputs.is_empty()
            || !self.removed_inputs.is_empty()
            || self.outputs_changed
    }
}

pub fn diff_states(old_state: Option<&AssemblerState>, new_state: &AssemblerState) -> StateDiff {
    let Some(old_state) = old_state else {
        return StateDiff {
            added_inputs: new_state.inputs.clone(),
            changed_inputs: Vec::new(),
            removed_inputs: Vec::new(),
            outputs_changed: !new_state.outputs.is_empty(),
        };
    };

    let old_inputs = inputs_by_path(&old_state.inputs);
    let new_inputs = inputs_by_path(&new_state.inputs);

    let mut added_inputs = Vec::new();
    let mut changed_inputs = Vec::new();
    let mut removed_inputs = Vec::new();

    for (path, new_file) in &new_inputs {
        match old_inputs.get(path) {
            None => added_inputs.push((*new_file).clone()),
            Some(old_file) if *old_file != *new_file => {
                changed_inputs.push((*new_file).clone());
            }
            Some(_) => {}
        }
    }

    for (path, old_file) in &old_inputs {
        if !new_inputs.contains_key(path) {
            removed_inputs.push((*old_file).clone());
        }
    }

    StateDiff {
        added_inputs,
        changed_inputs,
        removed_inputs,
        outputs_changed: outputs_by_path(&old_state.outputs) != outputs_by_path(&new_state.outputs),
    }
}

fn inputs_by_path(inputs: &[InputFile]) -> BTreeMap<String, &InputFile> {
    inputs
        .iter()
        .map(|input| (input.path.clone(), input))
        .collect()
}

fn outputs_by_path(outputs: &[OutputFile]) -> BTreeMap<String, &OutputFile> {
    outputs
        .iter()
        .map(|output| (output.path.clone(), output))
        .collect()
}
