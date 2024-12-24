use player::ProtagData;

pub mod player;

/// Anything can borrow this thing for 'level.
#[derive(Debug)]
pub struct StaticAssets {
    pub protag: ProtagData,
}

impl StaticAssets {
    pub(crate) fn new() -> Self {
        Self {
            protag: ProtagData::new(),
        }
    }
}
