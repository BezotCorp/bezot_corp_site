#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ContentKindFilter {
    All,
    Pages,
    Posts,
}

impl ContentKindFilter {
    pub(crate) fn matches(self, kind: &str) -> bool {
        match self {
            Self::All => true,
            Self::Pages => kind == "page",
            Self::Posts => kind == "post",
        }
    }
}
