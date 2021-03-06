#[macro_use]
extern crate log;
extern crate stderrlog;

#[test]
fn warn_level() {
    stderrlog::new().module(module_path!()).verbosity(1).init().unwrap();

    error!("error msg");
    warn!("warning msg");
    info!("info msg");
    debug!("debug msg");
    trace!("trace msg");

    assert_eq!(log::LogLevel::Warn, log::max_log_level())
}
