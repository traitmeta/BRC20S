use anyhow::Context;
use log4rs::{
  append::{
    console::ConsoleAppender,
    rolling_file::{
      policy::compound::{
        roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
      },
      RollingFileAppender,
    },
  },
  config::{Appender, Logger, Root},
  encode::pattern::PatternEncoder,
  Config,
};
use std::path::Path;

pub fn init<P: AsRef<Path>>(
  level: log::LevelFilter,
  log_file: P,
) -> anyhow::Result<log4rs::Handle> {
  let stdout = ConsoleAppender::builder().build();

  // using default encoder for now, change it as needed.
  let encoder = PatternEncoder::default();
  let trigger = SizeTrigger::new(1024 * 1024 * 20);
  let roller = FixedWindowRoller::builder()
    .build("{}-{}.gz", 50)
    .context("build FixedWindowRoller with '{}-{}.gz' failed.")?;
  let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));
  let rfile = RollingFileAppender::builder()
    .append(true)
    .encoder(Box::new(encoder))
    .build(log_file.as_ref(), Box::new(policy))
    .with_context(|| {
      format!(
        "Failed to create rolling file {}",
        log_file.as_ref().display()
      )
    })?;

  let cfg = Config::builder()
    .appender(Appender::builder().build("stdout", Box::new(stdout)))
    .appender(Appender::builder().build("rfile", Box::new(rfile)))
    .logger(Logger::builder().build("mio", log::LevelFilter::Error))
    .build(
      Root::builder()
        .appender("stdout")
        .appender("rfile")
        .build(level),
    )
    .context("build log config failed")?;

  Ok(log4rs::init_config(cfg).context("log4rs init config error")?)
}
