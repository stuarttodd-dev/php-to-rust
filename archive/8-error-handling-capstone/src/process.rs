use crate::logging::{log_info, log_warn};
use crate::parse::{parse_value_as_i32, split_assignment};
use std::error::Error;

pub fn sum_integer_values(text: &str) -> i32 {
    log_info(&format!(
        "parsing config ({} raw lines)",
        text.lines().count()
    ));
    let mut sum = 0;
    for (i, line) in text.lines().enumerate() {
        let line_num = i + 1;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        match split_assignment(trimmed) {
            Ok((key, value)) => match parse_value_as_i32(&value) {
                Ok(n) => {
                    sum += n;
                    log_info(&format!(
                        "line {}: {}={} parsed as +{}, sum now {}",
                        line_num, key, value, n, sum
                    ));
                }
                Err(e) => {
                    log_warn(&format!("line {}: {}", line_num, e));
                    if let Some(src) = e.source() {
                        log_warn(&format!("         caused by: {}", src));
                    }
                }
            },
            Err(e) => log_warn(&format!(
                "line {}: {} (input: {:?})",
                line_num, e, trimmed
            )),
        }
    }
    log_info(&format!("finished: total sum = {}", sum));
    sum
}
