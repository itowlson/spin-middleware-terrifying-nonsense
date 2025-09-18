wit_bindgen::generate!({
    path: "./wit",
    world: "spin:up/http-trigger@3.5.0",
    async: true,
    generate_all,
});

use wasi::http0_3_0_rc_2025_08_15::types::{Request, Response, ErrorCode, Fields};

struct Guest;

impl exports::wasi::http0_3_0_rc_2025_08_15::handler::Guest for Guest {
    #[allow(async_fn_in_trait)]
    async fn handle(request: Request) -> Result<Response,ErrorCode> {
        handle_middleware_terrifying_nonsense(request).await
    }
}

export!(Guest);


async fn handle_middleware_terrifying_nonsense(req: Request) -> Result<Response, ErrorCode> {
    for (name, value) in req.get_headers().await.copy_all().await {
        println!("HEADER: {name}={}", String::from_utf8_lossy(&value));
    }

    let (_wr, trailers) = wit_future::new(|| Ok(None));

    let (resp, _fr) = Response::new(Fields::new().await, None, trailers).await;

    Ok(resp)
}
