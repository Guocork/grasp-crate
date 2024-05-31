use env_logger::{Builder, Target};
use log::{debug, error, info};
use log::{Record, Level, Metadata, LevelFilter, SetLoggerError};

fn main() {

    // 初始化env_logger
    env_logger::init();

    debug!("this is a debug message");
    error!("this is printed by default");

    let x = 3 * 4;
    info!("the answer was: {}", x);


    // env_logger提供了Builder模式用于自定义配置，比如更改日志的输出目标。
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();
}


struct SimpleLogger;

impl log::Log for SimpleLogger {
    // 用于检查某个日志记录是否启用
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info // 只启用日志级别小于等于 Info 的日志记录（即 Trace, Debug, Info）
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {  // 首先检查日志记录是否启用
            println!("{} - {}", record.level(), record.args());  // 如果启用，就打印日志级别和日志消息
        }
    }

    // 刷新日志缓冲区
    fn flush(&self) {}
}

// 定义一个全局的 SimpleLogger 实例
static LOGGER: SimpleLogger = SimpleLogger;  

pub fn init_custom_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER) // 设置全局的日志记录器为 LOGGER
        .map(|()| log::set_max_level(LevelFilter::Info)) // 如果成功设置了日志记录器，接着设置最大日志级别为 Info。这意味着只有 Info 及以上级别的日志消息会被处理和打印。
}

// 日志级别（小到大）：Trace Debug Info Warn Error 更多详细参考：https://mikechen.cc/30576.html

