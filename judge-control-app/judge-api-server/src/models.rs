#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

      


/// テストケース実行時の実行時間など



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Execution {
    #[serde(rename = "optionalInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub optional_info: Option<models::OptionalInfo>,

/// シェルスクリプトのID
    #[serde(rename = "shellScriptId")]
    pub shell_script_id: uuid::Uuid,

/// ディレクトリの数
    #[serde(rename = "directoryCount")]
    pub directory_count: f64,

/// テキストリソースの数
    #[serde(rename = "textResourceCount")]
    pub text_resource_count: f64,

/// テキストの数
    #[serde(rename = "oneTimeTextCount")]
    pub one_time_text_count: f64,

}


impl Execution {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(shell_script_id: uuid::Uuid, directory_count: f64, text_resource_count: f64, one_time_text_count: f64, ) -> Execution {
        Execution {
            optional_info: None,
            shell_script_id,
            directory_count,
            text_resource_count,
            one_time_text_count,
        }
    }
}

/// Converts the Execution value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Execution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping optionalInfo in query parameter serialization

            // Skipping shellScriptId in query parameter serialization


            Some("directoryCount".to_string()),
            Some(self.directory_count.to_string()),


            Some("textResourceCount".to_string()),
            Some(self.text_resource_count.to_string()),


            Some("oneTimeTextCount".to_string()),
            Some(self.one_time_text_count.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Execution value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Execution {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub optional_info: Vec<models::OptionalInfo>,
            pub shell_script_id: Vec<uuid::Uuid>,
            pub directory_count: Vec<f64>,
            pub text_resource_count: Vec<f64>,
            pub one_time_text_count: Vec<f64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Execution".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "optionalInfo" => intermediate_rep.optional_info.push(<models::OptionalInfo as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "shellScriptId" => intermediate_rep.shell_script_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "directoryCount" => intermediate_rep.directory_count.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "textResourceCount" => intermediate_rep.text_resource_count.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "oneTimeTextCount" => intermediate_rep.one_time_text_count.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Execution".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Execution {
            optional_info: intermediate_rep.optional_info.into_iter().next(),
            shell_script_id: intermediate_rep.shell_script_id.into_iter().next().ok_or_else(|| "shellScriptId missing in Execution".to_string())?,
            directory_count: intermediate_rep.directory_count.into_iter().next().ok_or_else(|| "directoryCount missing in Execution".to_string())?,
            text_resource_count: intermediate_rep.text_resource_count.into_iter().next().ok_or_else(|| "textResourceCount missing in Execution".to_string())?,
            one_time_text_count: intermediate_rep.one_time_text_count.into_iter().next().ok_or_else(|| "oneTimeTextCount missing in Execution".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Execution> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Execution>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Execution>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Execution - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Execution> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Execution as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Execution - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




/// ジャッジの設定



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ExecutionConfigMap {
/// 静的なテキストリソースのID
    #[serde(rename = "textResourceIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub text_resource_ids: Option<Vec<uuid::Uuid>>,

/// 動的に変化するテキストデータ
    #[serde(rename = "oneTimeTextContents")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub one_time_text_contents: Option<Vec<String>>,

}


impl ExecutionConfigMap {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> ExecutionConfigMap {
        ExecutionConfigMap {
            text_resource_ids: None,
            one_time_text_contents: None,
        }
    }
}

/// Converts the ExecutionConfigMap value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ExecutionConfigMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping textResourceIds in query parameter serialization


            self.one_time_text_contents.as_ref().map(|one_time_text_contents| {
                [
                    "oneTimeTextContents".to_string(),
                    one_time_text_contents.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ExecutionConfigMap value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ExecutionConfigMap {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub text_resource_ids: Vec<Vec<uuid::Uuid>>,
            pub one_time_text_contents: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ExecutionConfigMap".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "textResourceIds" => return std::result::Result::Err("Parsing a container in this style is not supported in ExecutionConfigMap".to_string()),
                    "oneTimeTextContents" => return std::result::Result::Err("Parsing a container in this style is not supported in ExecutionConfigMap".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ExecutionConfigMap".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ExecutionConfigMap {
            text_resource_ids: intermediate_rep.text_resource_ids.into_iter().next(),
            one_time_text_contents: intermediate_rep.one_time_text_contents.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ExecutionConfigMap> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ExecutionConfigMap>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ExecutionConfigMap>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ExecutionConfigMap - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ExecutionConfigMap> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ExecutionConfigMap as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ExecutionConfigMap - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




/// ジャッジの設定



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Judge {
/// ジャッジのID
    #[serde(rename = "judgeId")]
    pub judge_id: uuid::Uuid,

/// テストケースの数
    #[serde(rename = "testCount")]
    pub test_count: f64,

    #[serde(rename = "beforeTestExecs")]
    pub before_test_execs: models::Execution,

    #[serde(rename = "onTestExecs")]
    pub on_test_execs: models::Execution,

    #[serde(rename = "afterTestExecs")]
    pub after_test_execs: models::Execution,

    #[serde(rename = "beforeTestConfigMap")]
    pub before_test_config_map: models::ExecutionConfigMap,

/// テストケース実行時に実行されるコマンドの設定
    #[serde(rename = "onTestConfigMaps")]
    pub on_test_config_maps: Vec<models::ExecutionConfigMap>,

    #[serde(rename = "afterTestConfigMap")]
    pub after_test_config_map: models::ExecutionConfigMap,

}


impl Judge {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(judge_id: uuid::Uuid, test_count: f64, before_test_execs: models::Execution, on_test_execs: models::Execution, after_test_execs: models::Execution, before_test_config_map: models::ExecutionConfigMap, on_test_config_maps: Vec<models::ExecutionConfigMap>, after_test_config_map: models::ExecutionConfigMap, ) -> Judge {
        Judge {
            judge_id,
            test_count,
            before_test_execs,
            on_test_execs,
            after_test_execs,
            before_test_config_map,
            on_test_config_maps,
            after_test_config_map,
        }
    }
}

/// Converts the Judge value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Judge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping judgeId in query parameter serialization


            Some("testCount".to_string()),
            Some(self.test_count.to_string()),

            // Skipping beforeTestExecs in query parameter serialization

            // Skipping onTestExecs in query parameter serialization

            // Skipping afterTestExecs in query parameter serialization

            // Skipping beforeTestConfigMap in query parameter serialization

            // Skipping onTestConfigMaps in query parameter serialization

            // Skipping afterTestConfigMap in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Judge value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Judge {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub judge_id: Vec<uuid::Uuid>,
            pub test_count: Vec<f64>,
            pub before_test_execs: Vec<models::Execution>,
            pub on_test_execs: Vec<models::Execution>,
            pub after_test_execs: Vec<models::Execution>,
            pub before_test_config_map: Vec<models::ExecutionConfigMap>,
            pub on_test_config_maps: Vec<Vec<models::ExecutionConfigMap>>,
            pub after_test_config_map: Vec<models::ExecutionConfigMap>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Judge".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "judgeId" => intermediate_rep.judge_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "testCount" => intermediate_rep.test_count.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "beforeTestExecs" => intermediate_rep.before_test_execs.push(<models::Execution as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "onTestExecs" => intermediate_rep.on_test_execs.push(<models::Execution as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "afterTestExecs" => intermediate_rep.after_test_execs.push(<models::Execution as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "beforeTestConfigMap" => intermediate_rep.before_test_config_map.push(<models::ExecutionConfigMap as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "onTestConfigMaps" => return std::result::Result::Err("Parsing a container in this style is not supported in Judge".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "afterTestConfigMap" => intermediate_rep.after_test_config_map.push(<models::ExecutionConfigMap as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Judge".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Judge {
            judge_id: intermediate_rep.judge_id.into_iter().next().ok_or_else(|| "judgeId missing in Judge".to_string())?,
            test_count: intermediate_rep.test_count.into_iter().next().ok_or_else(|| "testCount missing in Judge".to_string())?,
            before_test_execs: intermediate_rep.before_test_execs.into_iter().next().ok_or_else(|| "beforeTestExecs missing in Judge".to_string())?,
            on_test_execs: intermediate_rep.on_test_execs.into_iter().next().ok_or_else(|| "onTestExecs missing in Judge".to_string())?,
            after_test_execs: intermediate_rep.after_test_execs.into_iter().next().ok_or_else(|| "afterTestExecs missing in Judge".to_string())?,
            before_test_config_map: intermediate_rep.before_test_config_map.into_iter().next().ok_or_else(|| "beforeTestConfigMap missing in Judge".to_string())?,
            on_test_config_maps: intermediate_rep.on_test_config_maps.into_iter().next().ok_or_else(|| "onTestConfigMaps missing in Judge".to_string())?,
            after_test_config_map: intermediate_rep.after_test_config_map.into_iter().next().ok_or_else(|| "afterTestConfigMap missing in Judge".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Judge> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Judge>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Judge>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Judge - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Judge> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Judge as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Judge - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




/// 任意の情報



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct OptionalInfo {
/// 実行時間
    #[serde(rename = "execTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exec_time: Option<f64>,

/// メモリ使用量(byte)
    #[serde(rename = "memorySize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub memory_size: Option<f64>,

/// 使用言語
    #[serde(rename = "language")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub language: Option<String>,

}


impl OptionalInfo {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> OptionalInfo {
        OptionalInfo {
            exec_time: None,
            memory_size: None,
            language: None,
        }
    }
}

/// Converts the OptionalInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for OptionalInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.exec_time.as_ref().map(|exec_time| {
                [
                    "execTime".to_string(),
                    exec_time.to_string(),
                ].join(",")
            }),


            self.memory_size.as_ref().map(|memory_size| {
                [
                    "memorySize".to_string(),
                    memory_size.to_string(),
                ].join(",")
            }),


            self.language.as_ref().map(|language| {
                [
                    "language".to_string(),
                    language.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a OptionalInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for OptionalInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub exec_time: Vec<f64>,
            pub memory_size: Vec<f64>,
            pub language: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing OptionalInfo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "execTime" => intermediate_rep.exec_time.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "memorySize" => intermediate_rep.memory_size.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "language" => intermediate_rep.language.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing OptionalInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(OptionalInfo {
            exec_time: intermediate_rep.exec_time.into_iter().next(),
            memory_size: intermediate_rep.memory_size.into_iter().next(),
            language: intermediate_rep.language.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<OptionalInfo> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<OptionalInfo>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<OptionalInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for OptionalInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<OptionalInfo> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <OptionalInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into OptionalInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}



