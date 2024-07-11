use crate::utils::constants;

#[derive(Clone, Debug)]
pub struct Config {
    model: String,
    key: Option<String>,    
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model: constants::MODEL.to_string(),
            key: None,
        }
    }
}

#[allow(unused)]
impl Config {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            key: None,
        }
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }

    pub fn is_key_set(&self) -> bool {
        self.key.is_some()
    }

    pub fn set_key(&mut self, key: impl Into<String>) {
        self.key = Some(key.into());
    }

    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.set_key(key);
        self
    }
}
