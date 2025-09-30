use serde::{Deserialize, Serialize};
use twilight_model::id::{Id, marker::ApplicationMarker};

use tulpje_framework::Metadata;

pub mod color;
pub mod metrics;
pub mod shard_state;

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordEvent {
    pub meta: Metadata,
    pub payload: String,
}

impl DiscordEvent {
    pub fn new(shard: u32, payload: String) -> Self {
        Self {
            meta: Metadata::new(shard),
            payload,
        }
    }
}

#[expect(
    clippy::integer_division,
    reason = "we only care about the whole numbers, so truncating is fine"
)]
pub fn format_significant_duration(total_secs: u64) -> String {
    const SECS_IN_MIN: u64 = 60;
    const SECS_IN_HOUR: u64 = 60 * 60;
    const SECS_IN_DAY: u64 = 24 * 60 * 60;

    let days = total_secs / SECS_IN_DAY;
    let hours = (total_secs % SECS_IN_DAY) / SECS_IN_HOUR;
    let mins = (total_secs % SECS_IN_HOUR) / SECS_IN_MIN;
    let secs = total_secs % SECS_IN_MIN;

    if days > 0 {
        format!("{}d {}h", days, hours)
    } else if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else if mins > 0 {
        format!("{}m {}s", mins, secs)
    } else {
        format!("{}s", secs)
    }
}

pub fn is_pk_proxy(application_id: &Option<Id<ApplicationMarker>>) -> bool {
    application_id.is_some_and(|id| id.get() == 466378653216014359) // PluralKit Application ID
}

#[macro_export]
macro_rules! version {
    () => {
        match option_env!("TULPJE_VERSION_EXTRA") {
            Some(extra) => format!("{} ({})", env!("CARGO_PKG_VERSION"), extra),
            _ => String::from(env!("CARGO_PKG_VERSION")),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn format_significant_duration_test() {
        assert_eq!(format_significant_duration(2 * 86_400 + 4 * 3_600), "2d 4h");
        assert_eq!(format_significant_duration(5 * 3_600 + 5 * 60 + 5), "5h 5m");
        assert_eq!(format_significant_duration(20 * 60 + 1), "20m 1s");
        assert_eq!(format_significant_duration(0), "0s");
    }

    #[test]
    fn is_pk_proxy_test() {
        assert!(is_pk_proxy(&Some(Id::<ApplicationMarker>::new(
            466378653216014359
        ))));

        assert!(!is_pk_proxy(&Some(Id::<ApplicationMarker>::new(1))));
        assert!(!is_pk_proxy(&None));
    }
}
