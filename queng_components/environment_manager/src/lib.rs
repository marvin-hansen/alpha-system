mod util;

use common_env::prelude::EnvironmentType;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct EnvironmentManager {
    dbg: bool,
    env_type: EnvironmentType,
}

impl EnvironmentManager {
    /// Creates a new instance of `EnvironmentManager`.
    ///
    /// The `new` method creates an instance of `EnvironmentManager` with debug mode disabled.
    ///
    /// # Returns
    ///
    /// The constructed instance of `EnvironmentManager`.
    ///
    pub fn new() -> Self {
        let env_type = util::detect_env_type(false);
        Self::build(false, &env_type)
    }

    /// Creates a new instance of `EnvironmentManager` with debug mode enabled.
    ///
    /// # Returns
    ///
    /// The constructed instance of `EnvironmentManager`.
    ///
    pub fn with_debug() -> Self {
        let env_type = util::detect_env_type(true);
        Self::build(true, &env_type)
    }

    fn build(dbg: bool, env_type: &EnvironmentType) -> EnvironmentManager {
        EnvironmentManager {
            dbg,
            env_type: env_type.clone(),
        }
    }
}

impl EnvironmentManager {
    pub fn env_type(&self) -> EnvironmentType {
        self.env_type
    }
}
