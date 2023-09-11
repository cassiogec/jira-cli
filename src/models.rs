pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

pub struct Epic {
    id: u32,
    name: String,
    description: String,
    status: Status,
    stories: Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: 1,
            name,
            description,
            status: Status::Open,
            stories: Vec::new(),
        }
    }
}

pub struct Story {
    id: u32,
    name: String,
    description: String,
    status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: 1,
            name,
            description,
            status: Status::Open,
        }
    }
}

pub struct DBState {
    last_item_id: u32,
    epics: Vec<Epic>,
    stories: Vec<Story>,
}
