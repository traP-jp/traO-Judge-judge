use super::extra_envs::get_extra_envs;
use super::file_preparation::prepare_files;
use super::output_parser::parse_output;
use crate::models::judge_result::*;
use anyhow::{Context, Result};

pub struct Logic<
    ContainerType: crate::container::Container,
    ContainerFactoryType: crate::container::ContainerFactory<ContainerType, SingleRunPriorityType>,
    ReadonlyFileType: crate::custom_rc::ReadonlyFile,
    WriteableFileType: crate::custom_rc::WriteableFile<ReadonlyFileType>,
    ReadonlyFileLinkType: crate::custom_rc::FileLink<ReadonlyFileType>,
    WriteableFileLinkType: crate::custom_rc::FileLink<WriteableFileType>,
    RepoType: crate::text_resource_repository::TextResourceRepository,
    FileFactoryType: crate::custom_rc::FileFactory<
        WriteableFileType,
        ReadonlyFileType,
    >,

    SingleRunPriorityType: Ord + Clone + From<(
        crate::models::judge_recipe::SubmissionInput,
        super::models::Phase
    )> = super::heuristics::priority::JudgePriority,
> {
    container_factory: ContainerFactoryType,
    file_factory: FileFactoryType,
    _phantom: std::marker::PhantomData<(
        ContainerType,
        SingleRunPriorityType,
        ReadonlyFileType,
        WriteableFileType,
        ReadonlyFileLinkType,
        WriteableFileLinkType,
        RepoType,
    )>,
}

