CREATE TABLE IF NOT EXISTS `icons` (
    `id` BINARY(16) NOT NULL,
    `content_type` VARCHAR(255) NOT NULL,
    `icon` BYTEA NOT NULL,
);