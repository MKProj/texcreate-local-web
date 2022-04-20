use texc_web::texweb;

#[tokio::main]
async fn main() {
    texweb().launch().await.unwrap()
}
