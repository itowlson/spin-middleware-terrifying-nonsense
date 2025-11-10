use spin_sdk::http_wasip3::{IntoResponse, Request};
use spin_sdk::http_wasip3::body::IncomingBodyExt;
use futures::{SinkExt, StreamExt};

#[spin_sdk::http_wasip3::http_service]
async fn handle(request: Request) -> impl IntoResponse {
    for (name, value) in request.headers() {
        println!("HEADER: {name}={}", String::from_utf8_lossy(value.as_bytes()));
    }

    let mut ib = request.into_body().stream();

    let (mut tx, body) = stream_body();
    let response = http::Response::new(body);

    spin_sdk::http_wasip3::wasip3::wit_bindgen::spawn(async move {
        tx.send("~~ INBOUND MESSAGE ~~\n".into()).await.unwrap();
        loop {
            let Some(chunk) = ib.next().await else {
                break;
            };
            tx.send(chunk.unwrap()).await.unwrap();
        }
        tx.send("\n---------------------\n".into()).await.unwrap();
    });

    response

    // --- or raw WASI ---

    // let (mut sw, sr) = spin_sdk::http_wasip3::wasip3::wit_stream::new();
    // let (_tfw, tfr) = spin_sdk::http_wasip3::wasip3::wit_future::new(|| Ok(None));
    // let (resp, _efr) = spin_sdk::http_wasip3::wasip3::http::types::Response::new(spin_sdk::http_wasip3::wasip3::http::types::Headers::new(), Some(sr), tfr);

    // let mut ib = request.into_body().stream();

    // spin_sdk::http_wasip3::wasip3::wit_bindgen::spawn(async move {
    //     sw.write_all("-- INBOUND MESSAGE --\n".into()).await;
    //     loop {
    //         let Some(chunk) = ib.next().await else {
    //             break;
    //         };
    //         let chunk = chunk.unwrap();
    //         sw.write_all(chunk.to_vec()).await;
    //     }
    //     sw.write_all("\n---------------------\n".into()).await;
    // });

    // resp
}

fn stream_body() -> (futures::channel::mpsc::Sender<bytes::Bytes>, impl http_body::Body<Data = bytes::Bytes, Error = anyhow::Error>) {
    let (tx, rx) = futures::channel::mpsc::channel::<bytes::Bytes>(100_000);
    let stm = rx.map(data_frame);
    let body = http_body_util::StreamBody::new(stm);
    (tx, body)
}

fn data_frame(value: impl Into<bytes::Bytes>) -> Result<http_body::Frame<bytes::Bytes>, anyhow::Error> {
    Ok(http_body::Frame::data(value.into()))
}
