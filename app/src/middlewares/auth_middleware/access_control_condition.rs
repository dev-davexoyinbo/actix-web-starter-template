use crate::api::auth_module::auth_models::AuthInfo;

use super::{AccessControl, AccessControlFunction, AccessControlMiddlewareInitializer};

/// A condition builder for access control rules
#[derive(Clone, Debug)]
pub struct AccessControlCondition {
    conditions: Vec<AccessControlCheck>,
    mode: ConditionMode,
}

/// Internal enum to represent different types of condition checks
#[derive(Clone, Debug)]
enum AccessControlCheck {
    Simple(AccessControl),
    Nested(AccessControlCondition),
}

/// Mode for combining multiple conditions
#[derive(Clone, Debug)]
enum ConditionMode {
    All, // All conditions must pass (AND)
    Any, // Any condition must pass (OR)
}

impl AccessControlCondition {
    /// Create a new condition where all checks must pass
    pub fn all() -> Self {
        Self {
            conditions: Vec::new(),
            mode: ConditionMode::All,
        }
    }

    /// Create a new condition where any check must pass
    pub fn any() -> Self {
        Self {
            conditions: Vec::new(),
            mode: ConditionMode::Any,
        }
    }

    /// Add a condition to the existing set
    pub fn add(mut self, condition: impl Into<AccessControlCheck>) -> Self {
        self.conditions.push(condition.into());
        self
    }

    /// Check if the auth info satisfies the conditions
    pub fn check(&self, auth_info: Option<&AuthInfo>) -> bool {
        match self.mode {
            ConditionMode::All => self.conditions.iter().all(|c| c.check(auth_info)),
            ConditionMode::Any => self.conditions.iter().any(|c| c.check(auth_info)),
        }
    }

    /// Create a middleware initializer from this condition
    pub fn into_middleware(self) -> AccessControlMiddlewareInitializer {
        AccessControlMiddlewareInitializer { condition: self }
    }
}

impl AccessControlCheck {
    /// Check if the auth info satisfies this condition
    fn check(&self, auth_info: Option<&AuthInfo>) -> bool {
        match self {
            Self::Simple(control) => match control {
                AccessControl::Role(role) => auth_info.is_some_and(|info| info.has_role(role)),
                AccessControl::Permission(permission) => {
                    auth_info.is_some_and(|info| info.has_permission(permission))
                }
                AccessControl::Function(AccessControlFunction(f)) => f(auth_info),
                AccessControl::Guest => auth_info.is_none(),
            },
            Self::Nested(condition) => condition.check(auth_info),
        }
    }
}

impl From<AccessControl> for AccessControlCheck {
    fn from(control: AccessControl) -> Self {
        Self::Simple(control)
    }
}

impl From<AccessControlCondition> for AccessControlCheck {
    fn from(condition: AccessControlCondition) -> Self {
        Self::Nested(condition)
    }
}
