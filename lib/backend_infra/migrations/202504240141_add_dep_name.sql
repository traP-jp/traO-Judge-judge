CREATE TABLE IF NOT EXISTS `dep_name` (
    `dep_id` BINARY(16) NOT NULL PRIMARY KEY,
    `name` VARCHAR(255) NOT NULL,
    `problem_id` INT NOT NULL
);