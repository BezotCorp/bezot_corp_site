#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Page {
    Dashboard,
    Library,
    Editor,
    SiteTools,
}

impl Page {
    pub(crate) const ALL: [Page; 4] = [
        Self::Dashboard,
        Self::Library,
        Self::Editor,
        Self::SiteTools,
    ];

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Dashboard => "Tableau de bord",
            Self::Library => "Bibliothèque",
            Self::Editor => "Éditeur",
            Self::SiteTools => "Site",
        }
    }
}
