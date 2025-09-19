ALTER TABLE `editorials`
    MODIFY `id` BINARY(16) NOT NULL PRIMARY KEY;

ALTER TABLE `submissions`
    MODIFY `id` BINARY(16) NOT NULL PRIMARY KEY;

ALTER TABLE `submission_testcases`
    MODIFY `submission_id` BINARY(16) NOT NULL;
