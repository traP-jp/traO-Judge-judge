#!/usr/bin/env python3
from traopy_util.util import v0 as trau  # type: ignore[reportMissingModuleSource]
from traopy_util.util import common as trau_common  # type: ignore[reportMissingModuleSource]
import os

testcase_count = int(trau_common.read_file_with_envvar("TESTCASE_COUNT"))
ac_point = int(trau_common.read_file_with_envvar("AC_POINT"))

results: list[trau.ExecutionResult] = []
judge_statuses: list[trau.JudgeStatus] = []
for i in range(testcase_count):
    json_path = f"{os.environ.get(f'OUTPUT_JSON_{i}')}/out.json"
    with open(json_path, "r") as f:
        json = f.read()
    result = trau.dejsonify_output(json)
    if result is None:
        result = trau.ExecutionResult(
            status=trau.JudgeStatus.WE,
            time=0.0,
            memory=0.0,
            score=0,
        )
    results.append(result)
    judge_statuses.append(result.status)

status = trau.merge_judge_status(judge_statuses)
time = max((result.time for result in results), default=0.0)
memory = max((result.memory for result in results), default=0.0)
score = min((result.score for result in results), default=100)
json = trau.jsonify_displayable_output(
    status=status,
    time_ms=time,
    memory_kib=memory,
    score=score,
    continue_next=False,
)
print(json)
