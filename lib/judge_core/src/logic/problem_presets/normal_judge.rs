use crate::constant::label::single_judge;
use crate::constant::*;
use crate::logic::procedure_builder::ProcedureBuilder;
use crate::model::{procedure::writer_schema::*, *};

pub struct NormalJudgeTestcase {
    pub name: String,
    pub input: String,
    pub expected_output: String,
}

static NJ_BUILD_SCRIPT: &str = include_str!("./normal_judge_build.py");
static NJ_RUN_SCRIPT: &str = include_str!("./normal_judge_run.py");
static NJ_SUMMARIZE_SCRIPT: &str = include_str!("./normal_judge_summarize.py");

static COMPILE_PHASE_TIME_RESERVED_MS: i64 = 30000;
static TEST_PHASE_TIME_RESERVED_MS: i64 = 6000;
static SUMMARIZE_PHASE_TIME_RESERVED_MS: i64 = 2000;

pub fn create_normal_judge_procedure(
    testcases: Vec<NormalJudgeTestcase>,
) -> anyhow::Result<procedure::writer_schema::Procedure> {
    // 1st codeblock of builder.ipynb
    let mut builder = ProcedureBuilder::new();
    let source = builder.add_resource(ResourceKind::RuntimeTextFile(RuntimeText {
        name: "source".to_string(),
        label: single_judge::SUBMISSION_SOURCE.to_string(),
    }))?;
    let lang_tag = builder.add_resource(ResourceKind::RuntimeTextFile(RuntimeText {
        name: "language_tag".to_string(),
        label: single_judge::LANGUAGE_TAG.to_string(),
    }))?;
    let time_limit = builder.add_resource(ResourceKind::RuntimeTextFile(RuntimeText {
        name: "time_limit".to_string(),
        label: single_judge::TIME_LIMIT_MS.to_string(),
    }))?;
    let memory_limit = builder.add_resource(ResourceKind::RuntimeTextFile(RuntimeText {
        name: "memory_limit".to_string(),
        label: single_judge::MEMORY_LIMIT_KIB.to_string(),
    }))?;
    // 2nd codeblock of builder.ipynb
    let build_script = builder.add_script(Text {
        name: "build_script".to_string(),
        content: NJ_BUILD_SCRIPT.to_string(),
    })?;
    let run_script = builder.add_script(Text {
        name: "run_script".to_string(),
        content: NJ_RUN_SCRIPT.to_string(),
    })?;
    let summarize_script = builder.add_script(Text {
        name: "summarize_script".to_string(),
        content: NJ_SUMMARIZE_SCRIPT.to_string(),
    })?;
    // 3rd codeblock of builder.ipynb
    let build_tempdir = builder.add_resource(ResourceKind::EmptyDirectory(EmptyDirectory {
        name: "build_tempdir".to_string(),
    }))?;
    let build_result = builder.add_execution(Execution {
        name: job_name::COMPILE_PHASE.to_string(),
        script_name: build_script.clone(),
        dependencies: vec![
            Dependency {
                ref_to: source.clone(),
                envvar_name: "BUILD_SOURCE_PATH".to_string(),
            },
            Dependency {
                ref_to: lang_tag.clone(),
                envvar_name: "LANGUAGE_TAG".to_string(),
            },
            Dependency {
                ref_to: build_tempdir.clone(),
                envvar_name: "BUILD_TEMPDIR".to_string(),
            },
        ],
        time_reserved_ms: COMPILE_PHASE_TIME_RESERVED_MS as u64,
    })?;
    // 4th codeblock of builder.ipynb
    let mut test_results = Vec::new();
    for testcase in testcases.iter() {
        let input_file = builder.add_resource(ResourceKind::TextFile(Text {
            name: job_name::v0_features::testcase_input_name(&testcase.name),
            content: testcase.input.clone(),
        }))?;
        let expected_file = builder.add_resource(ResourceKind::TextFile(Text {
            name: job_name::v0_features::testcase_expected_name(&testcase.name),
            content: testcase.expected_output.clone(),
        }))?;
        let tempdir = builder.add_resource(ResourceKind::EmptyDirectory(EmptyDirectory {
            name: format!("tempdir_{}", testcase.name),
        }))?;
        let test_result = builder.add_execution(Execution {
            name: job_name::test_phase_execution_job_name(&testcase.name),
            script_name: run_script.clone(),
            dependencies: vec![
                Dependency {
                    ref_to: lang_tag.clone(),
                    envvar_name: "LANGUAGE_TAG".to_string(),
                },
                Dependency {
                    ref_to: time_limit.clone(),
                    envvar_name: "TIME_LIMIT_MS".to_string(),
                },
                Dependency {
                    ref_to: memory_limit.clone(),
                    envvar_name: "MEMORY_LIMIT_KIB".to_string(),
                },
                Dependency {
                    ref_to: input_file.clone(),
                    envvar_name: "INPUT_FILE".to_string(),
                },
                Dependency {
                    ref_to: expected_file.clone(),
                    envvar_name: "EXPECTED_FILE".to_string(),
                },
                Dependency {
                    ref_to: tempdir.clone(),
                    envvar_name: "TEMP_DIR".to_string(),
                },
                Dependency {
                    ref_to: build_result.clone(),
                    envvar_name: "BUILD_OUTPUT_PATH".to_string(),
                },
                Dependency {
                    ref_to: source.clone(),
                    envvar_name: "BUILD_SOURCE_PATH".to_string(),
                },
            ],
            time_reserved_ms: TEST_PHASE_TIME_RESERVED_MS as u64,
        })?;
        test_results.push(test_result);
    }
    // 5th codeblock of builder.ipynb
    let testcase_count = builder.add_resource(ResourceKind::TextFile(Text {
        name: "testcase_count".to_string(),
        content: testcases.len().to_string(),
    }))?;
    let ac_point = builder.add_resource(ResourceKind::TextFile(Text {
        name: "ac_point".to_string(),
        content: 100.to_string(),
    }))?;
    let mut summarize_dependencies = vec![
        Dependency {
            ref_to: testcase_count.clone(),
            envvar_name: "TESTCASE_COUNT".to_string(),
        },
        Dependency {
            ref_to: ac_point.clone(),
            envvar_name: "AC_POINT".to_string(),
        },
    ];
    for i in 0..test_results.len() {
        summarize_dependencies.push(Dependency {
            ref_to: test_results[i].clone(),
            envvar_name: format!("OUTPUT_JSON_{}", i),
        });
    }
    let _summarize_result = builder.add_execution(Execution {
        name: job_name::SUMMARY_PHASE.to_string(),
        script_name: summarize_script.clone(),
        dependencies: summarize_dependencies,
        time_reserved_ms: SUMMARIZE_PHASE_TIME_RESERVED_MS as u64,
    })?;
    return Ok(builder.get_procedure());
}
