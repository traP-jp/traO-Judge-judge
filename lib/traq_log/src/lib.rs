use std::sync::LazyLock;

static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);
static WEBHOOK_URL: LazyLock<Option<String>> = LazyLock::new(|| {
    std::env::var("TRAQ_WEBHOOK_ID")
        .ok()
        .map(|id| format!("https://q.trap.jp/api/v3/webhooks/{}", id))
});

fn get_webhook_url() -> anyhow::Result<&'static str> {
    WEBHOOK_URL
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("TRAQ_WEBHOOK_ID env var missing"))
}

/// webhook を使って traQ にメッセージを送信します。
/// ### 例
/// ```rust,ignore
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
pub async fn send_message(content: &str) -> anyhow::Result<()> {
    let url = get_webhook_url()?;

    let res = HTTP_CLIENT
        .post(url)
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(content.to_string())
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
/// ```rust,ignore
/// use traq_log::send_info_message;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     send_info_message(Some("CREATE PROBLEM"), "This is an info message").await?;
///     Ok(())
/// }
/// ```
/// traQ 上でのメッセージ:
/// ```md
/// # :information_source.large: CREATE PROBLEM
/// This is an info message
/// ```
pub async fn send_info_message(title: Option<&str>, content: &str) -> anyhow::Result<()> {
    let content = if let Some(t) = title {
        format!("# :information_source.large: {}\n{}", t, content)
    } else {
        format!("# :information_source.large: INFORMATION\n{}", content)
    };

    send_message(&content).await?;

    Ok(())
}

/// webhook を使って traQ に警告メッセージを送信します。
/// ### 例
/// ```rust,ignore
/// use traq_log::send_warning_message;
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     send_warning_message(Some("WRITTER ERROR"), "This is a warning message").await?;
///     Ok(())
/// }
/// ```
/// traQ 上でのメッセージ:
/// ```md
/// # :warning.large: WRITTER ERROR
/// This is a warning message
/// ```
pub async fn send_warning_message(title: Option<&str>, content: &str) -> anyhow::Result<()> {
    let content = if let Some(t) = title {
        format!("# :warning.large: {}\n{}", t, content)
    } else {
        format!("# :warning.large: WARNING\n{}", content)
    };

    send_message(&content).await?;

    Ok(())
}

/// webhook を使って traQ にエラーメッセージを送信します。
/// ### 例
/// ```rust,ignore
/// use traq_log::send_error_message;
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     send_error_message(Some("INTERNAL ERROR"), "This is an error message").await?;
///     Ok(())
/// }
/// ```
/// traQ 上でのメッセージ:
/// ```md
/// # :fire.large: INTERNAL ERROR
/// This is an error message
/// ```
pub async fn send_error_message(title: Option<&str>, content: &str) -> anyhow::Result<()> {
    let content = if let Some(t) = title {
        format!("# :fire.large: {}\n{}", t, content)
    } else {
        format!("# :fire.large: ERROR\n{}", content)
    };

    send_message(&content).await?;

    Ok(())
}
