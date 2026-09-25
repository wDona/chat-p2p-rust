use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    id: String,
    content: String,
    sender: String,
    receiver: String,
}