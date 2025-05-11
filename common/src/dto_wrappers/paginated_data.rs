use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct PaginatedData<T> {
    total: u64,
    per_page: u64,
    current_page: u64,
    last_page: u64,
    from: u64,
    to: u64,
    data: Vec<T>,
}

pub struct PaginatedDataOptions<T>
where
    T: Serialize,
{
    pub data: Vec<T>,
    pub total: u64,
    pub per_page: u64,
    pub current_page: u64,
}

impl<T> PaginatedData<T>
where
    T: Serialize,
{
    pub fn new(options: PaginatedDataOptions<T>) -> PaginatedData<T> {
        let PaginatedDataOptions {
            data,
            total,
            per_page,
            current_page,
        } = options;

        let limit = per_page;
        let offset = (current_page - 1) * limit;

        let last_page: u64 = (total as f64 / limit as f64).ceil() as u64;
        let mut from = total;
        let mut to = total;

        if !data.is_empty() {
            from = offset + 1;
            to = offset + data.len() as u64;
        }

        PaginatedData {
            total,
            per_page,
            current_page,
            last_page,
            from,
            to,
            data,
        }
    }
}

impl<T> From<PaginatedDataOptions<T>> for PaginatedData<T>
where
    T: Serialize,
{
    fn from(options: PaginatedDataOptions<T>) -> Self {
        PaginatedData::new(options)
    }
}
