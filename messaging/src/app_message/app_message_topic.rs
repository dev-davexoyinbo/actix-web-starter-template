use strum::{Display, EnumIter};

#[derive(EnumIter, Debug, Display, Clone)]
pub enum AppMessageTopic {
    #[strum(to_string = "general_email")]
    GeneralEmail,
    #[strum(to_string = "priority_email")]
    PriorityEmail,
}
