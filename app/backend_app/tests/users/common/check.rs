use serde_json::{Value, json};

pub fn users_check_by_id(id: i64, resp_json: &mut Value) -> anyhow::Result<()> {
    let users_json = match id {
        1 => json!({
            "id": "1",
            "name": "test_user_1",
            "traqId": null,
            "githubId": "test_github_id_1",
            "iconUrl": null,
            "postProblems": {
                "problems": [
                    {
                        "id": "1",
                        "title": "test",
                        "authorId": "1",
                        "isPublic": true,
                        "difficulty": 1,
                        "timeLimit": 10,
                        "memoryLimit": 10,
                        "solvedCount": 0,
                        "createdAt": "2023-01-01T09:15:32Z",
                        "updatedAt": "2023-01-01T10:20:47Z"
                    },
                ],
                "total": 1,
            },
            "submitProblems": {
                "submissions": [],
                "total": 0,
            },
            "xId": null,
            "selfIntroduction": "test_self_introduction_1",
            "role": "CommonUser",
            "createdAt": "2023-01-01T09:15:32Z",
            "updatedAt": "2023-01-01T10:20:47Z",
        }),
        2 => json!({
            "id": "2",
            "name": "test_user_2",
            "traqId": "test_traq_id_2",
            "githubId": null,
            "iconUrl": null,
            "postProblems": {
                "problems": [],
                "total": 0,
            },
            "submitProblems": {
                "submissions": [],
                "total": 0,
            },
            "xId": "test_user_2",
            "selfIntroduction": "",
            "role": "traPUser",
            "createdAt": "2023-02-12T14:05:12Z",
            "updatedAt": "2023-02-12T15:30:00Z",
        }),
        3 => json!({
            "id": "3",
            "name": "test_user_3",
            "traqId": null,
            "githubId": null,
            "iconUrl": std::env::var("ICON_HOST_URI").unwrap_or_default() + "/33333333-3333-3333-3333-333333333333",
            "postProblems": {
                "problems": [],
                "total": 0,
            },
            "submitProblems": {
                "submissions": [],
                "total": 0,
            },
            "xId": null,
            "selfIntroduction": "",
            "role": "Admin",
            "createdAt": "2023-03-20T08:00:00Z",
            "updatedAt": "2023-03-20T08:45:00Z",
        }),
        _ => return Err(anyhow::anyhow!("Invalid id")),
    };

    assert_eq!(resp_json, &users_json);

    Ok(())
}

pub fn users_me_check_by_id(id: i64, resp_json: &mut Value) -> anyhow::Result<()> {
    let users_json = match id {
        1 => json!({
            "id": "1",
            "name": "test_user_1",
            "traqId": null,
            "githubId": "test_github_id_1",
            "iconUrl": null,
            "postProblems": {
                "problems": [
                    {
                        "id": "1",
                        "title": "test",
                        "authorId": "1",
                        "isPublic": true,
                        "difficulty": 1,
                        "timeLimit": 10,
                        "memoryLimit": 10,
                        "solvedCount": 0,
                        "createdAt": "2023-01-01T09:15:32Z",
                        "updatedAt": "2023-01-01T10:20:47Z"
                    },
                ],
                "total": 1,
            },
            "submitProblems": {
                "submissions": [],
                "total": 0,
            },
            "xId": null,
            "selfIntroduction": "test_self_introduction_1",
            "role": "CommonUser",
            "createdAt": "2023-01-01T09:15:32Z",
            "updatedAt": "2023-01-01T10:20:47Z",
            "authentication": {
                "email": "test1@example.com",
                "googleAuth": null,
                "githubAuth": null,
                "traqAuth": null
            }
        }),
        2 => json!({
            "id": "2",
            "name": "test_user_2",
            "traqId": "test_traq_id_2",
            "githubId": null,
            "iconUrl": null,
            "postProblems": {
                "problems": [],
                "total": 0,
            },
            "submitProblems": {
                "submissions": [],
                "total": 0,
            },
            "xId": "test_user_2",
            "selfIntroduction": "",
            "role": "traPUser",
            "createdAt": "2023-02-12T14:05:12Z",
            "updatedAt": "2023-02-12T15:30:00Z",
            "authentication": {
                "email": null,
                "googleAuth": "test_google_oauth_2",
                "githubAuth": null,
                "traqAuth": "test_traq_oauth_2"
            }
        }),
        3 => json!({
            "id": "3",
            "name": "test_user_3",
            "traqId": null,
            "githubId": null,
            "iconUrl": std::env::var("ICON_HOST_URI").unwrap_or_default() + "/33333333-3333-3333-3333-333333333333",
            "postProblems": {
                "problems": [],
                "total": 0,
            },
            "submitProblems": {
                "submissions": [],
                "total": 0,
            },
            "xId": null,
            "selfIntroduction": "",
            "role": "Admin",
            "createdAt": "2023-03-20T08:00:00Z",
            "updatedAt": "2023-03-20T08:45:00Z",
            "authentication": {
                "email": "test3@example.com",
                "googleAuth": "test_google_oauth_3",
                "githubAuth": "test_github_oauth_3",
                "traqAuth": "test_traq_oauth_3"
            }
        }),
        _ => return Err(anyhow::anyhow!("Invalid id")),
    };

    assert_eq!(resp_json, &users_json);

    Ok(())
}
