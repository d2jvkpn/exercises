use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Clone)]
pub enum TaskStatus {
    Done,
    Pending,
}

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match self {
            Self::Done => "Done".to_string(),
            Self::Pending => "Pending".to_string(),
        }
    }

    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "Done" => TaskStatus::Done,
            "Pending" => TaskStatus::Pending,
            _ => panic!("input {input_string} not supported"),
        }
    }
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("TaskStatus", 1)?;
        s.serialize_field("status", &self.stringify())?;
        s.end()
    }
}
