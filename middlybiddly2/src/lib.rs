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
async fn handle(mut request: Request) -> impl IntoResponse {
    munge(&mut request);
    spin::up3_5_0::next::handle(request.into_request().unwrap()).await
}

fn munge(request: &mut Request) {
    request.headers_mut().append("honk", http::HeaderValue::from_static("HONK! HONK! HONK!"));
    // mmm, forbidden header
    request.headers_mut().remove("connection");
    request.headers_mut().remove("host");
}
