use crate::config::global_config;

pub(crate) async fn init_log() -> anyhow::Result<()> {
    let config = global_config();
    let path = std::path::Path::new(&config.log.file);
    let parent = path.parent().unwrap();
    let file_name_prefix = path.file_name().unwrap().to_str().unwrap();
    let file_appender = tracing_appender::rolling::daily(parent, file_name_prefix);
    // let (non_blocking_writer, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .json()
        .flatten_event(true)
        .with_max_level(config.log.level)
        .with_span_list(false)
        .with_target(false)
        .with_line_number(true)
        .with_file(true)
        .with_writer(file_appender)
        .init();
    Ok(())
}
