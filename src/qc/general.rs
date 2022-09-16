use std::{fs::File, io::Write, path::PathBuf};

use noodles_bam::lazy::Record;

use self::metrics::{GeneralMetricsFacet, SummaryMetrics};

use super::{ComputationalLoad, Error, QualityCheckFacet};

pub mod metrics;

impl QualityCheckFacet for GeneralMetricsFacet {
    fn name(&self) -> &'static str {
        "General Metrics"
    }

    fn computational_load(&self) -> ComputationalLoad {
        ComputationalLoad::Light
    }

    fn process(&mut self, record: &Record) -> Result<(), Error> {
        // (1) Count the number of reads in the file.
        self.records.total += 1;

        // (2) Compute metrics related to flags.
        if let Ok(s) = record.flags() {
            if s.is_duplicate() {
                self.records.duplicate += 1;
            }

            if s.is_unmapped() {
                self.records.unmapped += 1;
            }

            if s.is_secondary() {
                self.records.designation.secondary += 1;
            } else if s.is_supplementary() {
                self.records.designation.supplementary += 1;
            } else {
                self.records.designation.primary += 1;
            }
        }

        Ok(())
    }

    fn summarize(&mut self) -> Result<(), super::Error> {
        let summary = SummaryMetrics {
            duplication_pct: self.records.duplicate as f64 / self.records.total as f64 * 100.0,
            unmapped_pct: self.records.unmapped as f64 / self.records.total as f64 * 100.0,
        };

        self.summary = Some(summary);

        Ok(())
    }

    fn write(
        &self,
        output_prefix: String,
        directory: &std::path::Path,
    ) -> Result<(), std::io::Error> {
        let metrics_filename = output_prefix + ".summary.json";
        let mut metrics_filepath = PathBuf::from(directory);
        metrics_filepath.push(metrics_filename);

        let mut file = File::create(metrics_filepath).unwrap();
        let output = serde_json::to_string_pretty(&self).unwrap();
        file.write_all(output.as_bytes())?;

        Ok(())
    }
}
