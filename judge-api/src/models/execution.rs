/*
 * Judge to Backend API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Execution : テストケース実行時の実行時間など
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Execution {
    /// 実行時間
    #[serde(rename = "execTime")]
    pub exec_time: f64,
    /// メモリ使用量(byte)
    #[serde(rename = "memorySize")]
    pub memory_size: f64,
    /// ディレクトリのアクセス権限
    #[serde(rename = "directoryAccessibilities")]
    pub directory_accessibilities: Vec<models::FileAccessibility>,
    /// テキストリソースのアクセス権限
    #[serde(rename = "textResourceAccessibilities")]
    pub text_resource_accessibilities: Vec<models::FileAccessibility>,
    /// テキストデータのアクセス権限
    #[serde(rename = "textAccessibilities")]
    pub text_accessibilities: Vec<models::FileAccessibility>,
    /// シェルスクリプトのID
    #[serde(rename = "shellScriptId")]
    pub shell_script_id: uuid::Uuid,
}

impl Execution {
    /// テストケース実行時の実行時間など
    pub fn new(exec_time: f64, memory_size: f64, directory_accessibilities: Vec<models::FileAccessibility>, text_resource_accessibilities: Vec<models::FileAccessibility>, text_accessibilities: Vec<models::FileAccessibility>, shell_script_id: uuid::Uuid) -> Execution {
        Execution {
            exec_time,
            memory_size,
            directory_accessibilities,
            text_resource_accessibilities,
            text_accessibilities,
            shell_script_id,
        }
    }
}

