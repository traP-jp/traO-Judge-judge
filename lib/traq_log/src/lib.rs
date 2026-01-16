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
pub async fn send_message(content: String) -> anyhow::Result<()> {
    let webhook_id = std::env::var("TRAQ_WEBHOOK_ID")
        .map_err(|e| anyhow::anyhow!("TRAQ_WEBHOOK_ID env var missing: {}", e))?;
    let url = format!("https://q.trap.jp/api/v3/webhooks/{}", webhook_id);
    let client = reqwest::Client::new();

    let res = client
        .post(&url)
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(content)
        .send()
        .await?;

    if !res.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to send message to traQ webhook: HTTP {}",
            res.status()
        ));
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
///     send_info_message(Some("CREATE PROBLEM"), "This is an info message".to_string()).await?;
///     Ok(())
/// }
/// ```
/// traQ 上でのメッセージ:
/// ```md
/// # :information_source.large: CREATE PROBLEM
/// This is an info message
/// ```
pub async fn send_info_message(title: Option<String>, content: String) -> anyhow::Result<()> {
    let content = if let Some(t) = title {
        format!("# :information_source.large: {}\n{}", t, content)
    } else {
        format!("# :information_source.large: INFOMATION\n{}", content)
    };

    send_message(content).await?;

    Ok(())
}

/// webhook を使って traQ に警告メッセージを送信します。
/// ### 例
/// ```rust,no_run
/// use traq_log::send_warning_message;
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     send_warning_message(Some("WRITTER ERROR"), "This is a warning message".to_string()).await?;
///     Ok(())
/// }
/// ```
/// traQ 上でのメッセージ:
/// ```md
/// # :warning.large: WRITTER ERROR
/// This is a warning message
/// ```
pub async fn send_warning_message(title: Option<String>, content: String) -> anyhow::Result<()> {
    let content = if let Some(t) = title {
        format!("# :warning.large: {}\n{}", t, content)
    } else {
        format!("# :warning.large: WARNING\n{}", content)
    };

    send_message(content).await?;

    Ok(())
}

/// webhook を使って traQ にエラーメッセージを送信します。
/// ### 例
/// ```rust,no_run
/// use traq_log::send_error_message;
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     send_error_message(Some("INTERNAL ERROR"), "This is an error message".to_string()).await?;
///     Ok(())
/// }
/// ```
/// traQ 上でのメッセージ:
/// ```md
/// # :fire.large: INTERNAL ERROR
/// This is an error message
/// ```
pub async fn send_error_message(title: Option<String>, content: String) -> anyhow::Result<()> {
    let content = if let Some(t) = title {
        format!("# :fire.large: {}\n{}", t, content)
    } else {
        format!("# :fire.large: ERROR\n{}", content)
    };

    send_message(content).await?;

    Ok(())
}
