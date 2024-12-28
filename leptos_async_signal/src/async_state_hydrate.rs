#[derive(Default, Clone)]
pub(crate) struct AsyncState {}

impl AsyncState {
    #[inline]
    pub async fn wait(&self) {}

    #[inline]
    pub fn mark_ready(&self) {}
}
