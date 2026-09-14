/// How a model's on-disk size compares to the VRAM the operator declared
/// they have available. `size_bytes` from Ollama is the closest available
/// proxy for what a fully-loaded model needs in VRAM, so this is an
/// estimate, not a guarantee — context length and other loaded models still
/// affect the real number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VramFit {
    Unknown,
    Comfortable,
    Tight,
    TooLarge,
}

impl VramFit {
    pub(crate) fn assess(model_size_gb: f64, available_vram_gb: Option<f64>) -> Self {
        match available_vram_gb {
            None => Self::Unknown,
            Some(available) if model_size_gb <= available * 0.7 => Self::Comfortable,
            Some(available) if model_size_gb <= available => Self::Tight,
            Some(_) => Self::TooLarge,
        }
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Unknown => "VRAM non renseignée",
            Self::Comfortable => "✓ confortable",
            Self::Tight => "⚠ limite (réduis le contexte)",
            Self::TooLarge => "✗ trop gros pour ta VRAM (swap probable)",
        }
    }
}
