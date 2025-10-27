use hyper::{
    Body, Request, Response, Server,
    body::to_bytes,
    service::{make_service_fn, service_fn},
};
use serde::Serialize;
use std::{collections::HashMap, net::SocketAddr};

#[derive(Serialize)]
struct RequestInfo {
    host: Option<String>,
    remote_addr: Option<String>,
    url: String,
    request_uri: String,
    proto: String,
    method: String,
    headers: HashMap<String, String>,
    content_length: usize,
    body: String,
}

async fn handle(
    req: Request<Body>,
    remote_addr: Option<SocketAddr>,
) -> Result<Response<Body>, hyper::Error> {
    // 提前提取 request 元数据（不消耗 req）
    let uri = req.uri().clone();
    let method = req.method().clone();
    let version = req.version();
    let host = req
        .headers()
        .get("host")
        .map(|h| h.to_str().unwrap_or("").to_string());

    let mut headers_map = HashMap::new();
    for (k, v) in req.headers().iter() {
        headers_map.insert(k.to_string(), v.to_str().unwrap_or("").to_string());
    }

    // request_uri = path + query
    let mut request_uri = uri.path().to_string();
    if let Some(q) = uri.query() {
        request_uri.push('?');
        request_uri.push_str(q);
    }

    // 消耗 body
    let whole_body = to_bytes(req.into_body()).await?;
    let body_str = String::from_utf8_lossy(&whole_body).to_string();

    let info = RequestInfo {
        host,
        remote_addr: remote_addr.map(|a| a.to_string()),
        url: uri.to_string(),
        request_uri,
        proto: format!("{:?}", version),
        method: method.to_string(),
        headers: headers_map,
        content_length: whole_body.len(),
        body: body_str,
    };

    let json = serde_json::to_string(&info).unwrap_or_else(|e| format!(r#"{{"error":"{}"}}"#, e));

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::from(json))
        .unwrap()) // 在实际运行时几乎不会触发 panic。零风险的 unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8085));

    let make_svc = make_service_fn(|conn: &hyper::server::conn::AddrStream| {
        let remote_addr = Some(conn.remote_addr());
        async move { Ok::<_, hyper::Error>(service_fn(move |req| handle(req, remote_addr))) }
    });

    println!("Listening on http://{}", addr);
    Server::bind(&addr).serve(make_svc).await?;

    Ok(())
}
