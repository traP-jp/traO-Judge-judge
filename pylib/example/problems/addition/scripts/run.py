#!python-traopy-v0
from traopy_util.util import v0 as trau # type: ignore[reportMissingModuleSource]
from traopy_util.util import common as trau_common # type: ignore[reportMissingModuleSource]
import os

language_tag = trau_common.read_file_with_envvar("LANGUAGE_TAG")
time_limit_ms = int(trau_common.read_file_with_envvar("TIME_LIMIT_MS"))
memory_limit_kib = int(trau_common.read_file_with_envvar("MEMORY_LIMIT_KIB"))
input_file = trau_common.read_file_with_envvar("INPUT_FILE")
expected_file = trau_common.read_file_with_envvar("EXPECTED_FILE")
output_file = f"{trau_common.read_file_with_envvar('TEMP_DIR')}/output.txt"
source_path = os.environ.get("BUILD_SOURCE_PATH")
build_output_path = os.environ.get("BUILD_OUTPUT_PATH")

language_info = trau.get_language_info(language_tag)
command = f"{language_info.run} < {input_file} > {output_file}"

exec_stats = trau.exec_with_stats(
    cmd=command,
    envs={
        trau.build_output_envvar(): build_output_path,
        trau.build_source_envvar(): source_path,
    },
    time_limit_ms=2000,
)


if exec_stats is None:
    json = trau.jsonify_displayable_output(
        status=trau.JudgeStatus.TLE,
        time_ms=2000,
        memory_kib=0,
        score=0,
        continue_next=True,
    )
else:
    if exec_stats is None:
        json = trau.jsonify_displayable_output(
            status=trau.JudgeStatus.TLE,
            time_ms=2000,
            memory_kib=0,
            score=0,
            continue_next=True,
        )
    else:
        if exec_stats.exit_code != 0:
            json = trau.jsonify_displayable_output(
                status=trau.JudgeStatus.CE,
                time_ms=exec_stats.time_ms,
                memory_kib=exec_stats.memory_kib,
                score=0,
                continue_next=True,
            )
        elif exec_stats.time_ms > time_limit_ms:
            json = trau.jsonify_displayable_output(
                status=trau.JudgeStatus.TLE,
                time_ms=exec_stats.time_ms,
                memory_kib=exec_stats.memory_kib,
                score=0,
                continue_next=True,
            )
        elif exec_stats.memory_kib > memory_limit_kib:
            json = trau.jsonify_displayable_output(
                status=trau.JudgeStatus.MLE,
                time_ms=exec_stats.time_ms,
                memory_kib=exec_stats.memory_kib,
                score=0,
                continue_next=True,
            )
        else:
            output_file_path = f"{os.environ.get('TEMP_DIR')}/output.txt"
            with open(output_file_path, "r") as f:
                output_file = f.read()
            if trau_common.normal_judge_checker(
                expected=expected_file,
                actual=output_file,
            ):
                json = trau.jsonify_displayable_output(
                    status=trau.JudgeStatus.AC,
                    time_ms=exec_stats.time_ms,
                    memory_kib=exec_stats.memory_kib,
                    score=100,
                    continue_next=True,
                )
            else:
                json = trau.jsonify_displayable_output(
                    status=trau.JudgeStatus.WA,
                    time_ms=exec_stats.time_ms,
                    memory_kib=exec_stats.memory_kib,
                    score=0,
                    continue_next=True,
                )
print(json)
outcome_path = os.environ.get(trau.exec_job_outcome_path_envvar())
with open(outcome_path, "w") as f:
    f.write(json)
