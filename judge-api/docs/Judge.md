# Judge

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**judge_id** | [**uuid::Uuid**](uuid::Uuid.md) | ジャッジのID | 
**directory_count** | **f64** | ディレクトリの数 | 
**before_test_execs** | [**Vec<models::Execution>**](Execution.md) | テストケース実行前に一度だけ実行されるコマンド | 
**on_test_execs** | [**Vec<models::Execution>**](Execution.md) | テストケース実行時に実行されるコマンド | 
**after_test_execs** | [**Vec<models::Execution>**](Execution.md) | テストケース実行後に一度だけ実行されるコマンド | 
**text_resource_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | 静的なテキストリソースのID | 
**one_timetexts** | **Vec<String>** | 動的に変化するテキストデータ | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


