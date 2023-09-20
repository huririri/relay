//! Implements event filtering for events with a custom level.

use crate::FilterStatKey;
use relay_general::protocol::Event;
use relay_general::protocol::Level;
use relay_general::types::Annotated;
/// If the event's level is `Level::Custom`, this function will print the message from the `logentry`
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
/// * `Err(FilterStatKey::CustomFilterLevel)` if the event should be filtered.
pub fn should_filter(event: &Event) -> Result<(), FilterStatKey> {
    if event.level == Annotated::new(Level::CustomLevelForFilter) {
        if let Some(logentry) = event.logentry.value() {
            if let Some(message) = logentry.formatted.value() {
                println!("【custom-log-filter】: {:?}", message);
            } else if let Some(message) = logentry.message.value() {
                println!("【custom-log-filter】: {:?}", message);
            }
        }
        return Err(FilterStatKey::CustomFilterLevel);
    }
    Ok(())
}
