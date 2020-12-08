use async_trait::async_trait;
use mockall::predicate::*;
use mockall::*;

#[automock]
#[async_trait]
pub trait Sick {
    async fn cough(&self) -> String;
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_thing() {
        let mut mock = MockSick::new();
        mock.expect_cough().returning(|| "hey".into());
        let got = mock.cough().await;
        let want = "hey".to_string();
        assert_eq!(got, want);
    }
}
