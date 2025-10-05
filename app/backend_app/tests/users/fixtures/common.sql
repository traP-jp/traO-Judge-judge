ALTER TABLE users MODIFY updated_at DATETIME DEFAULT CURRENT_TIMESTAMP;

INSERT INTO users 
(id, name, role, github_id, self_introduction, email, created_at, updated_at) 
VALUES 
(
    UNHEX(REPLACE('11111111-1111-1111-1111-111111111111','-','')), 
    'test_user_1', 
    0,
    "test_github_id_1",
    "test_self_introduction_1",
    "test1@test.com",
    '2023-01-01 09:15:32',
    '2023-01-01 10:20:47'
);

INSERT INTO user_authentications
(user_id, password, github_oauth, google_oauth, traq_oauth)
VALUES
(
    UNHEX(REPLACE('11111111-1111-1111-1111-111111111111','-','')),
    "test_password_1",
    "test_github_oauth_1",
    "test_google_oauth_1",
    "test_traq_oauth_1"
);

INSERT INTO users 
(id, name, role, traq_id, x_id, created_at, updated_at) 
VALUES 
(
    UNHEX(REPLACE('22222222-2222-2222-2222-222222222222','-','')), 
    'test_user_2', 
    1,
    "test_traq_id_2",
    "test_user_2",
    '2023-02-12 14:05:12',
    '2023-02-12 15:30:00'
);

INSERT INTO users 
(id, name, role, icon_id, created_at, updated_at) 
VALUES 
(
    UNHEX(REPLACE('33333333-3333-3333-3333-333333333333','-','')),
    'test_user_3', 
    2,
    UNHEX(REPLACE('33333333-3333-3333-3333-333333333333','-','')),
    '2023-03-20 08:00:00',
    '2023-03-20 08:45:00'
);

ALTER TABLE users MODIFY updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP;

ALTER TABLE normal_problems MODIFY updated_at DATETIME DEFAULT CURRENT_TIMESTAMP;
INSERT INTO normal_problems
(author_id, title, statement, time_limit, memory_limit, difficulty, is_public, created_at, updated_at)
VALUES
(
    1,
    'test',
    'a',
    10,
    10,
    1,
    TRUE,
    '2023-01-01 09:15:32',
    '2023-01-01 10:20:47'
);

ALTER TABLE normal_problems MODIFY updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP;
