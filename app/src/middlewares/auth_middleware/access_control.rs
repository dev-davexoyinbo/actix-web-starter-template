use std::{fmt, sync::Arc};

use crate::api::auth_module::auth_models::AuthInfo;

type InnerAccessControlFunction = Arc<dyn Fn(Option<&AuthInfo>) -> bool + Send + Sync>;

#[derive(Clone, Debug)]
pub enum AccessControl {
    Role(String),
    Permission(String),
    Guest,
    Function(AccessControlFunction),
}

#[derive(Clone)]
pub struct AccessControlFunction(pub InnerAccessControlFunction);

impl fmt::Debug for AccessControlFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AccessControlFunction(<closure>)")
    }
}
