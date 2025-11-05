wit_bindgen::generate!({
    path: "../wit",
    world: "spin:up/http-middleware@3.5.0",
    async: true,
    with: {
        "wasi:http/types@0.3.0-rc-2025-09-16": spin_sdk::http_wasip3::wasip3::http::types,
    },
    generate_all,
});

use spin_sdk::http_wasip3::{IntoRequest, IntoResponse, Request};

#[spin_sdk::http_wasip3::http_service]
async fn handle(request: Request) -> impl IntoResponse {
    let request = munge(request).await;
    spin::up3_5_0::next::handle(request.into_request().unwrap()).await
}

async fn munge(request: Request) -> http::Request<impl http_body::Body<Data = bytes::Bytes, Error = spin_sdk::http_wasip3::wasip3::http::handler::ErrorCode>> {
    let (mut parts, body) = request.into_parts();

    parts.headers.append("my-fake-auth-header", http::HeaderValue::from_static("HOLY COW IT WORKS"));
    // mmm, forbidden header
    parts.headers.remove("connection");
    parts.headers.remove("host");

    use http_body_util::BodyExt;

    let boddo = body.map_frame(|f| {
        // !!!DELIBERATE DELAY!!! This is to make the streaming more visible in curl
        std::thread::sleep(std::time::Duration::from_millis(60));
        if let Some(data) = f.data_ref() {
            println!("txing a frame");
            let s = String::from_utf8_lossy(data);
            http_body::Frame::data(s.to_uppercase().into())
        } else {
            f
        }
    });

    http::Request::from_parts(parts, boddo)
}
