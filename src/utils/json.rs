/// Extracts the first complete JSON object from a buffer and returns it along with the remaining data.
///
/// This function counts '{' and '}' characters to determine when a complete JSON object is found.
/// When the brace count goes from >0 back to 0, we know we have a complete JSON object.
///
/// # Arguments
///
/// * `buffer` - The string buffer containing potentially partial JSON data
///
/// # Returns
///
/// Returns `Some((json_string, remaining_buffer))` if a complete JSON object is found,
/// or `None` if no complete JSON object is available yet.
pub(crate) fn extract_complete_json(buffer: &str) -> Option<(String, String)> {
  let mut brace_count = 0;
  let mut start_idx = None;

  for (i, ch) in buffer.char_indices() {
    match ch {
      '{' => {
        if brace_count == 0 {
          start_idx = Some(i);
        }
        brace_count += 1;
      }
      '}' => {
        brace_count -= 1;
        if brace_count == 0 {
          if let Some(start) = start_idx {
            // We found a complete JSON object
            let end = i + 1; // Include the closing brace
            let json_str = buffer[start..end].to_string();
            let remaining = buffer[end..].to_string();
            return Some((json_str, remaining));
          }
        }
      }
      _ => {}
    }
  }

  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_extract_complete_json() {
    // Test with a complete JSON object
    let buffer = r#"{"text": "hello"}"#;
    let result = extract_complete_json(buffer);
    assert!(result.is_some());
    let (json, remaining) = result.unwrap();
    assert_eq!(json, r#"{"text": "hello"}"#);
    assert_eq!(remaining, "");

    // Test with partial JSON (incomplete)
    let buffer = r#"{"text": "hel"#;
    let result = extract_complete_json(buffer);
    assert!(result.is_none());

    // Test with complete JSON followed by partial
    let buffer = r#"{"text": "hello"}{"text": "wor"#;
    let result = extract_complete_json(buffer);
    assert!(result.is_some());
    let (json, remaining) = result.unwrap();
    assert_eq!(json, r#"{"text": "hello"}"#);
    assert_eq!(remaining, r#"{"text": "wor"#);

    // Test with multiple complete JSON objects
    let buffer = r#"{"text": "hello"}{"text": "world"}"#;
    let result = extract_complete_json(buffer);
    assert!(result.is_some());
    let (json, remaining) = result.unwrap();
    assert_eq!(json, r#"{"text": "hello"}"#);
    assert_eq!(remaining, r#"{"text": "world"}"#);

    // Test with nested braces in string (should still work for simple JSON)
    let buffer = r#"{"text": "hello world"}"#;
    let result = extract_complete_json(buffer);
    assert!(result.is_some());
    let (json, remaining) = result.unwrap();
    assert_eq!(json, r#"{"text": "hello world"}"#);
    assert_eq!(remaining, "");
  }

  #[test]
  fn test_extract_complete_json_edge_cases() {
    // Test with whitespace around JSON
    let buffer = r#"  {"text": "hello"}  {"text": "world"}"#;
    let result = extract_complete_json(buffer);
    assert!(result.is_some());
    let (json, remaining) = result.unwrap();
    assert_eq!(json, r#"{"text": "hello"}"#);
    assert_eq!(remaining, r#"  {"text": "world"}"#);

    // Test with no JSON objects
    let buffer = "some random text without braces";
    let result = extract_complete_json(buffer);
    assert!(result.is_none());

    // Test with only opening brace
    let buffer = "{";
    let result = extract_complete_json(buffer);
    assert!(result.is_none());

    // Test with unmatched braces
    let buffer = "{{";
    let result = extract_complete_json(buffer);
    assert!(result.is_none());

    // Test empty buffer
    let buffer = "";
    let result = extract_complete_json(buffer);
    assert!(result.is_none());
  }
}
