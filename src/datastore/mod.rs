#[cfg(test)]
mod tests;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PartitionId {
    project_id: String,
    namespace_id: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum PathElement {
    Id { kind: String, id: String },
    Name { kind: String, name: String },
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Key {
    partition_id: PartitionId,
    path: Vec<PathElement>,
}
