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
    request.headers_mut().append("my-fake-auth-header", http::HeaderValue::from_static("HOLY COW IT WORKS"));
    // mmm, forbidden header
    request.headers_mut().remove("connection");
    request.headers_mut().remove("host");

    // wat? how does og modify request body? why does request body have headers?
    // let bod = request.body_mut();
    // if let Some(bodd) = bod.take_unstarted() {
    //     bodd.get_headers();
    // }
}


// wit_bindgen::generate!({
//     path: "../wit",
//     world: "spin:up/http-middleware@3.5.0",
//     async: true,
//     generate_all,
// });

// use wasi::http0_3_0_rc_2025_09_16::types::{Request, Response, ErrorCode};

// struct Guest;

// impl exports::wasi::http0_3_0_rc_2025_09_16::handler::Guest for Guest {
//     #[allow(async_fn_in_trait)]
//     async fn handle(request: Request) -> Result<Response,ErrorCode> {
//         handle_middlybiddly(request).await
//     }
// }

// export!(Guest);

// async fn handle_middlybiddly(request: Request) -> Result<Response, ErrorCode> {
//     // Requests are immutable so yes this is how it has to be done.
//     let method = request.get_method().await;
//     let scheme = request.get_scheme().await;
//     let authority = request.get_authority().await;

//     let pq = request.get_path_with_query().await;

//     let opts = request.get_options().await;

//     let headers = request.get_headers().await.clone().await;
//     headers.append("my-fake-auth-header".to_string(), "IT WORKS, LOOK".as_bytes().to_vec()).await.unwrap();

//     let (rx, trailers) =
//         // request.consume_body().await.unwrap();
//         Request::consume_body(request, wit_future::new(|| Ok(())).1).await;

//     let outgoing_request =
//         Request::new(headers, Some(rx), trailers, opts).await.0;

//     outgoing_request.set_method(method).await.unwrap();
//     outgoing_request
//         .set_path_with_query(pq)
//         .await
//         .unwrap();
//     outgoing_request
//         .set_scheme(scheme)
//         .await
//         .unwrap();
//     outgoing_request
//         .set_authority(authority)
//         .await
//         .unwrap();

//     spin::up3_5_0::next::handle(outgoing_request).await

//     //    let m = req.get_method().await;
//     // let h = req.get_headers().await; // .await.append("arse".to_string(), "biscuit".as_bytes().to_vec()).await.unwrap();
//     // let (b, bfr) = req.consume_body().await.unwrap();
//     // let opts = req.get_options().await;
//     // let au = req.get_authority().await;

//     // let (req2, fr) = Request::new(h, b, trailers, opts).await;

//     // spin::up3_5_0::next::handle(req).await
//     // for (name, value) in req.get_headers().await.copy_all().await {
//     //     println!("HEADER: {name}={}", String::from_utf8_lossy(&value));
//     // }

//     // let (_wr, trailers) = wit_future::new(|| Ok(None));

//     // let (resp, _fr) = Response::new(Fields::new().await, None, trailers).await;

//     // Ok(resp)
// }
