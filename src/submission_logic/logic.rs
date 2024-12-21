use anyhow::{Result, Context};


pub struct Logic<
    'a,

    ContainerType: crate::container::Container,
    ExternalAccessKeyType: Eq + std::hash::Hash + Clone + ToString,
    ContainerFactoryType: crate::container::ContainerFactory<ContainerType, SingleRunPriorityType>,
    
    ReadonlyFileType: crate::custom_rc::ReadonlyFile,
    WriteableFileType: crate::custom_rc::WriteableFile<ReadonlyFileType>,
    ReadonlyFileLinkType: crate::custom_rc::FileLink<'a, ReadonlyFileType>,
    WriteableFileLinkType: crate::custom_rc::FileLink<'a, WriteableFileType>,
    
    RepoType: crate::text_resource_repository::TextResourceRepository<ExternalAccessKeyType>,
    FileFactoryType: crate::custom_rc::FileFactory<
        WriteableFileType,
        ReadonlyFileType,
        ExternalAccessKeyType,
    >,

    SingleRunPriorityType: Ord + Clone + From<(
        crate::models::judge_recipe::SubmissionInput,
        super::models::Phase
    )> = super::heuristics::priority::JudgePriority,
> {
    container_factory: ContainerFactoryType,
    file_factory: FileFactoryType,
    _phantom: std::marker::PhantomData<(
        &'a (),
        ContainerType,
        SingleRunPriorityType,
        ExternalAccessKeyType,
        ReadonlyFileType,
        WriteableFileType,
        ReadonlyFileLinkType,
        WriteableFileLinkType,
        RepoType,
    )>,
}

impl <
    'a,

    ContainerType: crate::container::Container,
    ExternalAccessKeyType: Eq + std::hash::Hash + Clone + ToString,
    ContainerFactoryType: crate::container::ContainerFactory<ContainerType, SingleRunPriorityType>,
    
    ReadonlyFileType: crate::custom_rc::ReadonlyFile,
    WriteableFileType: crate::custom_rc::WriteableFile<ReadonlyFileType>,
    ReadonlyFileLinkType: crate::custom_rc::FileLink<'a, ReadonlyFileType>,
    WriteableFileLinkType: crate::custom_rc::FileLink<'a, WriteableFileType>,
    
    RepoType: crate::text_resource_repository::TextResourceRepository<ExternalAccessKeyType>,
    FileFactoryType: crate::custom_rc::FileFactory<
        WriteableFileType,
        ReadonlyFileType,
        ExternalAccessKeyType,
    >,

    SingleRunPriorityType: Ord + Clone + From<(
        crate::models::judge_recipe::SubmissionInput,
        super::models::Phase
    )>,
> Logic<
    'a,

    ContainerType,
    ExternalAccessKeyType,
    ContainerFactoryType,
    ReadonlyFileType,
    WriteableFileType,
    ReadonlyFileLinkType,
    WriteableFileLinkType,
    RepoType,
    FileFactoryType,

    SingleRunPriorityType,
> {
    pub fn new(
        container_factory: ContainerFactoryType,
        file_factory: FileFactoryType,
    ) -> Self {
        Self {
            container_factory,
            file_factory,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl <
    'a,

    ContainerType: crate::container::Container,
    ExternalAccessKeyType: Eq + std::hash::Hash + Clone + ToString,
    ContainerFactoryType: crate::container::ContainerFactory<ContainerType, SingleRunPriorityType>,

    ReadonlyFileType: crate::custom_rc::ReadonlyFile,
    WriteableFileType: crate::custom_rc::WriteableFile<ReadonlyFileType>,
    ReadonlyFileLinkType: crate::custom_rc::FileLink<'a, ReadonlyFileType>,
    WriteableFileLinkType: crate::custom_rc::FileLink<'a, WriteableFileType>,

    RepoType: crate::text_resource_repository::TextResourceRepository<ExternalAccessKeyType>,
    FileFactoryType: crate::custom_rc::FileFactory<
        WriteableFileType,
        ReadonlyFileType,
        ExternalAccessKeyType,
    >,

    SingleRunPriorityType: Ord + Clone + From<(
        crate::models::judge_recipe::SubmissionInput,
        super::models::Phase
    )>,
> super::Logic<
    ContainerType,
> for Logic<
    'a,

    ContainerType,
    ExternalAccessKeyType,
    ContainerFactoryType,
    ReadonlyFileType,
    WriteableFileType,
    ReadonlyFileLinkType,
    WriteableFileLinkType,
    RepoType,
    FileFactoryType,

    SingleRunPriorityType,
> {
    async fn judge(
        &self,
        input: crate::models::judge_recipe::SubmissionInput,
        connection_time_limit: std::time::Duration,
        execution_time_limit: std::time::Duration,
    ) -> Result<crate::models::judge_result::SubmissionOutput> {
        // container rx acquisition
        let before_test_container_rx = self.container_factory
            .get_rx(
                SingleRunPriorityType::from((
                    input.clone(),
                    super::models::Phase::BeforeTest
                ))
            )
            .await
            .context("Failed to receive container")?;
        let test_container_rxs = futures::future::join_all(
            (0..input.test_count)
                .map(|_| self.container_factory.get_rx(
                    SingleRunPriorityType::from((
                        input.clone(),
                        super::models::Phase::Test
                    ))
                ))
        )
            .await
            .into_iter()
            .collect::<Result<Vec<_>>>()
            .context("Failed to receive container")?;
        let after_test_container_rx = self.container_factory
            .get_rx(
                SingleRunPriorityType::from((
                    input.clone(),
                    super::models::Phase::AfterTest
                ))
            )
            .await
            .context("Failed to receive container")?;
        
        
        unimplemented!()
    }
}
