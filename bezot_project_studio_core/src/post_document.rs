use std::collections::BTreeMap;

use serde::Serialize;

use crate::post_editor_state::PostEditorState;

#[derive(Debug, Serialize)]
pub struct PostDocument {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub status: String,
    pub author: String,
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub locales: BTreeMap<String, PostLocaleDocument>,
}

#[derive(Debug, Serialize)]
pub struct PostLocaleDocument {
    pub slug: String,
    pub seo: PostSeoDocument,
    pub blocks: Vec<PostBlockDocument>,
}

#[derive(Debug, Serialize)]
pub struct PostSeoDocument {
    pub title: String,
    pub description: String,
    #[serde(rename = "ogTitle")]
    pub og_title: String,
    #[serde(rename = "ogDescription")]
    pub og_description: String,
    #[serde(rename = "ogImage")]
    pub og_image: String,
}

#[derive(Debug, Serialize)]
pub struct PostBlockDocument {
    #[serde(rename = "type")]
    pub kind: String,
    pub props: PostBlockPropsDocument,
}

#[derive(Debug, Serialize)]
pub struct PostBlockPropsDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disclosure: Option<String>,
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
                &editor.fr,
            ),
        );
        locales.insert(
            "en-us".to_string(),
            PostLocaleDocument::new(
                editor.en.slug.clone(),
                editor.en.title.clone(),
                editor.en.description.clone(),
                &editor.en,
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
    fn new(
        slug: String,
        title: String,
        description: String,
        editor: &crate::post_locale_editor::PostLocaleEditor,
    ) -> Self {
        let mut blocks = vec![
            PostBlockDocument {
                kind: "hero".to_string(),
                props: PostBlockPropsDocument {
                    title: Some(title.clone()),
                    subtitle: Some("Édité depuis Bezot Project Studio.".to_string()),
                    text: None,
                    url: None,
                    label: None,
                    disclosure: None,
                },
            },
            PostBlockDocument {
                kind: "paragraph".to_string(),
                props: PostBlockPropsDocument {
                    title: None,
                    subtitle: None,
                    text: Some(editor.paragraph.clone()),
                    url: None,
                    label: None,
                    disclosure: None,
                },
            },
        ];

        if !editor.affiliate_url.trim().is_empty() {
            blocks.push(PostBlockDocument {
                kind: "affiliate_callout".to_string(),
                props: PostBlockPropsDocument {
                    title: Some(editor.affiliate_title.clone()),
                    subtitle: None,
                    text: Some(editor.affiliate_text.clone()),
                    url: Some(editor.affiliate_url.clone()),
                    label: Some(editor.affiliate_label.clone()),
                    disclosure: Some(editor.affiliate_disclosure.clone()),
                },
            });
        }

        Self {
            slug,
            seo: PostSeoDocument {
                title: title.clone(),
                description: description.clone(),
                og_title: title.clone(),
                og_description: description,
                og_image: "/og/bezot-corp-default.png".to_string(),
            },
            blocks,
        }
    }
}
