use rand::{thread_rng, Rng};

pub trait IdProvider {
    fn get_id(&self) -> String;
}

pub struct SimpleIdProvider {
    length: usize,
}

impl SimpleIdProvider {
    pub fn new(length: usize) -> Self {
        Self { length }
    }
}

impl IdProvider for SimpleIdProvider {
    fn get_id(&self) -> String {
        let mut rng = thread_rng();

        (0..self.length)
            .map(|_| rng.gen_range(0..10).to_string())
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
        let provider = SimpleIdProvider::new(10);
        let id = provider.get_id();

        // Then
        assert_eq!(id.len(), 10);
    }

    #[tokio::test]
    async fn test_id_all_charterers_are_numbers() {
        // Given
        let provider = SimpleIdProvider::new(10);
        let id = provider.get_id();

        // Then
        assert!(id.chars().all(|c| c.is_ascii_digit()));
    }
}