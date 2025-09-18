wit_bindgen::generate!({
    path: "../wit",
    world: "spin:up/http-middleware@3.5.0",
    async: true,
    generate_all,
});

use wasi::http0_3_0_rc_2025_08_15::types::{Request, Response, ErrorCode, Fields};

struct Guest;

impl exports::wasi::http0_3_0_rc_2025_08_15::handler::Guest for Guest {
    #[allow(async_fn_in_trait)]
    async fn handle(request: Request) -> Result<Response,ErrorCode> {
        handle_middlybiddly(request).await
    }
}

export!(Guest);

async fn handle_middlybiddly(request: Request) -> Result<Response, ErrorCode> {
    // Requests are immutable so yes this is how it has to be done.
    let method = request.get_method().await;
    let (rx, trailers) =
        request.consume_body().await.unwrap();
        // Request::consume_body(request, wit_future::new(|| Ok(())).1);
    let headers = request.get_headers().await.clone().await;
    headers.append("my-fake-auth-header".to_string(), "IT WORKS, LOOK".as_bytes().to_vec()).await.unwrap();
    let outgoing_request =
        Request::new(headers, Some(rx), trailers, None).await.0;
    outgoing_request.set_method(method).await.unwrap();
    outgoing_request
        .set_path_with_query(request.get_path_with_query().await)
        .await
        .unwrap();
    outgoing_request
        .set_scheme(request.get_scheme().await)
        .await
        .unwrap();
    outgoing_request
        .set_authority(request.get_authority().await)
        .await
        .unwrap();
    spin::up3_5_0::next::handle(outgoing_request).await

    //    let m = req.get_method().await;
    // let h = req.get_headers().await; // .await.append("arse".to_string(), "biscuit".as_bytes().to_vec()).await.unwrap();
    // let (b, bfr) = req.consume_body().await.unwrap();
    // let opts = req.get_options().await;
    // let au = req.get_authority().await;

    // let (req2, fr) = Request::new(h, b, trailers, opts).await;

    // spin::up3_5_0::next::handle(req).await
    // for (name, value) in req.get_headers().await.copy_all().await {
    //     println!("HEADER: {name}={}", String::from_utf8_lossy(&value));
    // }

    // let (_wr, trailers) = wit_future::new(|| Ok(None));

    // let (resp, _fr) = Response::new(Fields::new().await, None, trailers).await;

    // Ok(resp)
}
