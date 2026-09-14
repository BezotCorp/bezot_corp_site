use crate::{assembler_state::AssemblerState, input_file::InputFile, output_file::OutputFile};

#[derive(Debug)]
pub(crate) struct StateDiff {
    pub(crate) added_inputs: Vec<InputFile>,
    pub(crate) changed_inputs: Vec<InputFile>,
    pub(crate) removed_inputs: Vec<InputFile>,
    pub(crate) outputs_changed: bool,
}

impl StateDiff {
    pub(crate) fn has_changes(&self) -> bool {
        !self.added_inputs.is_empty()
            || !self.changed_inputs.is_empty()
            || !self.removed_inputs.is_empty()
            || self.outputs_changed
    }

    pub(crate) fn diff_states(
        old_state: Option<&AssemblerState>,
        new_state: &AssemblerState,
    ) -> Self {
        let Some(old_state) = old_state else {
            return Self {
                added_inputs: new_state.inputs.clone(),
                changed_inputs: Vec::new(),
                removed_inputs: Vec::new(),
                outputs_changed: !new_state.outputs.is_empty(),
            };
        };

        let old_inputs = InputFile::inputs_by_path(&old_state.inputs);
        let new_inputs = InputFile::inputs_by_path(&new_state.inputs);

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

        Self {
            added_inputs,
            changed_inputs,
            removed_inputs,
            outputs_changed: OutputFile::outputs_by_path(&old_state.outputs)
                != OutputFile::outputs_by_path(&new_state.outputs),
        }
    }
}
