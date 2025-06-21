use uuid::Uuid;

pub struct Secret {
    pub id: Uuid,
    pub secret_data: Vec<u8>,
}

impl Secret {
    pub fn new(data: Vec<u8>) -> Secret {
        Secret {
            id: Uuid::now_v7(),
            secret_data: data,
        }
    }
}
