//! Environment model.

#[derive(Debug, Clone)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub vars: Vec<EnvVar>,
}

impl Default for Environment {
    fn default() -> Self {
        Self { vars: vec![] }
    }
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, name: String, value: String) {
        if let Some(var) = self.vars.iter_mut().find(|v| v.name == name) {
            var.value = value;
        } else {
            self.vars.push(EnvVar { name, value });
        }
    }

    pub fn get(&self, name: &str) -> Option<String> {
        self.vars.iter().find(|v| v.name == name).map(|v| v.value.clone())
    }

    pub fn unset(&mut self, name: &str) {
        self.vars.retain(|v| v.name != name);
    }

    pub fn list(&self) -> Vec<EnvVar> {
        self.vars.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn environment() {
        let mut env = Environment::new();
        env.set("PATH".to_string(), "/usr/bin".to_string());
        assert_eq!(env.get("PATH"), Some("/usr/bin".to_string()));
    }
}
