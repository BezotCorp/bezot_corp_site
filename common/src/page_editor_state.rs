use serde::{Deserialize, Serialize};

use crate::current_date::today_yyyy_mm_dd;
use crate::page_block::PageBlock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageEditorState {
    pub id: String,
    pub fr: PageLocaleEditor,
    pub en: PageLocaleEditor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageLocaleEditor {
    pub status: String,
    pub updated_at: String,
    pub slug: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub robots: String,
    #[serde(default)]
    pub og_title: String,
    #[serde(default)]
    pub og_description: String,
    #[serde(default)]
    pub og_image: String,
    pub blocks: Vec<PageBlock>,
}

impl Default for PageEditorState {
    fn default() -> Self {
        Self {
            id: String::new(),
            fr: PageLocaleEditor::french_default(),
            en: PageLocaleEditor::english_default(),
        }
    }
}

impl PageLocaleEditor {
    pub fn french_default() -> Self {
        let title = "Nouvelle page Bezot Corp".to_string();

        Self {
            status: "draft".to_string(),
            updated_at: today_yyyy_mm_dd(),
            slug: String::new(),
            title: title.clone(),
            description: String::new(),
            robots: "index, follow".to_string(),
            og_title: title.clone(),
            og_description: String::new(),
            og_image: "/og/bezot-corp-default.png".to_string(),
            blocks: vec![PageBlock::Hero {
                title,
                subtitle: String::new(),
            }],
        }
    }

    pub fn english_default() -> Self {
        let title = "New Bezot Corp page".to_string();

        Self {
            status: "draft".to_string(),
            updated_at: today_yyyy_mm_dd(),
            slug: String::new(),
            title: title.clone(),
            description: String::new(),
            robots: "index, follow".to_string(),
            og_title: title.clone(),
            og_description: String::new(),
            og_image: "/og/bezot-corp-default.png".to_string(),
            blocks: vec![PageBlock::Hero {
                title,
                subtitle: String::new(),
            }],
        }
    }
}
