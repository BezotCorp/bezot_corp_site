#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Page {
    Dashboard,
    Library,
    Editor,
    Media,
    Ai,
    SiteTools,
}

impl Page {
    pub(crate) const ALL: [Page; 6] = [
        Self::Dashboard,
        Self::Library,
        Self::Editor,
        Self::Media,
        Self::Ai,
        Self::SiteTools,
    ];

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Dashboard => "Tableau de bord",
            Self::Library => "Bibliothèque",
            Self::Editor => "Éditeur",
            Self::Media => "Médias",
            Self::Ai => "IA",
            Self::SiteTools => "Site",
        }
    }
}
