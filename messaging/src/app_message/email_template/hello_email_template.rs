use askama::Template;
use serde::{Deserialize, Serialize};

use super::EmailTemplate;

#[derive(Template, Serialize, Deserialize, Debug, Clone)]
#[template(path = "hello.html")]
pub struct HelloEmailTemplate {
    pub name: String,
}

impl From<HelloEmailTemplate> for EmailTemplate {
    fn from(val: HelloEmailTemplate) -> EmailTemplate {
        EmailTemplate::Hello(val)
    }
}
