#!python-traopy-v0
from traopy_util.util import v0 as trau # type: ignore[reportMissingModuleSource]
from traopy_util.util import common as trau_common # type: ignore[reportMissingModuleSource]

testcase_count = int(trau_common.read_file_with_envvar("TESTCASE_COUNT"))
ac_point = int(trau_common.read_file_with_envvar("AC_POINT"))

judge_statuses: list[trau.ExecutionResult] = []

for i in range(testcase_count):
    json = trau_common.read_file_with_envvar(f"OUTPUT_JSON_{i}")
    judge_status = trau.dejsonify_output(json)
    if judge_status is None:
        judge_status = trau.ExecutionResult(
            status=trau.JudgeStatus.WE,
            time=0.0,
            memory=0.0,
            score=0,
        )
    judge_statuses.append(judge_status)

status = trau.merge_judge_status(judge_statuses)
time = max(
    judge_status.time for judge_status in judge_statuses
)
memory = max(
    judge_status.memory for judge_status in judge_statuses
)
score = min(
    judge_status.score for judge_status in judge_statuses
)
json = trau.jsonify_displayable_output(
    status=status.status,
    time_ms=time,
    memory_kib=memory,
    score=score,
    continue_next=False,
)
print(json)
