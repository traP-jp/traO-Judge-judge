use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    di::DiContainer,
    extractor::session::ExtractedSessionUser,
    model::{
        error::AppError,
        testcase::{
            CreateTestcaseRequest, TestcaseResponse, TestcaseSummaryResponse, UpdateTestcaseRequest,
        },
    },
};

pub async fn get_testcase(
    State(di_container): State<DiContainer>,
    Path(testcase_id): Path<String>,
    ExtractedSessionUser(session_user): ExtractedSessionUser,
) -> Result<impl IntoResponse, StatusCode> {
    let resource_id = uuid::Uuid::parse_str(&testcase_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into();
    match di_container
        .testcase_service()
        .get_testcase(session_user, resource_id)
        .await
    {
        Ok(testcase) => {
            let resp = TestcaseResponse::from(testcase);
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => Err(AppError(e).into()),
    }
}

pub async fn get_testcases(
    State(di_container): State<DiContainer>,
    Path(problem_id): Path<String>,
    ExtractedSessionUser(session_user): ExtractedSessionUser,
) -> Result<impl IntoResponse, StatusCode> {
    let problem_id = problem_id
        .parse::<i64>()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into();
    match di_container
        .testcase_service()
        .get_testcases(session_user, problem_id)
        .await
    {
        Ok(testcases) => {
            let resp = testcases
                .into_iter()
                .map(TestcaseSummaryResponse::from)
                .collect::<Vec<_>>();
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => Err(AppError(e).into()),
    }
}

pub async fn post_testcase(
    State(di_container): State<DiContainer>,
    Path(problem_id): Path<String>,
    ExtractedSessionUser(session_user): ExtractedSessionUser,
    Json(testcases): Json<Vec<CreateTestcaseRequest>>,
) -> Result<impl IntoResponse, StatusCode> {
    let problem_id = problem_id
        .parse::<i64>()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into();

    match di_container
        .testcase_service()
        .post_testcases(
            session_user,
            problem_id,
            testcases.into_iter().map(|x| x.into()).collect(),
        )
        .await
    {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => Err(AppError(e).into()),
    }
}

pub async fn put_testcase(
    State(di_container): State<DiContainer>,
    Path(testcase_id): Path<String>,
    ExtractedSessionUser(session_user): ExtractedSessionUser,
    Json(testcase): Json<UpdateTestcaseRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let testcase_id = uuid::Uuid::parse_str(&testcase_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into();
    match di_container
        .testcase_service()
        .put_testcase(session_user, testcase_id, testcase.into())
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(AppError(e).into()),
    }
}

pub async fn delete_testcase(
    State(di_container): State<DiContainer>,
    Path(testcase_id): Path<String>,
    ExtractedSessionUser(session_user): ExtractedSessionUser,
) -> Result<impl IntoResponse, StatusCode> {
    let testcase_id = uuid::Uuid::parse_str(&testcase_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into();

    match di_container
        .testcase_service()
        .delete_testcase(session_user, testcase_id)
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(AppError(e).into()),
    }
}
