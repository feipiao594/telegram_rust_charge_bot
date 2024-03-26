pub async fn url_get(url: &String) -> String {
    surf::get(url).recv_string().await.unwrap()
}
