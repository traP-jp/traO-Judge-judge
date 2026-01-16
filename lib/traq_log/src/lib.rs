/// webhook を使って traQ にメッセージを送信します。
/// ### 例
/// ```rust,no_run
/// use traq_log::send_message;
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     send_message("This is a test message".to_string()).await?;
///     Ok(())
/// }
/// ```
/// traQ 上でのメッセージ:
/// ```md
/// This is a test message
/// ```
pub async fn send_message(
    content: String
) -> anyhow::Result<()> {
    let webhook_id = std::env::var("TRAQ_WEBHOOK_ID").map_err(|e| anyhow::anyhow!("TRAQ_WEBHOOK_ID env var missing: {}", e))?;
    let url = format!("https://q.trap.jp/api/v3/webhooks/{}", webhook_id);
    let client = reqwest::Client::new();
    
    let res = client.post(&url)
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(content)
        .send()
        .await?;

    if !res.status().is_success() {
        return Err(anyhow::anyhow!("Failed to send message to traQ webhook: HTTP {}", res.status()));
    }

    Ok(())
}

/// webhook を使って traQ に情報メッセージを送信します。
/// ### 例
/// ```rust,no_run
/// use traq_log::send_info_message;
/// 
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     send_info_message("This is an info message".to_string()).await?;
///     Ok(())
/// }
/// ``` 
/// traQ 上でのメッセージ:
/// ```md
/// # :information_source.large: INFOMATION
/// This is an info message
/// ``````
pub async fn send_info_message(
    content: String,
) -> anyhow::Result<()> {
    let content = format!("# :information_source.large: INFOMATION\n{}", content);

    send_message(content).await?;

    Ok(())
}

/// webhook を使って traQ に警告メッセージを送信します。
/// ### 例
/// ```rust,no_run
/// use traq_log::send_warning_message;
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     send_warning_message("This is a warning message".to_string()).await?;
///     Ok(())
/// }
/// ```
/// traQ 上でのメッセージ:
/// ```md
/// # :warning.large: WARNING
/// This is a warning message
/// ```
pub async fn send_warning_message(
    content: String,
) -> anyhow::Result<()> {
    let content = format!("# :warning.large: WARNING\n{}", content);

    send_message(content).await?;

    Ok(())
}

/// webhook を使って traQ にエラーメッセージを送信します。
/// ### 例
/// ```rust,no_run
/// use traq_log::send_error_message;
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     send_error_message("This is an error message".to_string()).await?;
///     Ok(())
/// }
/// ```
/// traQ 上でのメッセージ:
/// ```md
/// # :fire.large: ERROR
/// This is an error message
/// ```
pub async fn send_error_message(
    content: String,
) -> anyhow::Result<()> {
    let content = format!("# :fire.large: ERROR\n{}", content);

    send_message(content).await?;

    Ok(())
}