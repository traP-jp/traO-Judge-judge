use crate::model::rules::RuleType;

#[test]
fn test_user_name_validate() {
    let rule = RuleType::UserName;
    assert!(rule.validate("Alice").is_ok());
    assert!(rule.validate("alice").is_ok());
    assert!(rule.validate("ALICE").is_ok());
    assert!(rule.validate("1234567890").is_ok());

    assert!(rule.validate("1").is_ok());
    assert!(rule.validate("a").is_ok());
    assert!(rule.validate("A").is_ok());
    assert!(rule.validate("_").is_err());
    assert!(rule.validate("-").is_err());

    assert!(rule.validate("12345678901234567890123456789012").is_ok());
    assert!(rule.validate("abcdefghijklmnopqrstuvlxyzABCDEF").is_ok());
    assert!(rule.validate("abcdefghijklmnopqrstuvlxyz-_0123").is_ok());
    assert!(rule.validate("123456789012345678901234567890123").is_err());
    assert!(rule.validate("abcdefghijklmnopqrstuvlxyzABCDEFG").is_err());
    assert!(rule.validate("abcdefghijklmnopqrstuvlxyz-_01234").is_err());

    assert!(rule.validate("Bob_-Alice").is_ok());
    assert!(rule.validate("_Alice").is_err());
    assert!(rule.validate("-Bob").is_err());
    assert!(rule.validate("Alice_").is_err());
    assert!(rule.validate("Bob-").is_err());

    assert!(rule.validate("Test Test").is_err());
    assert!(rule.validate("Test/Test").is_err());
}

#[test]
fn test_password_validate() {
    let rule = RuleType::Password;
    assert!(rule.validate("Aa123456").is_ok());
    assert!(rule.validate("AbcdEfgh").is_ok());
    assert!(rule.validate("Aa0@$!%*?&").is_ok());

    assert!(rule.validate("Aa12345").is_err());
    assert!(rule.validate("aa123456").is_err());
    assert!(rule.validate("AA123456").is_err());
    assert!(rule.validate("12345678").is_err());
    assert!(rule.validate("abcdefgh").is_err());
    assert!(rule.validate("ABCDEFGH").is_err());
    assert!(rule.validate("@$!%*?&@$").is_err());
    assert!(rule.validate("Aa12345678_").is_err());
    assert!(rule.validate("Aa12345678-").is_err());

    assert!(
        rule.validate("AaBb123456789012345678901234567890123456789012345678901234567890")
            .is_ok()
    );
    assert!(
        rule.validate("AaBb1234567890123456789012345678901234567890123456789012345678901")
            .is_err()
    );
}

#[test]
fn test_github_id_validate() {
    let rule = RuleType::GitHubId;
    assert!(rule.validate("abc").is_ok());
    assert!(rule.validate("Abc").is_ok());
    assert!(rule.validate("ABC").is_ok());
    assert!(rule.validate("abc-def").is_ok());
    assert!(rule.validate("").is_err());
    assert!(rule.validate("abc.def").is_err());
    assert!(rule.validate("abc/def").is_err());

    assert!(rule.validate("-D-e-f-").is_err());
    assert!(rule.validate("-Def").is_err());
    assert!(rule.validate("D--ef").is_err());

    assert!(rule.validate("012345678901234567890123456789012345678").is_ok());
    assert!(rule.validate("0123456789012345678901234567890123456789").is_err());
}

#[test]
fn test_x_id_validate() {
    let rule = RuleType::XId;
    assert!(rule.validate("abc").is_ok());
    assert!(rule.validate("Abc").is_ok());
    assert!(rule.validate("ABC").is_ok());
    assert!(rule.validate("").is_err());
    assert!(rule.validate("abc.def").is_err());
    assert!(rule.validate("abc-def").is_err());
    assert!(rule.validate("abc/def").is_err());

    assert!(rule.validate("_D_e_f_").is_ok());
    assert!(rule.validate("-Def").is_err());

    assert!(rule.validate("012345678901234").is_ok());
    assert!(rule.validate("0123456789012345").is_err());
}
