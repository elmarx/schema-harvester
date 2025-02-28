//! Utilities to use vectors as hashmaps (which should be faster for small vectors)

pub(crate) trait VecExt {
    fn upsert(&mut self, key: String, value: String);
    fn insert_if_absent(&mut self, key: String, value: String);
}

impl VecExt for Vec<(String, String)> {
    fn upsert(&mut self, key: String, value: String) {
        if let Some((_, v)) = self.iter_mut().find(|(k, _)| k == &key) {
            *v = value; // Update existing value
        } else {
            self.push((key, value)); // Insert new entry
        }
    }

    fn insert_if_absent(&mut self, key: String, value: String) {
        if !self.iter().any(|(k, _)| *k == key) {
            self.push((key, value));
        }
    }
}

#[cfg(test)]
mod test {
    use crate::utils::VecExt;

    #[test]
    fn test_uspert_with_insert() {
        let mut sample = vec![
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "2".to_string()),
        ];

        // element not yet present, should be inserted
        sample.upsert("c".to_string(), "3".to_string());

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
        sample.upsert("b".to_string(), "3".to_string());

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
        sample.insert_if_absent("b".to_string(), "3".to_string());

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
        sample.insert_if_absent("c".to_string(), "3".to_string());

        let expected = vec![
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "2".to_string()),
            ("c".to_string(), "3".to_string()),
        ];
        assert_eq!(sample, expected);
    }
}
