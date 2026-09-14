#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum MediaTask {
    #[default]
    Idle,
    LoadingList,
    Uploading,
}

impl MediaTask {
    pub(crate) fn is_busy(self) -> bool {
        self != Self::Idle
    }
}
