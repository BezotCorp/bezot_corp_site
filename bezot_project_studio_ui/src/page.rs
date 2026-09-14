#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Page {
    Dashboard,
    Library,
    Editor,
    Ai,
    SiteTools,
}

impl Page {
    pub(crate) const ALL: [Page; 5] = [
        Self::Dashboard,
        Self::Library,
        Self::Editor,
        Self::Ai,
        Self::SiteTools,
    ];

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Dashboard => "Tableau de bord",
            Self::Library => "Bibliothèque",
            Self::Editor => "Éditeur",
            Self::Ai => "IA",
            Self::SiteTools => "Site",
        }
    }
}
