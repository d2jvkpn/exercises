use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Clone)]
pub enum TaskStatus {
    Done,
    Pending,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Done => "Done",
            Self::Pending => "Pending",
        }
    }

    pub fn from_string<T: AsRef<str> + std::fmt::Display>(input_string: T) -> Self {
        match input_string.as_ref() {
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
        s.serialize_field("status", &self.as_str())?;
        s.end()
    }
}
