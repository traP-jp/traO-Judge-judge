ALTER TABLE `editorials`
    MODIFY `id` BINARY(16) NOT NULL;

ALTER TABLE `submissions`
    MODIFY `id` BINARY(16) NOT NULL;

ALTER TABLE `submission_testcases`
    MODIFY `submission_id` BINARY(16) NOT NULL;
