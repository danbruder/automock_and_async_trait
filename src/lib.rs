use async_trait::async_trait;
use mockall::predicate::*;
use mockall::*;

#[automock]
#[async_trait]
pub trait Sick {
    async fn cough(&self) -> String;
}

pub async fn hospital(patient: &impl Sick) -> String {
    patient.cough().await
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_thing() {
        let mut mock = MockSick::new();
        mock.expect_cough().returning(|| "hey".into());
        let got = hospital(&mock).await;
        let want = "hey".to_string();
        assert_eq!(got, want);
    }
}
