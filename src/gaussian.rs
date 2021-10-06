use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct GaussConfig {}

#[derive(Debug, Clone, Copy)]
pub struct GaussInput {
    config: GaussConfig,
}
