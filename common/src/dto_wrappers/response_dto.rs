use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ResponseDto<T>
where
    T: Serialize,
{
    pub message: String,
    pub data: T,
}

impl<T> ResponseDto<T>
where
    T: Serialize,
{
    pub fn new(message: String, data: T) -> Self {
        ResponseDto { message, data }
    }
}
