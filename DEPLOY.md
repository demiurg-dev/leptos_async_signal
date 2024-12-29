# Deploy instructions for `crates.io`

- Obtain the API token
- `cargo login`
- `cargo publish -p leptos_async_signal -F ssr`
  - Note: can add `--dry-run` flag to test the deployment.