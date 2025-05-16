use serde::{Deserialize, Serialize};
use sqlx::types::chrono;
use usecase::model::testcase::{
    CreateTestcaseData, TestcaseDto, TestcaseSummaryDto, UpdateTestcaseData,
};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestcaseSummaryResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<TestcaseSummaryDto> for TestcaseSummaryResponse {
    fn from(testcase: TestcaseSummaryDto) -> Self {
        TestcaseSummaryResponse {
            id: testcase.id,
            name: testcase.name,
            created_at: testcase.created_at,
            updated_at: testcase.updated_at,
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestcaseResponse {
    pub id: Uuid,
    pub name: String,
    pub test_input: String,
    pub test_output: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<TestcaseDto> for TestcaseResponse {
    fn from(testcase: TestcaseDto) -> Self {
        TestcaseResponse {
            id: testcase.id,
            name: testcase.name,
            test_input: testcase.input,
            test_output: testcase.output,
            created_at: testcase.created_at,
            updated_at: testcase.updated_at,
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTestcaseRequest {
    pub name: String,
    pub test_input: String,
    pub test_output: String,
}

impl From<CreateTestcaseRequest> for CreateTestcaseData {
    fn from(request: CreateTestcaseRequest) -> Self {
        CreateTestcaseData {
            name: request.name,
            input: request.test_input,
            output: request.test_output,
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTestcaseRequest {
    pub name: String,
    pub test_input: String,
    pub test_output: String,
}

impl From<UpdateTestcaseRequest> for UpdateTestcaseData {
    fn from(request: UpdateTestcaseRequest) -> Self {
        UpdateTestcaseData {
            name: request.name,
            input: request.test_input,
            output: request.test_output,
        }
    }
}
