use std::io;
use std::path::Path;

use crate::assembler_state::AssemblerState;
use crate::block_definition::load_block_dataset;
use crate::content_reader::ContentModel;
use crate::content_validator::validate_content_model;
use crate::prebuild_writer::{PrebuildMaterialization, materialize_prebuild_sources};
use crate::project_config::ProjectConfig;
use crate::project_snapshot::ProjectSnapshot;
use crate::site_generator::GeneratedFile;
use crate::state_diff::StateDiff;
use crate::state_store::{load_state, store_state};

#[derive(Debug)]
pub struct AssemblyReport {
    state: AssemblerState,
    block_definitions: usize,
    content_pages: usize,
    content_posts: usize,
    diff: StateDiff,
    materialization: PrebuildMaterialization,
    state_written: bool,
}

impl AssemblyReport {
    pub fn project_root(&self) -> &Path {
        &self.state.project_root
    }

    pub fn print_summary(&self) {
        println!("project_root: {}", self.state.project_root.display());
        println!("inputs: {}", self.state.inputs.len());
        println!("block_definitions: {}", self.block_definitions);
        println!("content_pages: {}", self.content_pages);
        println!("content_posts: {}", self.content_posts);
        println!("added_inputs: {}", self.diff.added_inputs.len());
        println!("changed_inputs: {}", self.diff.changed_inputs.len());
        println!("removed_inputs: {}", self.diff.removed_inputs.len());
        println!("outputs_changed: {}", self.diff.outputs_changed);
        println!("prebuild_written: {}", self.materialization.written);
        println!("prebuild_unchanged: {}", self.materialization.unchanged);
        println!("prebuild_deleted: {}", self.materialization.deleted);
        println!("state_written: {}", self.state_written);
    }

    pub fn assemble_project(project_root: &Path) -> io::Result<AssemblyReport> {
        let config = ProjectConfig::default_site_project();
        let block_dataset = load_block_dataset()?;
        let snapshot = ProjectSnapshot::scan_project(project_root, &config)?;
        validate_source_boundaries(&snapshot)?;
        let content_model = ContentModel::read_content_model(&snapshot.project_root)?;

        validate_content_model(&content_model, &block_dataset)?;

        let generated_files = GeneratedFile::generate_site_files(&content_model, &block_dataset)?;
        let new_state = AssemblerState::from_snapshot(&snapshot, &generated_files);
        let old_state = load_state(&new_state.project_root)?;
        let diff = StateDiff::diff_states(old_state.as_ref(), &new_state);
        let materialization =
            materialize_prebuild_sources(old_state.as_ref(), &new_state, &generated_files)?;
        let state_written = if diff.has_changes() || materialization.has_changes() {
            store_state(&new_state)?;
            true
        } else {
            false
        };

        Ok(Self {
            state: new_state,
            block_definitions: block_dataset.blocks.len(),
            content_pages: content_model.pages.len(),
            content_posts: content_model.posts.len(),
            diff,
            materialization,
            state_written,
        })
    }
}

fn validate_source_boundaries(snapshot: &ProjectSnapshot) -> io::Result<()> {
    const FORBIDDEN_SOURCE_PREFIXES: [&str; 3] =
        ["src/generated/", "src/assembled/", "src/composition/"];

    for file in &snapshot.files {
        if FORBIDDEN_SOURCE_PREFIXES
            .iter()
            .any(|prefix| file.relative_path.starts_with(prefix))
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "{} is forbidden: generated or composition code must live in the assembler-owned prebuild, not in site/src",
                    file.relative_path
                ),
            ));
        }
    }

    Ok(())
}
