pub async fn url_get(url: &str) -> String {
    surf::get(url).recv_string().await.unwrap()
}
