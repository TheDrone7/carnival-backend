pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}\n",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Error)
        .level_for("backend", log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("graphql.log")?)
        .apply()?;
    Ok(())
}
