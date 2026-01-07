ALTER TABLE normal_problems RENAME COLUMN `memory_limit_mib` TO `memory_limit_kib`;
ALTER TABLE submissions RENAME COLUMN `max_memory_mib` TO `max_memory_kib`;
ALTER TABLE submission_testcases RENAME COLUMN `memory_mib` TO `memory_kib`;