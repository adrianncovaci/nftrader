use uuid::Uuid;
use serde::Serialize;
use serde::Deserialize;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag="input", content="payload")]
pub enum Input {
    Joined(JoinedInput),
    Posted(PostedInput)
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PostedInput {
    pub content: String
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct JoinedInput {
    pub nickname: String
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct InputParcel {
    pub client_id: Uuid,
    pub input: Input
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag="output", content="payload")]
pub enum Output {
    Error(ErrorType),
    Posted(PostedOutput),
    Joined(JoinedOutput),
    SelfJoined(SelfJoinedOutput)
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag="err", content="payload")]
pub enum ErrorType {
    NameTaken,
    InvalidName,
    InvalidContent,
    NotJoined
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PostedOutput {
    pub content: String
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct JoinedOutput {
    pub nickname: String
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct SelfJoinedOutput {
    pub client_id: Uuid,
    pub nickname: String
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct OutputParcel {
    pub client_id: Uuid,
    pub output: Output
}

impl OutputParcel {
    pub fn new(client_id: Uuid, output: Output) -> Self {
        Self {
            client_id,
            output
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_serialization() {
        let input = Input::Joined(JoinedInput { nickname: "vanea".to_string() });
        let serialized = serde_json::from_str(r#"{"input": "Joined", "payload": {"nickname": "vanea"}}"#).unwrap();
        assert_eq!(input, serialized);
    }

    #[test]
    fn test_deserialization() {
        let deserialized = r#"{"output":"Posted","payload":{"content":"derp"}}"#;
        let output = serde_json::to_string(&Output::Posted(PostedOutput {content: "derp".to_string()})).unwrap();
        assert_eq!(output, deserialized);
    }
}
