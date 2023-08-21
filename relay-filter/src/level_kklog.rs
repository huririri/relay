use relay_general::protocol::Event;

// use kk_log;
use relay_general::protocol::Level;
use relay_general::types::Annotated;

use crate::FilterStatKey;

// 过滤kklog类型的error，只输出到relay控制台，不上报到sentry
pub fn should_filter(event: &Event) -> Result<(), FilterStatKey> {
    if event.level == Annotated::new(Level::KKlog) {
        // 打印 logentry下的message
        if let Some(logentry) = event.logentry.value() {
            if let Some(message) = logentry.formatted.value() {
                println!("【kk-log】-message: {:?}", message);
            } else if let Some(message) = logentry.message.value() {
                println!("【kk-log】-message: {:?}", message);
            }
        }
        return Err(FilterStatKey::KKlog);
    } else {
        println!("ok");
    }

    Ok(())
}
