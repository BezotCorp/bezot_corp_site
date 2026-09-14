#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum AiTask {
    #[default]
    Idle,
    LoadingModels,
    GeneratingDraft,
    RunningReview,
}

impl AiTask {
    pub(crate) fn is_busy(self) -> bool {
        self != Self::Idle
    }
}
