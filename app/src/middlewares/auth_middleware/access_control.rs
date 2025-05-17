use std::{fmt, sync::Arc};

use crate::api::auth_module::auth_models::AuthInfo;

// Import from the module rather than specific files to avoid circular dependencies
use super::{AccessControlCondition, AccessControlMiddlewareInitializer};

type InnerAccessControlFunction = Arc<dyn Fn(Option<&AuthInfo>) -> bool + Send + Sync>;

#[derive(Clone, Debug)]
pub enum AccessControl {
    Role(String),
    Permission(String),
    Guest,
    Function(AccessControlFunction),
}

impl AccessControl {
    /// Converts an AccessControl directly into middleware by wrapping it in an AccessControlCondition
    pub fn into_middleware(self) -> AccessControlMiddlewareInitializer {
        match self {
            // For single condition access controls, we wrap them in a condition and then convert to middleware
            access_control => AccessControlCondition::all().add(access_control).into_middleware()
        }
    }
}

#[derive(Clone)]
pub struct AccessControlFunction(pub InnerAccessControlFunction);

impl fmt::Debug for AccessControlFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AccessControlFunction(<closure>)")
    }
}
