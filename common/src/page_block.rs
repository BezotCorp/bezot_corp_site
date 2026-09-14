use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct CardItem {
    pub title: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "props", rename_all = "snake_case")]
pub enum PageBlock {
    Hero {
        title: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        subtitle: String,
    },
    Paragraph {
        text: String,
    },
    MailLink {
        email: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        label: String,
    },
    CardGrid {
        items: Vec<CardItem>,
    },
    AffiliateCallout {
        title: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        text: String,
        url: String,
        label: String,
        disclosure: String,
    },
    PostList {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        limit: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        page: Option<u32>,
        #[serde(default, rename = "showDescription", skip_serializing_if = "is_false")]
        show_description: bool,
        #[serde(default, rename = "showAuthor", skip_serializing_if = "is_false")]
        show_author: bool,
        #[serde(default, rename = "showDate", skip_serializing_if = "is_false")]
        show_date: bool,
    },
}

fn is_false(value: &bool) -> bool {
    !*value
}

impl PageBlock {
    pub fn kind(&self) -> PageBlockKind {
        match self {
            Self::Hero { .. } => PageBlockKind::Hero,
            Self::Paragraph { .. } => PageBlockKind::Paragraph,
            Self::MailLink { .. } => PageBlockKind::MailLink,
            Self::CardGrid { .. } => PageBlockKind::CardGrid,
            Self::AffiliateCallout { .. } => PageBlockKind::AffiliateCallout,
            Self::PostList { .. } => PageBlockKind::PostList,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageBlockKind {
    Hero,
    Paragraph,
    MailLink,
    CardGrid,
    AffiliateCallout,
    PostList,
}

impl PageBlockKind {
    pub const ALL: [PageBlockKind; 6] = [
        Self::Hero,
        Self::Paragraph,
        Self::MailLink,
        Self::CardGrid,
        Self::AffiliateCallout,
        Self::PostList,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Hero => "Introduction (hero)",
            Self::Paragraph => "Paragraphe",
            Self::MailLink => "Lien email",
            Self::CardGrid => "Grille de cartes",
            Self::AffiliateCallout => "Encart affilié",
            Self::PostList => "Liste d’articles",
        }
    }

    pub fn new_block(self) -> PageBlock {
        match self {
            Self::Hero => PageBlock::Hero {
                title: String::new(),
                subtitle: String::new(),
            },
            Self::Paragraph => PageBlock::Paragraph {
                text: String::new(),
            },
            Self::MailLink => PageBlock::MailLink {
                email: String::new(),
                label: String::new(),
            },
            Self::CardGrid => PageBlock::CardGrid { items: Vec::new() },
            Self::AffiliateCallout => PageBlock::AffiliateCallout {
                title: String::new(),
                text: String::new(),
                url: String::new(),
                label: String::new(),
                disclosure: String::new(),
            },
            Self::PostList => PageBlock::PostList {
                limit: None,
                page: None,
                show_description: false,
                show_author: false,
                show_date: false,
            },
        }
    }
}
