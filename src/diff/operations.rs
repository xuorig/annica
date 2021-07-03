use super::common::OptionalStringDiff;
use super::parameters::ParametersDiff;
use super::request_body::RequestBodyDiff;
use openapiv3::Operation;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Serialize)]
pub struct OperationDiff {
    pub tags: TagsDiff,
    pub summary: Option<OptionalStringDiff>,
    pub description: Option<OptionalStringDiff>,
    pub operation_id: Option<OptionalStringDiff>,
    pub parameters: ParametersDiff,
    pub request_body: Option<RequestBodyDiff>,
}

impl OperationDiff {
    pub fn has_changes(&self) -> bool {
        self.tags.has_changes()
            || self.summary.is_some()
            || self.description.is_some()
            || self.operation_id.is_some()
    }

    pub fn from_operations(base: &Operation, head: &Operation) -> Self {
        let tags_diff = TagsDiff::from_tags(&base.tags, &head.tags);

        let summary_diff =
            OptionalStringDiff::from_strings(base.summary.clone(), head.summary.clone());

        let description_diff =
            OptionalStringDiff::from_strings(base.description.clone(), head.description.clone());

        let operation_id_diff =
            OptionalStringDiff::from_strings(base.operation_id.clone(), head.operation_id.clone());

        let parameters = ParametersDiff::from_params(&base.parameters, &head.parameters);

        let request_body_diff =
            RequestBodyDiff::from_request_bodies(&base.request_body, &head.request_body);

        let request_body = if request_body_diff.has_changes() {
            Some(request_body_diff)
        } else {
            None
        };

        Self {
            tags: tags_diff,
            summary: summary_diff,
            description: description_diff,
            operation_id: operation_id_diff,
            parameters,
            request_body,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TagsDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
}

impl TagsDiff {
    pub fn from_tags(base: &Vec<String>, head: &Vec<String>) -> Self {
        let base_set: HashSet<_> = base.iter().collect();
        let added: Vec<_> = head
            .iter()
            .filter(|item| !base_set.contains(item))
            .map(|item| item.clone())
            .collect();

        let head_set: HashSet<_> = head.iter().collect();
        let removed: Vec<_> = base
            .iter()
            .filter(|item| !head_set.contains(item))
            .map(|item| item.clone())
            .collect();

        Self { added, removed }
    }

    pub fn has_changes(&self) -> bool {
        !self.added.is_empty() || !self.removed.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_id_changed_from_none_to_some() {
        let base_operation = Operation::default();
        let mut head_operation = Operation::default();
        head_operation.operation_id = Some("cats/create".into());

        let diff = OperationDiff::from_operations(&base_operation, &head_operation);
        let op_id_diff = diff.operation_id.unwrap();

        assert_eq!(op_id_diff.from, None);
        assert_eq!(op_id_diff.to, Some("cats/create".into()));
    }

    #[test]
    fn operation_id_changed_from_some_to_some() {
        let mut base_operation = Operation::default();
        base_operation.operation_id = Some("cats-create".into());
        let mut head_operation = Operation::default();
        head_operation.operation_id = Some("cats/create".into());

        let diff = OperationDiff::from_operations(&base_operation, &head_operation);
        let op_id_diff = diff.operation_id.unwrap();

        assert_eq!(op_id_diff.from, Some("cats-create".into()));
        assert_eq!(op_id_diff.to, Some("cats/create".into()));
    }

    #[test]
    fn summary_changed_from_none_to_some() {
        let base_operation = Operation::default();
        let mut head_operation = Operation::default();
        head_operation.summary = Some("Creates a feline.".into());

        let diff = OperationDiff::from_operations(&base_operation, &head_operation);
        let summary_diff = diff.summary.unwrap();

        assert_eq!(summary_diff.from, None);
        assert_eq!(summary_diff.to, Some("Creates a feline.".into()));
    }

    #[test]
    fn summary_changed_from_some_to_some() {
        let mut base_operation = Operation::default();
        base_operation.summary = Some("Creates a cat.".into());
        let mut head_operation = Operation::default();
        head_operation.summary = Some("Creates a feline.".into());

        let diff = OperationDiff::from_operations(&base_operation, &head_operation);
        let summary_diff = diff.summary.unwrap();

        assert_eq!(summary_diff.from, Some("Creates a cat.".into()));
        assert_eq!(summary_diff.to, Some("Creates a feline.".into()));
    }

    #[test]
    fn description_changed_from_none_to_some() {
        let base_operation = Operation::default();
        let mut head_operation = Operation::default();
        head_operation.description = Some("Creates a feline.".into());

        let diff = OperationDiff::from_operations(&base_operation, &head_operation);
        let description_diff = diff.description.unwrap();

        assert_eq!(description_diff.from, None);
        assert_eq!(description_diff.to, Some("Creates a feline.".into()));
    }

    #[test]
    fn description_changed_from_some_to_some() {
        let mut base_operation = Operation::default();
        base_operation.description = Some("Creates a cat.".into());
        let mut head_operation = Operation::default();
        head_operation.description = Some("Creates a feline.".into());

        let diff = OperationDiff::from_operations(&base_operation, &head_operation);
        let description_diff = diff.description.unwrap();

        assert_eq!(description_diff.from, Some("Creates a cat.".into()));
        assert_eq!(description_diff.to, Some("Creates a feline.".into()));
    }

    #[test]
    fn operation_tags_added() {
        let mut base_operation = Operation::default();
        base_operation.tags.push("Cats".into());
        base_operation.tags.push("Dogs".into());

        let mut head_operation = Operation::default();
        head_operation.tags.push("Cats".into());
        head_operation.tags.push("Fish".into());

        let diff = OperationDiff::from_operations(&base_operation, &head_operation);

        assert_eq!(vec!["Fish"], diff.tags.added);
        assert_eq!(vec!["Dogs"], diff.tags.removed);
    }
}
