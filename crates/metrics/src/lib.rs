use indexmap::IndexMap;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub duration: Duration,
}

#[derive(Default, Debug)]
pub struct Metrics {
    ongoing: IndexMap<String, Instant>,
    completed: IndexMap<String, Duration>,
}

impl Metrics {
    pub fn begin(&mut self, name: &str) {
        self.ongoing.insert(name.to_string(), Instant::now());
    }

    pub fn end(&mut self, name: &str) {
        if let Some(start_time) = self.ongoing.remove(name) {
            let duration = start_time.elapsed();
            self.completed.insert(name.to_string(), duration);
        }
    }

    #[allow(dead_code)]
    pub fn report(&self) -> String {
        format!(
            "{} | total {:?}",
            self.completed
                .iter()
                .map(|(name, duration)| format!("{} {:?}", name, duration))
                .collect::<Vec<_>>()
                .join(" | "),
            &self.total()
        )
    }

    pub fn total(&self) -> Duration {
        self.completed.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_tracking() {
        let mut metrics = Metrics::default();

        metrics.begin("z first");
        std::thread::sleep(Duration::from_millis(10));
        metrics.end("z first");

        metrics.begin("a second");
        std::thread::sleep(Duration::from_millis(20));
        metrics.end("a second");

        let report = metrics.report();
        println!("{}", report);
        assert!(report.contains("z first"));
        assert!(report.contains("a second"));
    }
}
