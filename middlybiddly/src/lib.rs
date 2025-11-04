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
    let request = munge(request);
    spin::up3_5_0::next::handle(request.into_request().unwrap()).await
}

fn munge(request: Request) -> Request {
    let (mut parts, body) = request.into_parts();

    parts.headers.append("my-fake-auth-header", http::HeaderValue::from_static("HOLY COW IT WORKS"));
    // mmm, forbidden header
    parts.headers.remove("connection");
    parts.headers.remove("host");

    Request::from_parts(parts, body)
}

// fn munge(request: Request) -> Request {
//     let (mut parts, body) = request.into_parts();

//     parts.headers.append("my-fake-auth-header", http::HeaderValue::from_static("HOLY COW IT WORKS"));
//     // mmm, forbidden header
//     parts.headers.remove("connection");
//     parts.headers.remove("host");

//     // use http_body_util::BodyExt;

//     // println!("map_frame");
//     // let bod2 = body.map_frame(|f| { println!("{:?}", f.data_ref().map(|d| d.len())); f });
//     // println!("into_inner");
//     // let bod3 = bod2.into_inner();
//     // println!("from_parts");
//     // let rr = Request::from_parts(parts, bod3);
//     // println!("ret");
//     // rr

//     Request::from_parts(parts, body)
// }
