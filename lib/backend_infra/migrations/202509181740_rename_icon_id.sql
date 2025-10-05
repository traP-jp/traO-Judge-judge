ALTER TABLE `users` DROP COLUMN `icon_url`;
ALTER TABLE `users` ADD COLUMN `icon_id` BINARY(16);