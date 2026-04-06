//! OpenAPI specification parsing and processing
//!
//! This module handles the core logic for parsing OpenAPI specifications
//! and transforming them into the internal data structures used by the
//! XDK generator system.

use crate::Result;
use crate::models::{Metadata, OperationGroup, OperationInfo};
use openapi::OpenApi;
use std::collections::HashMap;

/// Extract operations by tag from the OpenAPI specification with automatic name transformations
pub fn extract_operations_by_tag(
    openapi: &OpenApi,
) -> Result<HashMap<Vec<String>, Vec<OperationGroup>>> {
    let mut operations_by_tag: HashMap<Vec<String>, Vec<OperationGroup>> = HashMap::new();

    /// Helper function to process an operation and add it to the operations_by_tag map
    fn process_operation(
        operations_by_tag: &mut HashMap<Vec<String>, Vec<OperationGroup>>,
        path: &str,
        method: &str,
        operation: &Option<openapi::Operation>,
    ) {
        if let Some(op) = operation
            && let Some(tags) = &op.tags
            && let Some(first_tag) = tags.first()
        {
            let normalized_tag: Vec<String> = normalize_tag(first_tag);

            let normalized_operation_id: Vec<String> = normalize_operation_id(&op.operation_id);

            let operation_info = OperationInfo {
                path: path.to_string(),
                method: method.to_string(),
                class_name: String::new(), // Will be set when casing is applied
                method_name: String::new(), // Will be set when casing is applied
                summary: op.summary.clone(),
                description: op.description.clone(),
                parameters: None, // Will be processed with casing in the macro
                security: op.security.clone(),
                request_body: op.request_body.clone(),
                responses: op.responses.clone(),
                is_streaming: op.streaming.unwrap_or(false),
            };
            let operation_group = OperationGroup {
                operation: operation_info,
                metadata: Metadata {
                    normalized_operation_id: clean_operation_id(
                        normalized_operation_id,
                        &normalized_tag,
                    ),
                },
                raw_parameters: op.parameters.clone(),
            };
            operations_by_tag
                .entry(normalized_tag)
                .or_default()
                .push(operation_group);
        }
    }

    for (path, path_item) in &openapi.paths {
        // Process each HTTP method type using the same logic
        process_operation(&mut operations_by_tag, path, "get", &path_item.get);
        process_operation(&mut operations_by_tag, path, "post", &path_item.post);
        process_operation(&mut operations_by_tag, path, "put", &path_item.put);
        process_operation(&mut operations_by_tag, path, "delete", &path_item.delete);
        process_operation(&mut operations_by_tag, path, "patch", &path_item.patch);
    }

    Ok(operations_by_tag)
}

/// Normalize tag names for consistent processing
pub fn normalize_tag(tag: &str) -> Vec<String> {
    tag.split_whitespace()
        .map(|word| {
            word.to_lowercase()
                .replace("tweets", "posts")
                .replace("tweet", "post")
        })
        .collect()
}

/// Normalize operation ID into word components
pub fn normalize_operation_id(operation_id: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut prev_is_lower = false;
    let mut prev_is_upper = false;

    let mut chars = operation_id.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch.is_uppercase() {
            let next_is_lower = chars.peek().is_some_and(|c| c.is_lowercase());
            if !current.is_empty() && (prev_is_lower || (prev_is_upper && next_is_lower)) {
                words.push(std::mem::take(&mut current));
            }
        }
        // Handle characters that have multiple lowercase variants,
        // though typically it's just one character.
        for lower_ch in ch.to_lowercase() {
            current.push(lower_ch);
        }
        prev_is_lower = ch.is_lowercase();
        prev_is_upper = ch.is_uppercase();
    }

    if !current.is_empty() {
        words.push(current);
    }

    words
}

/// Clean operation ID by removing words that appear in the tag
pub fn clean_operation_id(
    operation_id_as_vec: Vec<String>,
    tag_as_slice: &[String],
) -> Vec<String> {
    let mut cleaned_operation_id = Vec::with_capacity(operation_id_as_vec.len());
    for word in operation_id_as_vec {
        if !tag_as_slice.contains(&word) {
            cleaned_operation_id.push(word.to_lowercase());
        }
    }
    cleaned_operation_id
}
