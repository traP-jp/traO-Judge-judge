CREATE TABLE IF NOT EXISTS `resource_id_counter` (
    resource_id VARCHAR(36) NOT NULL,
    ref_count INT DEFAULT 0,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (resource_id),
    INDEX idx_ref_updated (ref_count, updated_at)
);

ALTER TABLE procedures CONVERT TO CHARACTER SET utf8mb4 COLLATE utf8mb4_uca1400_ai_ci;

INSERT INTO resource_id_counter (resource_id, ref_count)
SELECT 
    t.resource_id,
    COUNT(*) as ref_count
FROM procedures p,
JSON_TABLE(
    p.`procedure`,
    '$.texts[*]' COLUMNS (
        resource_id VARCHAR(36) COLLATE utf8mb4_uca1400_ai_ci PATH '$.resource_id'
    )
) AS t
WHERE t.resource_id IS NOT NULL
GROUP BY t.resource_id
ON DUPLICATE KEY UPDATE ref_count = VALUES(ref_count);

DROP TRIGGER IF EXISTS trigger_procedures_insert;
DROP TRIGGER IF EXISTS trigger_procedures_delete;
DROP TRIGGER IF EXISTS trigger_procedures_update;

CREATE TRIGGER trigger_procedures_insert
AFTER INSERT ON procedures
FOR EACH ROW
BEGIN
    INSERT INTO resource_id_counter (resource_id, ref_count)
    SELECT resource_id, 1
    FROM JSON_TABLE(
        NEW.`procedure`,
        '$.texts[*]' COLUMNS (resource_id VARCHAR(36) COLLATE utf8mb4_uca1400_ai_ci PATH '$.resource_id')
    ) as t
    WHERE resource_id IS NOT NULL
    ON DUPLICATE KEY UPDATE ref_count = ref_count + 1;
END;

CREATE TRIGGER trigger_procedures_delete
AFTER DELETE ON procedures
FOR EACH ROW
BEGIN
    UPDATE resource_id_counter rt
    JOIN JSON_TABLE(
        OLD.`procedure`,
        '$.texts[*]' COLUMNS (resource_id VARCHAR(36) COLLATE utf8mb4_uca1400_ai_ci PATH '$.resource_id')
    ) as t ON rt.resource_id = t.resource_id
    SET rt.ref_count = rt.ref_count - 1;
END;


CREATE TRIGGER trigger_procedures_update
AFTER UPDATE ON procedures
FOR EACH ROW
BEGIN
    UPDATE resource_id_counter rt
    JOIN JSON_TABLE(
        OLD.`procedure`,
        '$.texts[*]' COLUMNS (resource_id VARCHAR(36) COLLATE utf8mb4_uca1400_ai_ci PATH '$.resource_id')
    ) as t_old ON rt.resource_id = t_old.resource_id
    SET rt.ref_count = rt.ref_count - 1;

    INSERT INTO resource_id_counter (resource_id, ref_count)
    SELECT resource_id, 1
    FROM JSON_TABLE(
        NEW.`procedure`,
        '$.texts[*]' COLUMNS (resource_id VARCHAR(36) COLLATE utf8mb4_uca1400_ai_ci PATH '$.resource_id')
    ) as t_new
    WHERE resource_id IS NOT NULL
    ON DUPLICATE KEY UPDATE ref_count = ref_count + 1;
END;