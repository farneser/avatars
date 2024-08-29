use rand::{thread_rng, Rng};

pub trait IdProvider {
    fn get_id(&self, length: usize) -> String;
    fn get_numeric_id(&self, length: usize) -> String;
    fn get_from_alphabet(&self, alphabet: Vec<&str>, length: usize) -> String;
}

pub struct SimpleIdProvider {}

impl SimpleIdProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl IdProvider for SimpleIdProvider {
    fn get_id(&self, length: usize) -> String {
        self.get_from_alphabet(
            vec![
                "abcdefghijklmnopqrstuvwxyz",
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
                "0123456789",
                "!@#$%^&*()_+-=[]{}|;:,.<>?/`~",
            ],
            length,
        )
    }

    fn get_numeric_id(&self, length: usize) -> String {
        self.get_from_alphabet(vec!["0123456789"], length)
    }

    fn get_from_alphabet(&self, alphabet: Vec<&str>, length: usize) -> String {
        let mut rng = thread_rng();

        (0..length)
            .map(|_| {
                let index = rng.gen_range(0..alphabet.len());
                let charterers = alphabet[index];

                let char_id = rng.gen_range(0..charterers.len());

                charterers.chars().nth(char_id).unwrap()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_id_provider_length() {
        // Given
        let provider = SimpleIdProvider::new();
        let id = provider.get_numeric_id(10);

        // Then
        assert_eq!(id.len(), 10);
    }

    #[tokio::test]
    async fn test_id_all_charterers_are_numbers() {
        // Given
        let provider = SimpleIdProvider::new();
        let id = provider.get_numeric_id(10);

        // Then
        assert!(id.chars().all(|c| c.is_ascii_digit()));
    }

    #[tokio::test]
    async fn test_get_id_length() {
        // Given
        let provider = SimpleIdProvider::new();
        let id = provider.get_id(10);

        // Then
        assert_eq!(id.len(), 10, "ID length should be 10");
    }

    #[tokio::test]
    async fn test_get_numeric_id_length() {
        // Given
        let provider = SimpleIdProvider::new();
        let numeric_id = provider.get_numeric_id(8);

        // Then
        assert_eq!(numeric_id.len(), 8, "Numeric ID length should be 8");
    }

    #[tokio::test]
    async fn test_get_id_contains_valid_characters() {
        // Given
        let provider = SimpleIdProvider::new();
        let id = provider.get_id(20);
        let valid_chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;:,.<>?/`~".chars().collect();

        // Then
        assert!(id.chars().all(|c| valid_chars.contains(&c)), "ID should only contain valid characters");
    }

    #[tokio::test]
    async fn test_get_from_alphabet_length() {
        // Given
        let provider = SimpleIdProvider::new();
        let alphabet = vec!["abc", "123"];
        let result = provider.get_from_alphabet(alphabet, 12);

        // Then
        assert_eq!(result.len(), 12, "Result length should be 12");
    }

    #[tokio::test]
    async fn test_get_from_alphabet_valid_characters() {
        // Given
        let provider = SimpleIdProvider::new();
        let alphabet = vec!["abc", "123"];
        let result = provider.get_from_alphabet(alphabet, 10);
        let valid_chars: Vec<char> = "abc123".chars().collect();

        // Then
        assert!(result.chars().all(|c| valid_chars.contains(&c)), "Result should only contain characters from the provided alphabet");
    }
}
