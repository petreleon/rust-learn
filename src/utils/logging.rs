use std::io::Write;

pub fn init_logging(service: &'static str) {
    let env = env_logger::Env::default().filter_or("RUST_LOG", "info");
    let mut builder = env_logger::Builder::from_env(env);

    builder.format(move |buf, record| {
        let timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        writeln!(
            buf,
            "ts={} level={} service={} target={} {}",
            timestamp,
            record.level(),
            service,
            record.target(),
            record.args()
        )
    });

    let _ = builder.try_init();
}
