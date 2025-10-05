ALTER TABLE `users` DROP COLUMN `x_link`;
ALTER TABLE `users` DROP COLUMN `github_link`;
ALTER TABLE `users` ADD COLUMN `x_id` VARCHAR(255);
