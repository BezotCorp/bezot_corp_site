use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::PathBuf;

use serde::Deserialize;

use crate::asserts::assert_known_field;
use crate::content_validator::invalid_data_message;

#[derive(Debug, Deserialize)]
pub struct BlockDataset {
    pub version: u32,
    pub blocks: BTreeMap<String, BlockDefinition>,
}

#[derive(Debug, Deserialize)]
pub struct BlockDefinition {
    pub source: BlockSource,
    pub component: BlockComponent,
    pub rules: BlockRules,
    pub knowledge: BlockKnowledge,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum BlockSource {
    Content,
    Runtime,
}

#[derive(Debug, Deserialize)]
pub struct BlockComponent {
    pub module: String,
    pub export: String,
}

#[derive(Debug, Deserialize)]
pub struct BlockRules {
    pub placement: PlacementRule,
    pub cardinality: CardinalityRule,
    pub fields: BTreeMap<String, FieldRule>,
    pub output: OutputNode,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum PlacementRule {
    PageStart,
    Anywhere,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum CardinalityRule {
    AtMostOne,
    Many,
}

#[derive(Debug, Deserialize)]
pub struct FieldRule {
    pub value_type: FieldValueType,
    pub required: bool,
}

#[derive(Debug, Deserialize)]
pub enum FieldValueType {
    String,
    PositiveInteger,
    Boolean,
    CardList,
}

#[derive(Debug, Deserialize)]
pub enum OutputNode {
    Element {
        tag: String,
        children: Vec<OutputNode>,
    },
    Field {
        name: String,
    },
    Conditional {
        field: String,
        output: Box<OutputNode>,
    },
    Text {
        value: String,
    },
}

#[derive(Debug, Deserialize)]
pub struct BlockKnowledge {
    pub purpose: String,
    pub usage: String,
    pub good_examples: Vec<BTreeMap<String, String>>,
    pub bad_uses: Vec<String>,
}

pub fn load_block_dataset() -> io::Result<BlockDataset> {
    let path = dataset_path();
    let source = fs::read_to_string(&path)?;

    let dataset: BlockDataset = ron::from_str(&source).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Invalid block dataset {}: {error}", path.display()),
        )
    })?;

    validate_dataset(&dataset)?;

    Ok(dataset)
}

fn dataset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("datasets")
        .join("blocks.ron")
}

fn validate_dataset(dataset: &BlockDataset) -> io::Result<()> {
    if dataset.version == 0 {
        return Err(invalid_data_message(
            "Block dataset version must be greater than zero",
        ));
    }

    if dataset.blocks.is_empty() {
        return Err(invalid_data_message(
            "Block dataset must define at least one block",
        ));
    }

    for (block_name, definition) in &dataset.blocks {
        if block_name.is_empty() {
            return Err(invalid_data_message("Block name must not be empty"));
        }

        if definition.knowledge.purpose.trim().is_empty() {
            return Err(invalid_data_message(format!(
                "Block \"{block_name}\" knowledge purpose must not be empty"
            )));
        }

        if definition.knowledge.usage.trim().is_empty() {
            return Err(invalid_data_message(format!(
                "Block \"{block_name}\" knowledge usage must not be empty"
            )));
        }

        if definition.knowledge.good_examples.is_empty() {
            return Err(invalid_data_message(format!(
                "Block \"{block_name}\" knowledge must contain at least one good example"
            )));
        }

        if definition.knowledge.bad_uses.is_empty() {
            return Err(invalid_data_message(format!(
                "Block \"{block_name}\" knowledge must contain at least one bad use"
            )));
        }

        validate_component(block_name, &definition.component)?;

        validate_output_node(
            block_name,
            &definition.rules.fields,
            &definition.rules.output,
        )?;
    }

    Ok(())
}

fn validate_component(block_name: &str, component: &BlockComponent) -> io::Result<()> {
    if component.module.is_empty()
        || component.module.starts_with('/')
        || component
            .module
            .split('/')
            .any(|segment| segment.is_empty() || segment == "..")
    {
        return Err(invalid_data_message(format!(
            "Block \"{block_name}\" has an invalid component module \"{}\"",
            component.module
        )));
    }

    let mut characters = component.export.chars();
    let valid_export = characters
        .next()
        .is_some_and(|character| character == '_' || character.is_ascii_alphabetic())
        && characters.all(|character| character == '_' || character.is_ascii_alphanumeric());

    if !valid_export {
        return Err(invalid_data_message(format!(
            "Block \"{block_name}\" has an invalid component export \"{}\"",
            component.export
        )));
    }

    Ok(())
}

fn validate_output_node(
    block_name: &str,
    fields: &BTreeMap<String, FieldRule>,
    node: &OutputNode,
) -> io::Result<()> {
    match node {
        OutputNode::Element { tag, children } => {
            if tag.trim().is_empty() {
                return Err(invalid_data_message(format!(
                    "Block \"{block_name}\" contains an element with an empty tag"
                )));
            }

            for child in children {
                validate_output_node(block_name, fields, child)?;
            }
        }

        OutputNode::Field { name } => {
            assert_known_field(block_name, fields, name)?;
        }

        OutputNode::Conditional { field, output } => {
            assert_known_field(block_name, fields, field)?;
            validate_output_node(block_name, fields, output)?;
        }

        OutputNode::Text { value } => {
            if value.trim().is_empty() {
                return Err(invalid_data_message(format!(
                    "Block \"{block_name}\" contains an empty output text"
                )));
            }
        }
    }

    Ok(())
}
