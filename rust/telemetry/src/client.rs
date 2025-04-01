use crate::events::ProductTelemetryEvent;
use async_trait::async_trait;

#[async_trait]
pub trait TelemetryClient {
    async fn capture(&mut self, event: Box<dyn ProductTelemetryEvent + Send + Sync>);
}
