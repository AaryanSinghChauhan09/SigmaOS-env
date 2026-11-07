//! Options model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnsetMode {
    Remove,
    Ignore,
}

#[derive(Debug, Clone)]
pub struct EnvOptions {
    pub unset_mode: UnsetMode,
    pub null_separated: bool,
    pub ignore_case: bool,
}

impl Default for EnvOptions {
    fn default() -> Self {
        Self {
            unset_mode: UnsetMode::Remove,
            null_separated: false,
            ignore_case: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_options() {
        let o = EnvOptions::default();
        assert_eq!(o.unset_mode, UnsetMode::Remove);
    }
}
