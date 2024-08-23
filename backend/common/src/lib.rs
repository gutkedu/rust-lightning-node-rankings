pub mod dtos;
pub mod dynamo;
pub mod dynamo_utils;

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        // This needs to be set to remove duplicated information in the log.
        .with_current_span(false)
        // This needs to be set to false, otherwise ANSI color codes will
        // show up in a confusing manner in CloudWatch logs.
        .with_ansi(false)
        // Disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        // Remove the name of the function from every log entry.
        .with_target(false)
        .init();
}
