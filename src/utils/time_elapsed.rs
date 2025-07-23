use std::time::Duration;

pub fn format_elapsed(elapsed: Duration) -> String {
    let secs = elapsed.as_secs();
    let nanos = elapsed.subsec_nanos();

    let seconds = secs;
    let millis = nanos / 1_000_000;
    let micros = (nanos % 1_000_000) / 1_000;

    let mut parts = Vec::new();

    if seconds > 0 {
        parts.push(format!("{}s", seconds));
    }
    if millis > 0 {
        parts.push(format!("{}ms", millis));
    }
    if micros > 0 && millis == 0 {
        parts.push(format!("{}µs", micros));
    }

    if parts.is_empty() {
        parts.push("0µs".to_string());
    }

    parts.join(" : ")
}