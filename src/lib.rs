use spin_sdk::http_wasip3::{IntoResponse, Request};

#[spin_sdk::http_wasip3::http_service]
async fn handle(request: Request) -> impl IntoResponse {
    for (name, value) in request.headers() {
        println!("HEADER: {name}={}", String::from_utf8_lossy(value.as_bytes()));
    }

    use spin_sdk::http_wasip3::body::IncomingBodyExt;

    let response = if let Ok(by) = request.into_body().bytes().await {
        println!("there are bytes");
        let text = String::from_utf8_lossy(&by);
        format!("-- INBOUND MESSAGE --\n{text}\n---------------------\n")
    } else {
        println!("there are... not bytes");
        "done\n".to_string()
    };

    response
}
