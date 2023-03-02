use anyhow::{ensure, Result};
use hyper::client::HttpConnector;
use hyper::{body, Body, Client, Request, Response};
use hyper_tls::HttpsConnector;
use std::fmt;

use std::str;
pub async fn assert_status(url: &str, expected: u16) -> Result<()> {
    let resp = make_request("GET", url, "").await?;
    let status = resp.status();

    let response = body::to_bytes(resp.into_body()).await.unwrap().to_vec();
    let actual_body = str::from_utf8(&response).unwrap().to_string();

    // assert_eq!(status, expected, "{}", actual_body).map_err(anyhow::Error::msg)
    ensure!(
        status == expected,
        assert_failure_msg(
            "==".to_string(),
            &status,
            &expected,
            Some(format_args!("{}", "status code assertion failed"))
        )
    );

    Ok(())
}

pub async fn assert_http_response(
    url: &str,
    expected: u16,
    expected_headers: &[(&str, &str)],
    expected_body: Option<&str>,
) -> Result<()> {
    let res = make_request("GET", url, "").await?;

    let status = res.status();
    // assert_eq!(expected, status.as_u16(), "{}", "unexpected http status code")?;
    ensure!(
        expected == status.as_u16(),
        "{}",
        "unexpected http status code"
    );

    let headers = res.headers();
    for (k, v) in expected_headers {
        ensure!(
            &headers
                .get(k.to_string())
                .unwrap_or_else(|| panic!("cannot find header {}", k))
                .to_str()?
                == v,
            "{}",
            "expected header not found",
        );
    }

    if let Some(expected_body_str) = expected_body {
        let response = body::to_bytes(res.into_body()).await.unwrap().to_vec();
        let actual_body = str::from_utf8(&response).unwrap().to_string();
        ensure!(
            expected_body_str == actual_body,
            "{}",
            "body assertion failed"
        );
    }

    Ok(())
}

pub async fn create_request(method: &str, url: &str, body: &str) -> Result<Request<Body>> {
    let req = Request::builder()
        .method(method)
        .uri(url)
        .body(Body::from(body.to_string()))
        .expect("request builder");

    Ok(req)
}

pub fn create_client() -> Client<HttpsConnector<HttpConnector>> {
    let connector = HttpsConnector::new();
    Client::builder().build::<_, hyper::Body>(connector)
}

pub async fn make_request(method: &str, path: &str, body: &str) -> Result<Response<Body>> {
    let c = create_client();
    let req = create_request(method, path, body);

    let resp = c.request(req.await?).await.unwrap();
    Ok(resp)
}

fn assert_failure_msg<T, U>(
    op: String,
    left: &T,
    right: &U,
    args: Option<fmt::Arguments<'_>>,
) -> String
where
    T: fmt::Debug + ?Sized,
    U: fmt::Debug + ?Sized,
{
    match args {
        Some(args) => format!(
            r#"assertion failed: `(left {} right)`
  left: `{:?}`,
 right: `{:?}`: {}"#,
            op, left, right, args
        ),
        None => format!(
            r#"assertion failed: `(left {} right)`
  left: `{:?}`,
 right: `{:?}`"#,
            op, left, right,
        ),
    }
}
