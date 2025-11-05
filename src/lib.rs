use spin_sdk::http_wasip3::{IntoResponse, Request};

#[spin_sdk::http_wasip3::http_service]
async fn handle(request: Request) -> impl IntoResponse {
    for (name, value) in request.headers() {
        println!("HEADER: {name}={}", String::from_utf8_lossy(value.as_bytes()));
    }

    use spin_sdk::http_wasip3::body::IncomingBodyExt;
    use futures::StreamExt;

    let (mut sw, sr) = spin_sdk::http_wasip3::wasip3::wit_stream::new();
    let (_tfw, tfr) = spin_sdk::http_wasip3::wasip3::wit_future::new(|| Ok(None));
    let (resp, _efr) = spin_sdk::http_wasip3::wasip3::http::types::Response::new(spin_sdk::http_wasip3::wasip3::http::types::Headers::new(), Some(sr), tfr);

    let mut ib = request.into_body().stream();

    spin_sdk::http_wasip3::wasip3::wit_bindgen::spawn(async move {
        sw.write_all("-- INBOUND MESSAGE --\n".into()).await;
        loop {
            let Some(chunk) = ib.next().await else {
                break;
            };
            let chunk = chunk.unwrap();
            sw.write_all(chunk.to_vec()).await;
        }
        sw.write_all("\n---------------------\n".into()).await;
    });

    resp
}
