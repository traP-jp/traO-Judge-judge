CREATE TABLE IF NOT EXISTS `solved_status` (
    `user_id` INT,
    `problem_id` INT,
    `created_at` DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, problem_id)
);

DROP TRIGGER IF EXISTS `trigger_solved_add`;
DROP TRIGGER IF EXISTS `trigger_solved_remove`;
DROP TRIGGER IF EXISTS `trigger_submission_update`;
DROP TRIGGER IF EXISTS `trg_submissions_delete`;

-- solved_status にある (user_id, problem_id) の個数によって増減させる
CREATE TRIGGER `trigger_solved_add`
AFTER INSERT ON `solved_status`
FOR EACH ROW
UPDATE normal_problems SET solved_count = solved_count + 1 WHERE id = NEW.problem_id;

CREATE TRIGGER `trigger_solved_remove`
AFTER DELETE ON `solved_status`
FOR EACH ROW
UPDATE normal_problems SET solved_count = GREATEST(0, solved_count - 1) WHERE id = OLD.problem_id;

-- submissions の変更に伴って solved_status に insert したり delete したりする
CREATE TRIGGER `trigger_submission_update`
AFTER UPDATE ON submissions
FOR EACH ROW
BEGIN
    -- AC になった時 (WJ -> AC なので、INSERT は無視しても良い)
    IF (OLD.judge_status != 'AC' OR OLD.judge_status IS NULL) AND NEW.judge_status = 'AC' THEN
        INSERT IGNORE INTO solved_status (user_id, problem_id)
        VALUES (NEW.user_id, NEW.problem_id);
    END IF;

    -- AC から それ以外になった時 (自分以外にACがない場合)
    IF OLD.judge_status = 'AC' AND NEW.judge_status != 'AC' THEN
        IF NOT EXISTS (
            SELECT 1 FROM submissions 
            WHERE user_id = NEW.user_id 
              AND problem_id = NEW.problem_id 
              AND judge_status = 'AC' 
              AND id != NEW.id
        ) THEN
            DELETE FROM solved_status 
            WHERE user_id = NEW.user_id AND problem_id = NEW.problem_id;
        END IF;
    END IF;
END;

CREATE TRIGGER `trg_submissions_delete`
AFTER DELETE ON submissions
FOR EACH ROW
BEGIN
    IF OLD.judge_status = 'AC' THEN
        IF NOT EXISTS (
            SELECT 1 FROM submissions 
            WHERE user_id = OLD.user_id 
              AND problem_id = OLD.problem_id 
              AND judge_status = 'AC'
              AND id != OLD.id
        ) THEN
            DELETE FROM solved_status 
            WHERE user_id = OLD.user_id AND problem_id = OLD.problem_id;
        END IF;
    END IF;
END;


CREATE INDEX sub_user_prolem_status_idx ON submissions (user_id, problem_id, judge_status);