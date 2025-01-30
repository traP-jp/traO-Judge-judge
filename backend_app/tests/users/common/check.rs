use super::remove_field;
use serde_json::{json, Value};

pub fn users_check_by_id(id: i64, resp_json: &mut Value) -> anyhow::Result<()> {
    let users_json = match id {
        1 => json!({
            "id": "11111111-1111-1111-1111-111111111111",
            "displayId": 1,
            "name": "test_user_1",
            "traqId": null,
            "githubId": "test_github_id_1",
            "iconUrl": null,
            "githubLink": "https://github.com/test_user_1",
            "xLink": null,
            "selfIntroduction": "test_self_introduction_1",
            "role": "commonUser",
        }),
        2 => json!({
            "id": "22222222-2222-2222-2222-222222222222",
            "displayId": 2,
            "name": "test_user_2",
            "traqId": "test_traq_id_2",
            "githubId": null,
            "iconUrl": null,
            "githubLink": null,
            "xLink": "https://x.com/test_user_2",
            "selfIntroduction": "",
            "role": "traPUser",
        }),
        3 => json!({
            "id": "33333333-3333-3333-3333-333333333333",
            "displayId": 3,
            "name": "test_user_3",
            "traqId": null,
            "githubId": null,
            "iconUrl": "https://icon.com/test_user_3",
            "githubLink": null,
            "xLink": null,
            "selfIntroduction": "",
            "role": "admin",
        }),
        _ => return Err(anyhow::anyhow!("Invalid id")),
    };
    let ignore_field = vec!["createdAt", "updatedAt"];

    remove_field(resp_json, &ignore_field);
    assert_eq!(resp_json, &users_json);

    Ok(())
}
