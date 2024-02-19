use assyst_common::config::CONFIG;
use bytes::Bytes;

use crate::assyst::ThreadSafeAssyst;

pub async fn post_filer_upload(
    assyst: ThreadSafeAssyst,
    buf: Bytes,
    content_type: &str,
) -> Result<String, reqwest::Error> {
    assyst
        .reqwest_client
        .post(CONFIG.urls.cdn.clone())
        .header(reqwest::header::CONTENT_TYPE, content_type)
        .header(reqwest::header::AUTHORIZATION, CONFIG.authentication.cdn_token.clone())
        .body(buf)
        .send()
        .await?
        .text()
        .await
}
