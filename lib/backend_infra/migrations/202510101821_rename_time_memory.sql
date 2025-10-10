ALTER TABLE normal_problems RENAME COLUMN `time_limit` TO `time_limit_ms`;
ALTER TABLE normal_problems RENAME COLUMN `memory_limit` TO `memory_limit_mib`;
ALTER TABLE submissions RENAME COLUMN `max_time` TO `max_time_ms`;
ALTER TABLE submissions RENAME COLUMN `max_memory` TO `max_memory_mib`;
ALTER TABLE submission_testcases RENAME COLUMN `time` TO `time_ms`;
ALTER TABLE submission_testcases RENAME COLUMN `memory` TO `memory_mib`;
