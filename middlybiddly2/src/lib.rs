wit_bindgen::generate!({
    path: "../wit",
    world: "wasi:http/middleware@0.3.0-rc-2026-01-06",
    async: true,
    with: {
        "wasi:http/types@0.3.0-rc-2026-01-06": spin_sdk::http_wasip3::wasip3::http::types,
    },
    generate_all,
});

use spin_sdk::http_wasip3::{IntoRequest, IntoResponse, Request};

#[spin_sdk::http_wasip3::http_service]
async fn handle(mut request: Request) -> impl IntoResponse {
    munge(&mut request);
    wasi::http0_3_0_rc_2026_01_06::handler::handle(request.into_request().unwrap()).await
}

fn munge(request: &mut Request) {
    request.headers_mut().append("honk", http::HeaderValue::from_static("HONK! HONK! HONK!"));
    // mmm, forbidden header
    request.headers_mut().remove("connection");
    request.headers_mut().remove("host");
}
