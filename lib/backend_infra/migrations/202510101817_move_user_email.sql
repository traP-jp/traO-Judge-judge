ALTER TABLE `users` DROP COLUMN `email`;
ALTER TABLE `user_authentications` ADD COLUMN `email` VARCHAR(255) UNIQUE KEY;