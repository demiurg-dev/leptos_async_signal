pub async fn fetch_data() -> (String, String) {
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    ("Hello world".to_string(), "42".to_string())
}

pub fn init_test() {
    // Set async executor
    any_spawner::Executor::init_tokio().unwrap();

    // This sets sandbox arena for reactive graph
    let owner = reactive_graph::owner::Owner::new();
    owner.set();
}
