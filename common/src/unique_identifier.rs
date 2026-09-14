use std::fs;
use std::process;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static FALLBACK_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn new_uuid() -> String {
    read_linux_uuid().unwrap_or_else(fallback_uuid)
}

fn read_linux_uuid() -> Option<String> {
    let value = fs::read_to_string("/proc/sys/kernel/random/uuid").ok()?;
    let value = value.trim();

    if is_uuid(value) {
        Some(value.to_string())
    } else {
        None
    }
}

fn fallback_uuid() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    let pid = u128::from(process::id());
    let counter = u128::from(FALLBACK_COUNTER.fetch_add(1, Ordering::Relaxed));
    let value = nanos ^ (pid << 64) ^ counter;
    let source = format!("{value:032x}");

    format!(
        "{}-{}-{}-{}-{}",
        &source[0..8],
        &source[8..12],
        &source[12..16],
        &source[16..20],
        &source[20..32]
    )
}

fn is_uuid(value: &str) -> bool {
    value.len() == 36
        && value.chars().enumerate().all(|(index, character)| {
            matches!(index, 8 | 13 | 18 | 23) == (character == '-')
                && (character == '-' || character.is_ascii_hexdigit())
        })
}
