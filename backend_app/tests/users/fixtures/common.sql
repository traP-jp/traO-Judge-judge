INSERT INTO users 
(id, name, role, github_id, github_link, self_introduction, email) 
VALUES 
(
    UNHEX(REPLACE('11111111-1111-1111-1111-111111111111','-','')), 
    'test_user_1', 
    0,
    "test_github_id_1",
    "https://github.com/test_user_1",
    "test_self_introduction_1",
    "test1@test.com"
);

INSERT INTO users_passwords
(user_id, password)
VALUES
(
    UNHEX(REPLACE('11111111-1111-1111-1111-111111111111','-','')),
    "test_password_1"
);

INSERT INTO 
users (id, name, role, traq_id, x_link) 
VALUES 
(
    UNHEX(REPLACE('22222222-2222-2222-2222-222222222222','-','')), 
    'test_user_2', 
    1,
    "test_traq_id_2",
    "https://x.com/test_user_2"
);

INSERT INTO users 
(id, name, role, icon_url) 
VALUES 
(
    UNHEX(REPLACE('33333333-3333-3333-3333-333333333333','-','')),
    'test_user_3', 
    2,
    "https://icon.com/test_user_3"
);