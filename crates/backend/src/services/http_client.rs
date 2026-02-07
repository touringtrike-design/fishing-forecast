/// Build a configured HTTP client for external API calls.
pub fn build_http_client() -> anyhow::Result<reqwest::Client> {
    reqwest::Client::builder()
        .user_agent("fishing-forecast/1.0 (contact: dev@fishing-forecast.local)")
        .timeout(std::time::Duration::from_secs(30))
        .connect_timeout(std::time::Duration::from_secs(10))
        .pool_max_idle_per_host(10)
        .use_rustls_tls()
        .build()
        .map_err(|err| anyhow::anyhow!("failed to build http client: {err}"))
}
