use spin_sdk::http_wasip3::{IntoResponse, Request};

#[spin_sdk::http_wasip3::http_service]
async fn handle(request: Request) -> impl IntoResponse {
    for (name, value) in request.headers() {
        println!("HEADER: {name}={}", String::from_utf8_lossy(value.as_bytes()));
    }

    "done\n"
}
