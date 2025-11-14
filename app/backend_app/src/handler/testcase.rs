use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::{TypedHeader, headers::Cookie};

use crate::{
    di::DiContainer,
    model::{error::AppError, testcase::{
        CreateTestcaseRequest, TestcaseResponse, TestcaseSummaryResponse, UpdateTestcaseRequest,
    }},
};

pub async fn get_testcase(
    State(di_container): State<DiContainer>,
    Path(testcase_id): Path<String>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .testcase_service()
        .get_testcase(session_id, testcase_id)
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
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .testcase_service()
        .get_testcases(session_id, problem_id)
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
    TypedHeader(cookie): TypedHeader<Cookie>,
    Json(testcases): Json<Vec<CreateTestcaseRequest>>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .testcase_service()
        .post_testcases(
            session_id,
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
    TypedHeader(cookie): TypedHeader<Cookie>,
    Json(testcase): Json<UpdateTestcaseRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .testcase_service()
        .put_testcase(session_id, testcase_id, testcase.into())
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(AppError(e).into()),
    }
}

pub async fn delete_testcase(
    State(di_container): State<DiContainer>,
    Path(testcase_id): Path<String>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .testcase_service()
        .delete_testcase(session_id, testcase_id)
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(AppError(e).into()),
    }
}
