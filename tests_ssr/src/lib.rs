pub async fn fetch_data() -> (String, String) {
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    ("Hello world".to_string(), "42".to_string())
}
