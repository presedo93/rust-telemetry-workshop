use helpers::MockWriter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_log::LogTracer;

pub fn init_test_subscriber() -> MockWriter {
    let writer = MockWriter::new();
    let writer2 = writer.clone();
    tracing_subscriber::fmt()
        .with_writer(move || writer.clone())
        .with_span_events(FmtSpan::FULL)
        .compact()
        .init();

    // TODO: redirect `log` events to `tracing`!
    let _ = LogTracer::init();
    writer2
}
