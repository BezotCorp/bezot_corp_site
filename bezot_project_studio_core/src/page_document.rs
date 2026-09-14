use std::collections::BTreeMap;

use serde::Serialize;

use common::{PageBlock, PageEditorState, PageLocaleEditor};

#[derive(Debug, Serialize)]
pub struct PageIndexDocument {
    pub locales: BTreeMap<String, PageLocaleIndexDocument>,
}

#[derive(Debug, Serialize)]
pub struct PageLocaleIndexDocument {
    pub status: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct PageLocaleContentDocument {
    pub slug: String,
    pub seo: PageSeoDocument,
    pub blocks: Vec<PageBlock>,
}

#[derive(Debug, Serialize)]
pub struct PageSeoDocument {
    pub title: String,
    pub description: String,
    pub robots: String,
    #[serde(rename = "ogTitle")]
    pub og_title: String,
    #[serde(rename = "ogDescription")]
    pub og_description: String,
    #[serde(rename = "ogImage")]
    pub og_image: String,
}

impl From<&PageEditorState> for PageIndexDocument {
    fn from(editor: &PageEditorState) -> Self {
        let mut locales = BTreeMap::new();
        locales.insert(
            "fr-fr".to_string(),
            PageLocaleIndexDocument::from(&editor.fr),
        );
        locales.insert(
            "en-us".to_string(),
            PageLocaleIndexDocument::from(&editor.en),
        );

        Self { locales }
    }
}

impl From<&PageLocaleEditor> for PageLocaleIndexDocument {
    fn from(editor: &PageLocaleEditor) -> Self {
        Self {
            status: editor.status.clone(),
            updated_at: editor.updated_at.clone(),
        }
    }
}

impl From<&PageLocaleEditor> for PageLocaleContentDocument {
    fn from(editor: &PageLocaleEditor) -> Self {
        Self {
            slug: editor.slug.clone(),
            seo: PageSeoDocument {
                title: editor.title.clone(),
                description: editor.description.clone(),
                robots: editor.robots.clone(),
                og_title: editor.og_title.clone(),
                og_description: editor.og_description.clone(),
                og_image: editor.og_image.clone(),
            },
            blocks: editor.blocks.clone(),
        }
    }
}
