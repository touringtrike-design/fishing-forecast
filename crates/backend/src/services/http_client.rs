/// Build a configured HTTP client for external API calls.
pub fn build_http_client() -> anyhow::Result<reqwest::Client> {
    reqwest::Client::builder()
        .user_agent("fishing-forecast/1.0 (contact: dev@fishing-forecast.local)")
        .build()
        .map_err(|err| anyhow::anyhow!("failed to build http client: {err}"))
}
