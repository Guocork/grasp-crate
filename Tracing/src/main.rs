use tracing::{span, Level};
use tracing::{debug, error, info, warn};
use tracing_subscriber;
use tracing::{info_span, Info};

fn main() {

    // 初始化 tracing_subscriber 以记录日志
    tracing_subscriber::fmt::init();
    // 创建并进入一个名为 `my_span` 的 span，该 span 的等级为 INFO。
    let my_span = span!(Level::INFO, "my_span");
    // 返回值 _enter 是一个用于管理 span 生命周期的对象。只要这个对象在作用域内，span 就是活动的。
    let _enter = my_span.enter();

    // 代码区间的执行

    // 当 `_enter` 离开作用域时，span 自动关闭。
}

// 在 span 内，我们可以记录 events。以下是如何做到这一点的示例。
fn perform_operation() {
    info!("performing operation");
    // 操作代码

    if let Err(e) = some_operation_that_may_fail() {
        error!("failed to perform operation: {}", e);
    }
}

fn some_operation_that_may_fail() -> Result<(), &'static str> {
    Err("Operation failed")
}

async fn handle_request(request: Request) -> Result<Response, Error> {
    let request_span = info_span!("handle_request", %request);
    let _enter = request_span.enter();

    // 处理请求...

    Ok(response)
}
