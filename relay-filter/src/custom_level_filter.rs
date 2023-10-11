//! Implements event filtering for events with a custom level.
use std::collections::HashMap;
use crate::FilterStatKey;
use relay_general::protocol::Event;
use relay_general::protocol::Level;
use relay_general::protocol::User;
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
pub fn should_filter(event: &Event, project_id: &u64) -> Result<(), FilterStatKey> {
    if event.level == Annotated::new(Level::CustomLevelForFilter) {
        print_result(event, project_id);
        return Err(FilterStatKey::CustomFilterLevel);
    }
    Ok(())
}

pub fn print_result(event: &Event, project_id: &u64) {

    // 获取用户信息
    let mut result_user_id = "-";
    if let Some(user) = event.user.value() {
        result_user_id = user.id.as_str().unwrap_or("-");
    }

    // 上报信息
    let mut result_message = "-";
    if let Some(logentry) = event.logentry.value() {
        if let Some(message) = logentry.formatted.value() {
            result_message = message.as_ref();
        } else if let Some(message) = logentry.message.value() {
            result_message = message.as_ref();
        }
    }

    // 项目信息
    let mut result_url = "-";
    let mut result_user_agent = "-";
    if let Some(request) = event.request.value() {
        result_url = request.url.as_str().unwrap_or("-");
        if let Some(headers) = request.headers.value() {
            result_user_agent = headers.get_header("User-Agent").unwrap_or("-")
        }
    }
    let result_project_id :&str = &project_id.to_string();

    let mut result = HashMap::new();
    result.insert("message", result_message);
    result.insert("project_id", result_project_id);
    result.insert("h5_url", result_url);
    result.insert("user_id", result_user_id);
    result.insert("user-agent", result_user_agent);

    println!("【custom-log-filter】, {:?}", result);
}
