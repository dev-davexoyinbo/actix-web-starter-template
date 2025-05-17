use std::{fmt, sync::Arc};

use crate::api::auth_module::auth_models::AuthInfo;

type InnerAccessControlFunction = Arc<dyn Fn(&AuthInfo) -> bool + Send + Sync>;

/// Represents different types of access control conditions
#[derive(Clone, Debug)]
pub enum AccessControl {
    /// Role-based access control
    Role(String),
    /// Permission-based access control
    Permission(String),
    /// Function-based access control using a closure
    Function(AccessControlFunction),
}

#[derive(Clone)]
pub struct AccessControlFunction(pub InnerAccessControlFunction);

impl fmt::Debug for AccessControlFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AccessControlFunction(<closure>)")
    }
}
