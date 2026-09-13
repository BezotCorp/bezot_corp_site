use std::collections::BTreeMap;

use serde::Serialize;

use crate::post_editor_state::PostEditorState;

#[derive(Debug, Serialize)]
pub(crate) struct PostDocument {
    pub(crate) id: String,
    #[serde(rename = "type")]
    pub(crate) kind: String,
    pub(crate) status: String,
    pub(crate) author: String,
    #[serde(rename = "publishedAt")]
    pub(crate) published_at: String,
    #[serde(rename = "updatedAt")]
    pub(crate) updated_at: String,
    pub(crate) locales: BTreeMap<String, PostLocaleDocument>,
}

#[derive(Debug, Serialize)]
pub(crate) struct PostLocaleDocument {
    pub(crate) slug: String,
    pub(crate) seo: PostSeoDocument,
    pub(crate) blocks: Vec<PostBlockDocument>,
}

#[derive(Debug, Serialize)]
pub(crate) struct PostSeoDocument {
    pub(crate) title: String,
    pub(crate) description: String,
    #[serde(rename = "ogTitle")]
    pub(crate) og_title: String,
    #[serde(rename = "ogDescription")]
    pub(crate) og_description: String,
    #[serde(rename = "ogImage")]
    pub(crate) og_image: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct PostBlockDocument {
    #[serde(rename = "type")]
    pub(crate) kind: String,
    pub(crate) props: PostBlockPropsDocument,
}

#[derive(Debug, Serialize)]
pub(crate) struct PostBlockPropsDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) text: Option<String>,
}

impl From<&PostEditorState> for PostDocument {
    fn from(editor: &PostEditorState) -> Self {
        let mut locales = BTreeMap::new();
        locales.insert(
            "fr-fr".to_string(),
            PostLocaleDocument::new(
                editor.fr.slug.clone(),
                editor.fr.title.clone(),
                editor.fr.description.clone(),
                editor.fr.paragraph.clone(),
            ),
        );
        locales.insert(
            "en-us".to_string(),
            PostLocaleDocument::new(
                editor.en.slug.clone(),
                editor.en.title.clone(),
                editor.en.description.clone(),
                editor.en.paragraph.clone(),
            ),
        );

        Self {
            id: editor.id.clone(),
            kind: "editorial".to_string(),
            status: editor.status.clone(),
            author: editor.author.clone(),
            published_at: editor.date.clone(),
            updated_at: editor.date.clone(),
            locales,
        }
    }
}

impl PostLocaleDocument {
    fn new(slug: String, title: String, description: String, paragraph: String) -> Self {
        Self {
            slug,
            seo: PostSeoDocument {
                title: title.clone(),
                description: description.clone(),
                og_title: title.clone(),
                og_description: description,
                og_image: "/og/bezot-corp-default.png".to_string(),
            },
            blocks: vec![
                PostBlockDocument {
                    kind: "hero".to_string(),
                    props: PostBlockPropsDocument {
                        title: Some(title),
                        subtitle: Some("Édité depuis Bezot Project Studio.".to_string()),
                        text: None,
                    },
                },
                PostBlockDocument {
                    kind: "paragraph".to_string(),
                    props: PostBlockPropsDocument {
                        title: None,
                        subtitle: None,
                        text: Some(paragraph),
                    },
                },
            ],
        }
    }
}
