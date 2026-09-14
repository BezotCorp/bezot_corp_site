use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostLocaleEditor {
    pub title: String,
    #[serde(default)]
    pub subtitle: String,
    pub slug: String,
    pub description: String,
    #[serde(default)]
    pub og_image: String,
    pub paragraphs: Vec<String>,
    #[serde(default)]
    pub affiliate_title: String,
    #[serde(default)]
    pub affiliate_text: String,
    #[serde(default)]
    pub affiliate_url: String,
    #[serde(default)]
    pub affiliate_label: String,
    #[serde(default)]
    pub affiliate_disclosure: String,
}

impl PostLocaleEditor {
    pub fn french_default() -> Self {
        Self {
            title: "Nouvel article Bezot Corp".to_string(),
            subtitle: String::new(),
            slug: "blog/nouvel-article".to_string(),
            description: "Article édité depuis Bezot Project Studio.".to_string(),
            og_image: "/og/bezot-corp-default.png".to_string(),
            paragraphs: vec!["Texte de l’article à compléter.".to_string()],
            affiliate_title: String::new(),
            affiliate_text: String::new(),
            affiliate_url: String::new(),
            affiliate_label: "Voir l’offre".to_string(),
            affiliate_disclosure: "Lien affilié ou sponsorisé.".to_string(),
        }
    }

    pub fn english_default() -> Self {
        Self {
            title: "New Bezot Corp article".to_string(),
            subtitle: String::new(),
            slug: "blog/new-article".to_string(),
            description: "Article edited from Bezot Project Studio.".to_string(),
            og_image: "/og/bezot-corp-default.png".to_string(),
            paragraphs: vec!["Article text to complete.".to_string()],
            affiliate_title: String::new(),
            affiliate_text: String::new(),
            affiliate_url: String::new(),
            affiliate_label: "View offer".to_string(),
            affiliate_disclosure: "Affiliate or sponsored link.".to_string(),
        }
    }
}
