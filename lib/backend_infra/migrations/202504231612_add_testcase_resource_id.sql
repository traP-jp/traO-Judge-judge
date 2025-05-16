ALTER TABLE `testcases`
    ADD COLUMN `input_id` BINARY(16) NOT NULL,
    ADD COLUMN `output_id` BINARY(16) NOT NULL;