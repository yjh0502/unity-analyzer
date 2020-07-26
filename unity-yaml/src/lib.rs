mod types;
pub use types::*;

impl Root {
    pub fn from_str(s: &str) -> serde_yaml::Result<Self> {
        serde_yaml::from_str(s)
    }
}
