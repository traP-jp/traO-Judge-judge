#!/usr/bin/env python3
import os
import asyncio
from traopy_util.util import v0 as trau # type: ignore[reportMissingModuleSource]
from traopy_util.util import common as trau_common # type: ignore[reportMissingModuleSource]

async def main():
    language_tag = trau_common.read_file_with_envvar("LANGUAGE_TAG")
    language_info = trau.get_language_info(language_tag)
    outcome_path = os.environ.get(trau.exec_job_outcome_path_envvar())
    source_path = os.environ.get("BUILD_SOURCE_PATH")
    tempdir_path = os.environ.get("BUILD_TEMPDIR")
    exec_stats = await trau.exec_with_stats(
        cmd=language_info.compile,
        envs={
            trau.build_output_envvar(): f"{outcome_path}/main.out",
            trau.build_source_envvar(): source_path,
            trau.build_tempdir_envvar(): tempdir_path,
        },
        time_limit_ms=30000,
    )
    if exec_stats is None:
        json = trau.jsonify_displayable_output(
            status=trau.JudgeStatus.CE,
            time_ms=0,
            memory_kib=0,
            score=0,
            continue_next=False,
        )
    else:
        if exec_stats.exit_code != 0:
            json = trau.jsonify_displayable_output(
                status=trau.JudgeStatus.CE,
                time_ms=exec_stats.time_ms,
                memory_kib=exec_stats.memory_kib,
                score=0,
                continue_next=False,
            )
        else:
            json = trau.jsonify_displayable_output(
                status=trau.JudgeStatus.AC,
                time_ms=exec_stats.time_ms,
                memory_kib=exec_stats.memory_kib,
                score=0,
                continue_next=True,
            )

    print(json)

if __name__ == "__main__":
    asyncio.run(main())