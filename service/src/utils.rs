//! Utilities to use vectors as hashmaps (which should be faster for small vectors)

pub(crate) fn upsert(vec: &mut Vec<(String, String)>, key: &str, value: &str) {
    if let Some((_, v)) = vec.iter_mut().find(|(k, _)| k == &key) {
        *v = value.to_string(); // Update existing value
    } else {
        vec.push((key.to_string(), value.to_string())); // Insert new entry
    }
}

pub(crate) fn insert_if_absent(vec: &mut Vec<(String, String)>, key: &str, value: &str) {
    if !vec.iter().any(|(k, _)| k == key) {
        vec.push((key.to_string(), value.to_string()));
    }
}

#[cfg(test)]
mod test {
    use super::{insert_if_absent, upsert};

    #[test]
    fn test_uspert_with_insert() {
        let mut sample = vec![
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "2".to_string()),
        ];

        // element not yet present, should be inserted
        upsert(&mut sample, "c", "3");

        let expected = vec![
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "2".to_string()),
            ("c".to_string(), "3".to_string()),
        ];
        assert_eq!(sample, expected);
    }

    #[test]
    fn test_upsert_with_replace() {
        let mut sample = vec![
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "2".to_string()),
        ];

        // element already present, should be updated
        upsert(&mut sample, "b", "3");

        let expected = vec![
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "3".to_string()),
        ];
        assert_eq!(sample, expected);
    }

    #[test]
    fn test_insert_with_element_already_present() {
        let mut sample = vec![
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "2".to_string()),
        ];

        // element already present, should not be overriden
        insert_if_absent(&mut sample, "b", "3");

        let expected = vec![
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "2".to_string()),
        ];
        assert_eq!(sample, expected);
    }

    #[test]
    fn test_insert_with_element_not_yet_present() {
        let mut sample = vec![
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "2".to_string()),
        ];

        // element not yet present, should be inserted
        insert_if_absent(&mut sample, "c", "3");

        let expected = vec![
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "2".to_string()),
            ("c".to_string(), "3".to_string()),
        ];
        assert_eq!(sample, expected);
    }
}
