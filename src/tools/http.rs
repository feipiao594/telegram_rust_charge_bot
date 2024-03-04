use reqwest::Result;

pub async fn url_get(url: &String) -> Result<String> {
    let body = reqwest::get(url).await?.text().await?;
    // println!("{}", body);
    Ok(body)
}
