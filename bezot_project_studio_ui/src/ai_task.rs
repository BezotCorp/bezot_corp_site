#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum AiTask {
    #[default]
    Idle,
    GeneratingDraft,
    RunningReview,
}

impl AiTask {
    pub(crate) fn is_busy(self) -> bool {
        self != Self::Idle
    }
}
