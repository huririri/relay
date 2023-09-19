use crate::FilterStatKey;
use relay_general::protocol::Event;
use relay_general::protocol::Level;
use relay_general::types::Annotated;
/// If the event's level is `Level::KKlog`, this function will print the message from the `logentry`
/// field and return an error to indicate that the event should be filtered. Otherwise, it will
/// print "ok" and return `Ok(())`.
///
/// # Arguments
///
/// * `event` - The event to check.
///
/// # Returns
///
/// * `Ok(())` if the event should not be filtered.
/// * `Err(FilterStatKey::KKlog)` if the event should be filtered.
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
    }
    Ok(())
}
