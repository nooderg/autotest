use crate::core::domain::testing::Testing;

#[derive(Serialize, Deserialize)]
pub struct TestingSummary {
    file: String
}

impl TestingSummary {
    pub fn new(testing: Testing) -> TestingSummary {
        TestingSummary{
            file : testing.file
        }
    }
}