impl<
        ContainerType: crate::container::Container,
        ContainerFactoryType: crate::container::ContainerFactory<ContainerType, SingleRunPriorityType>,
        ReadonlyFileType: crate::custom_rc::ReadonlyFile,
        WriteableFileType: crate::custom_rc::WriteableFile<ReadonlyFileType>,
        ReadonlyFileLinkType: crate::custom_rc::FileLink<ReadonlyFileType>,
        WriteableFileLinkType: crate::custom_rc::FileLink<WriteableFileType>,
        RepoType: crate::text_resource_repository::TextResourceRepository,
        FileFactoryType: crate::custom_rc::FileFactory<WriteableFileType, ReadonlyFileType>,
        SingleRunPriorityType: Ord
            + Clone
            + From<(
                crate::models::judge_recipe::SubmissionInput,
                super::models::Phase,
            )>,
    >
    Logic<
        ContainerType,
        ContainerFactoryType,
        ReadonlyFileType,
        WriteableFileType,
        ReadonlyFileLinkType,
        WriteableFileLinkType,
        RepoType,
        FileFactoryType,
        SingleRunPriorityType,
    >
{
    pub fn new(container_factory: ContainerFactoryType, file_factory: FileFactoryType) -> Self {
        Self {
            container_factory,
            file_factory,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<
        ContainerType: crate::container::Container,
        ContainerFactoryType: crate::container::ContainerFactory<ContainerType, SingleRunPriorityType>,
        ReadonlyFileType: crate::custom_rc::ReadonlyFile,
        WriteableFileType: crate::custom_rc::WriteableFile<ReadonlyFileType>,
        ReadonlyFileLinkType: crate::custom_rc::FileLink<ReadonlyFileType>,
        WriteableFileLinkType: crate::custom_rc::FileLink<WriteableFileType>,
        RepoType: crate::text_resource_repository::TextResourceRepository,
        FileFactoryType: crate::custom_rc::FileFactory<WriteableFileType, ReadonlyFileType>,
        SingleRunPriorityType: Ord
            + Clone
            + From<(
                crate::models::judge_recipe::SubmissionInput,
                super::models::Phase,
            )>,
    > super::Logic<ContainerType>
    for Logic<
        ContainerType,
        ContainerFactoryType,
        ReadonlyFileType,
        WriteableFileType,
        ReadonlyFileLinkType,
        WriteableFileLinkType,
        RepoType,
        FileFactoryType,
        SingleRunPriorityType,
    >
{
    async fn judge(
        &self,
        input: crate::models::judge_recipe::SubmissionInput,
        connection_time_limit: std::time::Duration,
        execution_time_limit: std::time::Duration,
    ) -> Result<crate::models::judge_result::SubmissionOutput> {
        // container rx acquisition
        let before_test_container_rx = self
            .container_factory
            .get_rx(SingleRunPriorityType::from((
                input.clone(),
                super::models::Phase::BeforeTest,
            )))
            .await
            .context("Failed to receive container")?;
        let test_container_rxs = futures::future::join_all((0..input.test_count).map(|_| {
            self.container_factory.get_rx(SingleRunPriorityType::from((
                input.clone(),
                super::models::Phase::Test,
            )))
        }))
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()
        .context("Failed to receive container")?;
        let after_test_container_rx = self
            .container_factory
            .get_rx(SingleRunPriorityType::from((
                input.clone(),
                super::models::Phase::AfterTest,
            )))
            .await
            .context("Failed to receive container")?;

        // prepare before test files
        let (before_test_readonly_files, before_test_writeable_files, before_test_filename_dict) =
            prepare_files::<ReadonlyFileType, WriteableFileType, FileFactoryType>(
                &self.file_factory,
                &input.before_test_execs,
                &input.before_test_config_map,
            )
            .await
            .context("Failed to prepare files")?;

        // execute before test and prepare test files
        let (before_test, test_files) = futures::join!(
            super::single_run::single_run::<
                ContainerType,
                ReadonlyFileType,
                WriteableFileType,
                ReadonlyFileLinkType,
                WriteableFileLinkType,
            >(
                "sh $SHELLHOOK",
                get_extra_envs(&input.before_test_execs.optional_info),
                connection_time_limit,
                execution_time_limit,
                before_test_container_rx,
                before_test_filename_dict.clone(),
                before_test_writeable_files,
                before_test_readonly_files,
            ),
            futures::future::join_all((0..input.test_count).map(|i| {
                prepare_files::<ReadonlyFileType, WriteableFileType, FileFactoryType>(
                    &self.file_factory,
                    &input.on_test_execs,
                    &input.on_test_config_maps[i as usize],
                )
            }))
        );

        // handle before test result
        let (before_test_output, before_test_readonly_files) =
            before_test.context("Failed to execute before test")?;
        let before_test_result =
            parse_output(&before_test_output).context("Failed to parse before test output")?;
        if let JudgeStatus::Critical(_) = before_test_result.status {
            return Ok(SubmissionOutput {
                judge_id: input.judge_id,
                result: JudgeResult::BeforeTestFailure(before_test_result),
            });
        }

        // prepare test files
        let test_files = test_files
            .into_iter()
            .collect::<Result<Vec<_>>>()
            .context("Failed to prepare test files")?;

        // execute tests and prepare after test files
        let (zipped_test_results, after_test_files) = futures::join!(
            futures::future::join_all(
                (0..input.test_count)
                    .zip(test_files)
                    .zip(test_container_rxs)
                    .map(|((_, test_files), container_rx)| {
                        let (mut readonly_files, writeable_files, mut filename_dict) = test_files;
                        readonly_files.extend(before_test_readonly_files.clone());
                        filename_dict.extend(before_test_filename_dict.clone().into_iter().map(
                            |(uuid, filename)| (uuid, "BEFORE_TEST_".to_string() + &filename),
                        ));
                        futures::future::join(
                            super::single_run::single_run::<
                                ContainerType,
                                ReadonlyFileType,
                                WriteableFileType,
                                ReadonlyFileLinkType,
                                WriteableFileLinkType,
                            >(
                                "sh $SHELLHOOK",
                                get_extra_envs(&input.on_test_execs.optional_info),
                                connection_time_limit,
                                execution_time_limit,
                                container_rx,
                                filename_dict.clone(),
                                writeable_files,
                                readonly_files,
                            ),
                            async move { filename_dict },
                        )
                    })
            ),
            prepare_files::<ReadonlyFileType, WriteableFileType, FileFactoryType>(
                &self.file_factory,
                &input.after_test_execs,
                &input.after_test_config_map
            )
        );

        // handle test results
        let (test_results, test_filename_dicts) = zipped_test_results
            .into_iter()
            .unzip::<_, _, Vec<_>, Vec<_>>();
        let (on_test_results, on_test_readonly_files) = test_results
            .into_iter()
            .collect::<Result<Vec<_>>>()
            .context("Failed to execute tests")?
            .into_iter()
            .map(|(execution_output, readonly_files)| {
                (
                    parse_output(&execution_output).context("Failed to parse test output"),
                    readonly_files,
                )
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();
        let on_test_results = on_test_results
            .into_iter()
            .collect::<Result<Vec<_>>>()
            .context("Failed to parse test output")?;
        if on_test_results.iter().any(|result| {
            if let JudgeStatus::Critical(_) = result.status {
                true
            } else {
                false
            }
        }) {
            return Ok(SubmissionOutput {
                judge_id: input.judge_id,
                result: JudgeResult::OnTestFailure(before_test_result, on_test_results),
            });
        }

        // prepare after test files
        let (
            mut after_test_readonly_files,
            after_test_writeable_files,
            mut after_test_filename_dict,
        ) = after_test_files.context("Failed to prepare after test files")?;

        after_test_readonly_files.extend(before_test_readonly_files.clone());
        after_test_filename_dict.extend(
            before_test_filename_dict
                .clone()
                .into_iter()
                .map(|(uuid, filename)| (uuid, "BEFORE_TEST_".to_string() + &filename)),
        );
        after_test_readonly_files.extend(
            on_test_readonly_files
                .into_iter()
                .map(|hashmap| hashmap.into_iter())
                .flatten(),
        );
        after_test_filename_dict.extend(
            test_filename_dicts
                .into_iter()
                .enumerate()
                .map(|(i, filename_dict)| {
                    filename_dict
                        .into_iter()
                        .map(move |(uuid, filename)| (uuid, format!("TEST_{}_{}", i, filename)))
                })
                .flatten(),
        );

        // execute after test
        let (after_test_output, _) = super::single_run::single_run::<
            ContainerType,
            ReadonlyFileType,
            WriteableFileType,
            ReadonlyFileLinkType,
            WriteableFileLinkType,
        >(
            "sh $SHELLHOOK",
            get_extra_envs(&input.after_test_execs.optional_info),
            connection_time_limit,
            execution_time_limit,
            after_test_container_rx,
            after_test_filename_dict.clone(),
            after_test_writeable_files,
            after_test_readonly_files,
        )
        .await
        .context("Failed to execute after test")?;

        // handle after test result
        let after_test_result =
            parse_output(&after_test_output).context("Failed to parse after test output")?;
        if let JudgeStatus::Critical(_) = after_test_result.status {
            return Ok(SubmissionOutput {
                judge_id: input.judge_id,
                result: JudgeResult::AfterTestFailure(
                    before_test_result,
                    on_test_results,
                    after_test_result,
                ),
            });
        }
        Ok(SubmissionOutput {
            judge_id: input.judge_id,
            result: JudgeResult::Success(before_test_result, on_test_results, after_test_result),
        })
    }
}
