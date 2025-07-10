use theframework::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {}

impl Default for Project {
    fn default() -> Self {
        Self::new()
    }
}

impl Project {
    pub fn new() -> Self {
        Self {}
    }
}
