/// Part 1 — clamp `n` to `0..=max`. Assume `max >= 0`.
pub fn clamp_non_negative(n: i32, max: i32) -> i32 {
    if n < 0 {
        0
    } else if n > max {
        max
    } else {
        n
    }
}

/// Part 2 — rate limit: can we take one token?
pub trait RateLimit {
    fn tokens_remaining(&self) -> u32;
}

pub fn try_consume(r: &dyn RateLimit) -> Option<()> {
    if r.tokens_remaining() > 0 {
        Some(())
    } else {
        None
    }
}

/// Part 2 — collaborator that records “what was called” (manual mock / spy).
pub trait CallLogger {
    fn mark(&mut self, label: &str);
}

pub fn two_step_job(log: &mut dyn CallLogger) {
    log.mark("step_a");
    log.mark("step_b");
}

pub struct RecordingLogger {
    pub marks: Vec<String>,
}

impl RecordingLogger {
    pub fn new() -> Self {
        Self { marks: Vec::new() }
    }
}

impl CallLogger for RecordingLogger {
    fn mark(&mut self, label: &str) {
        self.marks.push(label.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_becomes_zero() {
        assert_eq!(clamp_non_negative(-10, 100), 0);
    }

    #[test]
    fn above_max_clamped() {
        assert_eq!(clamp_non_negative(500, 100), 100);
    }

    #[test]
    fn in_range_unchanged() {
        assert_eq!(clamp_non_negative(42, 100), 42);
    }

    #[test]
    fn zero_boundary() {
        assert_eq!(clamp_non_negative(0, 10), 0);
    }

    #[test]
    fn max_boundary() {
        assert_eq!(clamp_non_negative(10, 10), 10);
    }

    struct FixedTokens(u32);
    impl RateLimit for FixedTokens {
        fn tokens_remaining(&self) -> u32 {
            self.0
        }
    }

    #[test]
    fn try_consume_succeeds_when_tokens_left() {
        let limit = FixedTokens(3);
        assert_eq!(try_consume(&limit), Some(()));
    }

    #[test]
    fn try_consume_fails_when_empty() {
        let limit = FixedTokens(0);
        assert_eq!(try_consume(&limit), None);
    }

    #[test]
    fn two_step_job_records_order() {
        let mut log = RecordingLogger::new();
        two_step_job(&mut log);
        assert_eq!(log.marks, vec!["step_a", "step_b"]);
    }
}
