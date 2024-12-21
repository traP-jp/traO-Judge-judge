use crate::custom_rc::{FileFactory, ReadonlyFile, WritableFile};
use crate::models::judge_recipe::{Execution, ExecutionConfigMap};
use anyhow::{Context, Result};
use std::collections::HashMap;
use uuid::Uuid;

pub async fn prepare_files<
    ReadonlyFileType: ReadonlyFile,
    WritableFileType: WritableFile<ReadonlyFileType>,
    FileFactoryType: FileFactory<WritableFileType, ReadonlyFileType>,
>(
    file_factory: &FileFactoryType,
    execution: &Execution,
    execution_config: &ExecutionConfigMap,
) -> Result<(
    HashMap<Uuid, ReadonlyFileType>,
    HashMap<Uuid, WritableFileType>,
    HashMap<Uuid, String>,
)> {
    let (text_files, onetime_text_files, directories, shellhook) = futures::join!(
        futures::future::join_all(
            execution_config
                .text_resource_ids
                .iter()
                .map(|text_resource_id| { file_factory.new_textfile(text_resource_id) })
        ),
        futures::future::join_all(execution_config.one_time_text_contents.iter().map(
            |onetime_text_resource| { file_factory.new_textfile_from_raw(onetime_text_resource) }
        )),
        futures::future::join_all(
            (0..execution.directory_count).map(|_| { file_factory.new_directory() })
        ),
        file_factory.new_textfile(&execution.shell_script_id)
    );

    // Error handling
    let text_files = text_files
        .into_iter()
        .enumerate()
        .map(|(i, file)| {
            let name = format!("TEXT_{}", i).to_string();
            let file = file.with_context(|| format!("Failed to create text file {}", i))?;
            Ok((name, file))
        })
        .collect::<Result<HashMap<_, _>>>()?;
    let one_time_text_files = onetime_text_files
        .into_iter()
        .enumerate()
        .map(|(i, file)| {
            let name = format!("ONETIME_TEXT_{}", i).to_string();
            let file = file.with_context(|| format!("Failed to create onetime text file {}", i))?;
            Ok((name, file))
        })
        .collect::<Result<HashMap<_, _>>>()?;
    let directories = directories
        .into_iter()
        .enumerate()
        .map(|(i, file)| {
            let name = format!("DIR_{}", i).to_string();
            let file = file.with_context(|| format!("Failed to create directory {}", i))?;
            Ok((name, file))
        })
        .collect::<Result<HashMap<_, _>>>()?;
    let shellhook = shellhook.with_context(|| "Failed to create shellhook")?;

    let mut all_readonly_files = text_files;
    let mut all_writable_files = one_time_text_files;
    all_writable_files.extend(directories);
    all_readonly_files.insert("SHELLHOOK".to_string(), shellhook);

    // set uuids
    let mut all_readonly_files_uuid = HashMap::new();
    let mut all_writable_files_uuid = HashMap::new();
    let mut filename_dict = HashMap::new();
    for (name, file) in all_readonly_files {
        let uuid = Uuid::new_v4();
        all_readonly_files_uuid.insert(uuid, file);
        filename_dict.insert(uuid, name);
    }
    for (name, file) in all_writable_files {
        let uuid = Uuid::new_v4();
        all_writable_files_uuid.insert(uuid, file);
        filename_dict.insert(uuid, name);
    }
    Ok((
        all_readonly_files_uuid,
        all_writable_files_uuid,
        filename_dict,
    ))
}
