ALTER TABLE `users_passwords` RENAME TO `user_authentications`;
ALTER TABLE `user_authentications`
    MODIFY COLUMN `password` VARCHAR(255) NULL,
    ADD COLUMN `github_oauth` VARCHAR(255) UNIQUE KEY,
    ADD COLUMN `google_oauth` VARCHAR(255) UNIQUE KEY,
    ADD COLUMN `traq_oauth` VARCHAR(255) UNIQUE KEY;