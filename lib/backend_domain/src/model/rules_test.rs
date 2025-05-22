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
fn test_x_link_validate() {
    let rule = RuleType::XLink;
    assert!(rule.validate("https://x.com/abc").is_ok());
    assert!(rule.validate("http://x.com/abc").is_ok());
    assert!(rule.validate("https://www.x.com/abc").is_ok());
    assert!(rule.validate("http://www.x.com/abc").is_ok());
    assert!(rule.validate("https://x.com/abc/def").is_ok());
    assert!(rule.validate("www.x.com/abc/def").is_ok());
    assert!(rule.validate("x.com/abc/def/").is_ok());
    assert!(rule.validate("https://x.com").is_err());
    assert!(rule.validate("http://x.com").is_err());
    assert!(rule.validate("https://www.x.com").is_err());
    assert!(rule.validate("http://www.x.com").is_err());
    assert!(rule.validate("www.x.com/").is_err());
    assert!(rule.validate("x.com/").is_err());
    assert!(rule.validate("www.x.com").is_err());
    assert!(rule.validate("x.com").is_err());

    assert!(rule.validate("https://twitter.com/abc").is_ok());
    assert!(rule.validate("https://twitter.com").is_err());

    assert!(rule.validate("https://x.com/abc/_D_e_f_").is_ok());
    assert!(rule.validate("https://x.com/abc/-Def").is_err());

    assert!(rule.validate("https://x.com/01234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567").is_ok());
    assert!(rule.validate("https://x.com/012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678").is_err());

    assert!(rule.validate("https://x.com/home").is_ok());
    assert!(rule.validate("https://x.com/explore").is_ok());
    assert!(rule.validate("https://x.com/notification").is_ok());
    assert!(rule.validate("https://x.com/messages").is_ok());
    assert!(rule.validate("https://x.com/abc/status/0123456789").is_ok());
}

#[test]
fn test_github_link_validate() {
    let rule = RuleType::GitHubLink;
    assert!(rule.validate("https://github.com/abc").is_ok());
    assert!(rule.validate("http://github.com/abc").is_ok());
    assert!(rule.validate("https://www.github.com/abc").is_ok());
    assert!(rule.validate("http://www.github.com/abc").is_ok());
    assert!(rule.validate("https://github.com/abc/def").is_ok());
    assert!(rule.validate("www.github.com/abc/def").is_ok());
    assert!(rule.validate("github.com/abc/def/").is_ok());
    assert!(rule.validate("https://github.com").is_err());
    assert!(rule.validate("http://github.com").is_err());
    assert!(rule.validate("https://www.github.com").is_err());
    assert!(rule.validate("http://www.github.com").is_err());
    assert!(rule.validate("www.github.com/").is_err());
    assert!(rule.validate("github.com/").is_err());
    assert!(rule.validate("www.github.com").is_err());
    assert!(rule.validate("github.com").is_err());

    assert!(rule.validate("https://github.com/abc/_D_e_-f_").is_ok());
    assert!(rule.validate("https://github.com/abc/@Def").is_err());

    assert!(rule.validate("https://github.com/01234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567").is_ok());
    assert!(rule.validate("https://github.com/012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678").is_err());

    assert!(
        rule.validate("https://github.com/a/a/projects?query=is%3Aopen")
            .is_err()
    );
}
