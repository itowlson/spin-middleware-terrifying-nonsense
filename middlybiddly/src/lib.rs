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
    let (request, stm_write_fut) = munge(request);
    let send_fut = spin::up3_5_0::next::handle(request.into_request().unwrap());
    let (resp, _arse) = futures::future::join(send_fut, stm_write_fut).await;
    resp
}

fn munge(request: Request) -> (Request, impl Future<Output = ()>) {
    let (mut parts, body) = request.into_parts();

    parts.headers.append("my-fake-auth-header", http::HeaderValue::from_static("HOLY COW IT WORKS"));
    // mmm, forbidden header
    parts.headers.remove("connection");
    parts.headers.remove("host");

    // use spin_sdk::http_wasip3::body::IncomingBodyExt;
    use http_body_util::BodyExt;

    // let mut boddo = body.map_frame(|f| {
    //     if let Some(data) = f.data_ref() {
    //         let s = String::from_utf8_lossy(data);
    //         http_body::Frame::new(s.to_uppercase().as_bytes())
    //     } else {
    //         f
    //     }
    // }).collect();

    // let (_tfw, tfr) = spin_sdk::http_wasip3::wasip3::wit_future::new(|| Ok(None));
    // let (req2, _) = spin_sdk::http_wasip3::wasip3::http::types::Request::new(
    //     spin_sdk::http_wasip3::wasip3::http::types::Headers::new(),
    //     Some(sr),
    //     tfr,
    //     None
    // );
    // let body2 = spin_sdk::http_wasip3::wasip3::http_compat::IncomingRequestBody::new(req2).unwrap();
    // todo!()

    let mut boddo = body.map_frame(|f| f);

    let (_tfw, tfr) = spin_sdk::http_wasip3::wasip3::wit_future::new(|| Ok(None));
    let (mut sw, sr) = spin_sdk::http_wasip3::wasip3::wit_stream::new();
    let (req2, _) = spin_sdk::http_wasip3::wasip3::http::types::Request::new(
        spin_sdk::http_wasip3::wasip3::http::types::Headers::new(),
        Some(sr),
        tfr,
        None
    );
    let body2 = spin_sdk::http_wasip3::wasip3::http_compat::IncomingRequestBody::new(req2).unwrap();

    let fut = async move {
        loop {
            let Some(f) = boddo.frame().await else {
                println!("map_frame done");
                break;
            };
            println!("mapped a frame, ok={}", f.is_ok());
            let f = f.unwrap();
            let Some(data) = f.data_ref() else {
                println!("map_frame is past the data");
                break;
            };
            let s = String::from_utf8_lossy(data);
            println!("**orig** {s:?}");
            let supper = s.to_uppercase();
            println!("**uppo** {supper}");
            sw.write_all(supper.into()).await;
            // sw.write_all("BISCOTTI".into()).await;
        }
    };

    (Request::from_parts(parts, body2), fut)
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
