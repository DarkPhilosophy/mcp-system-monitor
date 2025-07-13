//! MCP System Monitor Demo Visualization
//!
//! This demo script tests the MCP project and generates visual charts/screenshots
//! of system monitoring data for demonstration purposes.

use anyhow::Result;
use chrono::{DateTime, Utc};
use plotters::coord::Shift;
use plotters::prelude::*;
use std::fs;

/// Demo configuration
#[derive(Debug)]
struct DemoConfig {
    /// Output directory for charts and screenshots
    output_dir: String,
    /// Chart dimensions
    width: u32,
    height: u32,
    /// Number of data points to collect
    data_points: usize,
    /// Interval between data collection (seconds)
    interval: u64,
}

impl Default for DemoConfig {
    fn default() -> Self {
        Self {
            output_dir: "demo_output".to_string(),
            width: 1200,
            height: 800,
            data_points: 10,
            interval: 2,
        }
    }
}

/// System metrics collector
struct MetricsCollector {
    config: DemoConfig,
    data: Vec<SystemSnapshot>,
}

#[derive(Debug, Clone)]
struct SystemSnapshot {
    timestamp: DateTime<Utc>,
    cpu_usage: f64,
    memory_usage: f64,
    disk_usage: f64,
    network_rx: u64,
    network_tx: u64,
    process_count: usize,
}

impl MetricsCollector {
    fn new(config: DemoConfig) -> Self {
        Self {
            config,
            data: Vec::new(),
        }
    }

    /// Collect system metrics
    async fn collect_metrics(&mut self) -> Result<()> {
        println!("🔍 Collecting system metrics...");

        // Create output directory
        fs::create_dir_all(&self.config.output_dir)?;

        for i in 0..self.config.data_points {
            println!(
                "📊 Collecting data point {}/{}",
                i + 1,
                self.config.data_points
            );

            // Simulate system metrics (in real scenario, this would come from MCP server)
            let snapshot = self.simulate_system_metrics().await?;
            self.data.push(snapshot);

            if i < self.config.data_points - 1 {
                tokio::time::sleep(tokio::time::Duration::from_secs(self.config.interval)).await;
            }
        }

        Ok(())
    }

    /// Simulate system metrics (replace with actual MCP server calls)
    async fn simulate_system_metrics(&self) -> Result<SystemSnapshot> {
        // In a real scenario, you would call the MCP server here
        // For demo purposes, we'll simulate realistic data

        let now = Utc::now();
        let base_time = now.timestamp() as f64;

        // Simulate realistic system metrics with some variation
        let cpu_usage = 20.0 + (base_time.sin() * 30.0) + (rand::random::<f64>() * 20.0);
        let memory_usage = 60.0 + (base_time.cos() * 15.0) + (rand::random::<f64>() * 10.0);
        let disk_usage = 45.0 + (rand::random::<f64>() * 5.0);

        let network_rx = 1000000 + (rand::random::<u64>() % 5000000);
        let network_tx = 500000 + (rand::random::<u64>() % 2000000);
        let process_count = 150 + (rand::random::<usize>() % 50);

        Ok(SystemSnapshot {
            timestamp: now,
            cpu_usage: cpu_usage.max(0.0).min(100.0),
            memory_usage: memory_usage.max(0.0).min(100.0),
            disk_usage: disk_usage.max(0.0).min(100.0),
            network_rx,
            network_tx,
            process_count,
        })
    }

    /// Generate CPU usage chart
    fn generate_cpu_chart(&self) -> Result<()> {
        println!("📈 Generating CPU usage chart...");

        let path = format!("{}/cpu_usage.png", self.config.output_dir);
        let root =
            BitMapBackend::new(&path, (self.config.width, self.config.height)).into_drawing_area();
        root.fill(&WHITE)?;

        let (min_time, max_time) = self.get_time_range();
        let (min_cpu, max_cpu) = self.get_cpu_range();

        let mut chart = ChartBuilder::on(&root)
            .caption("CPU Usage Over Time", ("sans-serif", 30))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(min_time..max_time, min_cpu..max_cpu)?;

        chart
            .configure_mesh()
            .x_desc("Time")
            .y_desc("CPU Usage (%)")
            .draw()?;

        chart.draw_series(LineSeries::new(
            self.data.iter().map(|d| (d.timestamp, d.cpu_usage)),
            RED.mix(0.8).stroke_width(3),
        ))?;

        chart.draw_series(AreaSeries::new(
            self.data.iter().map(|d| (d.timestamp, d.cpu_usage)),
            0.0,
            RED.mix(0.1).filled(),
        ))?;

        root.present()?;
        println!("✅ CPU chart saved to: {}", path);
        Ok(())
    }

    /// Generate memory usage chart
    fn generate_memory_chart(&self) -> Result<()> {
        println!("📈 Generating memory usage chart...");

        let path = format!("{}/memory_usage.png", self.config.output_dir);
        let root =
            BitMapBackend::new(&path, (self.config.width, self.config.height)).into_drawing_area();
        root.fill(&WHITE)?;

        let (min_time, max_time) = self.get_time_range();
        let (min_mem, max_mem) = self.get_memory_range();

        let mut chart = ChartBuilder::on(&root)
            .caption("Memory Usage Over Time", ("sans-serif", 30))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(min_time..max_time, min_mem..max_mem)?;

        chart
            .configure_mesh()
            .x_desc("Time")
            .y_desc("Memory Usage (%)")
            .draw()?;

        chart.draw_series(LineSeries::new(
            self.data.iter().map(|d| (d.timestamp, d.memory_usage)),
            BLUE.mix(0.8).stroke_width(3),
        ))?;

        chart.draw_series(AreaSeries::new(
            self.data.iter().map(|d| (d.timestamp, d.memory_usage)),
            0.0,
            BLUE.mix(0.1).filled(),
        ))?;

        root.present()?;
        println!("✅ Memory chart saved to: {}", path);
        Ok(())
    }

    /// Generate system overview dashboard
    fn generate_dashboard(&self) -> Result<()> {
        println!("📊 Generating system dashboard...");

        let path = format!("{}/system_dashboard.png", self.config.output_dir);
        let root =
            BitMapBackend::new(&path, (self.config.width, self.config.height)).into_drawing_area();
        root.fill(&WHITE)?;

        // Create subplots
        let areas = root.split_evenly((2, 2));

        // CPU Usage
        self.draw_subplot(&areas[0], "CPU Usage (%)", |d| d.cpu_usage, &RED)?;

        // Memory Usage
        self.draw_subplot(&areas[1], "Memory Usage (%)", |d| d.memory_usage, &BLUE)?;

        // Disk Usage
        self.draw_subplot(&areas[2], "Disk Usage (%)", |d| d.disk_usage, &GREEN)?;

        // Process Count
        self.draw_subplot(
            &areas[3],
            "Process Count",
            |d| d.process_count as f64,
            &RGBColor(128, 0, 128),
        )?;

        root.present()?;
        println!("✅ Dashboard saved to: {}", path);
        Ok(())
    }

    /// Generate a combined dashboard with all charts in one file
    fn generate_combined_dashboard(&self) -> Result<()> {
        println!("📊 Generating combined dashboard...");
        let path = format!("{}/dashboard_combined.png", self.config.output_dir);
        let width = 1600;
        let height = 1200;
        let root = BitMapBackend::new(&path, (width, height)).into_drawing_area();
        root.fill(&WHITE)?;

        // Draw main title
        root.titled("MCP System Monitor - Demo Dashboard", ("sans-serif", 40))?;

        // Split into 2x2 grid
        let areas = root.split_evenly((2, 2));

        // CPU Usage
        self.draw_subplot(&areas[0], "CPU Usage (%)", |d| d.cpu_usage, &RED)?;
        // Memory Usage
        self.draw_subplot(&areas[1], "Memory Usage (%)", |d| d.memory_usage, &BLUE)?;
        // Network Traffic (show RX only for clarity)
        self.draw_subplot(
            &areas[2],
            "Network RX (bytes)",
            |d| d.network_rx as f64,
            &GREEN,
        )?;
        // Process Count
        self.draw_subplot(
            &areas[3],
            "Process Count",
            |d| d.process_count as f64,
            &RGBColor(128, 0, 128),
        )?;

        root.present()?;
        println!("✅ Combined dashboard saved to: {}", path);
        Ok(())
    }

    /// Draw a subplot
    fn draw_subplot<F>(
        &self,
        area: &DrawingArea<BitMapBackend, Shift>,
        title: &str,
        value_fn: F,
        color: &RGBColor,
    ) -> Result<()>
    where
        F: Fn(&SystemSnapshot) -> f64,
    {
        let (min_time, max_time) = self.get_time_range();
        let values: Vec<f64> = self.data.iter().map(&value_fn).collect();
        let min_val = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let range = (max_val - min_val).max(1.0);

        let mut chart = ChartBuilder::on(area)
            .caption(title, ("sans-serif", 20))
            .margin(5)
            .build_cartesian_2d(
                min_time..max_time,
                (min_val - range * 0.1)..(max_val + range * 0.1),
            )?;

        chart.configure_mesh().draw()?;

        chart.draw_series(LineSeries::new(
            self.data.iter().map(|d| (d.timestamp, value_fn(d))),
            color.mix(0.8).stroke_width(2),
        ))?;

        Ok(())
    }

    /// Generate network traffic chart
    fn generate_network_chart(&self) -> Result<()> {
        println!("📈 Generating network traffic chart...");

        let path = format!("{}/network_traffic.png", self.config.output_dir);
        let root =
            BitMapBackend::new(&path, (self.config.width, self.config.height)).into_drawing_area();
        root.fill(&WHITE)?;

        let (min_time, max_time) = self.get_time_range();
        let max_network = self
            .data
            .iter()
            .map(|d| d.network_rx.max(d.network_tx))
            .max()
            .unwrap_or(1000000) as f64;

        let mut chart = ChartBuilder::on(&root)
            .caption("Network Traffic", ("sans-serif", 30))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(min_time..max_time, 0.0..max_network)?;

        chart
            .configure_mesh()
            .x_desc("Time")
            .y_desc("Bytes")
            .draw()?;

        // Received traffic
        chart
            .draw_series(LineSeries::new(
                self.data.iter().map(|d| (d.timestamp, d.network_rx as f64)),
                GREEN.mix(0.8).stroke_width(3),
            ))?
            .label("Received")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

        // Transmitted traffic
        chart
            .draw_series(LineSeries::new(
                self.data.iter().map(|d| (d.timestamp, d.network_tx as f64)),
                RGBColor(255, 165, 0).mix(0.8).stroke_width(3),
            ))?
            .label("Transmitted")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(255, 165, 0)));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;
        println!("✅ Network chart saved to: {}", path);
        Ok(())
    }

    /// Generate system summary report
    fn generate_summary_report(&self) -> Result<()> {
        println!("📋 Generating summary report...");

        let path = format!("{}/system_summary.txt", self.config.output_dir);
        let mut report = String::new();

        report.push_str("=== MCP System Monitor Demo Report ===\n\n");
        report.push_str(&format!("Generated: {}\n", Utc::now()));
        report.push_str(&format!("Data points collected: {}\n", self.data.len()));
        report.push_str(&format!(
            "Collection interval: {} seconds\n\n",
            self.config.interval
        ));

        // Calculate averages
        let avg_cpu = self.data.iter().map(|d| d.cpu_usage).sum::<f64>() / self.data.len() as f64;
        let avg_memory =
            self.data.iter().map(|d| d.memory_usage).sum::<f64>() / self.data.len() as f64;
        let avg_disk = self.data.iter().map(|d| d.disk_usage).sum::<f64>() / self.data.len() as f64;
        let avg_processes =
            self.data.iter().map(|d| d.process_count).sum::<usize>() / self.data.len();

        report.push_str("=== System Metrics Summary ===\n");
        report.push_str(&format!("Average CPU Usage: {:.1}%\n", avg_cpu));
        report.push_str(&format!("Average Memory Usage: {:.1}%\n", avg_memory));
        report.push_str(&format!("Average Disk Usage: {:.1}%\n", avg_disk));
        report.push_str(&format!("Average Process Count: {}\n", avg_processes));

        // Peak values
        let peak_cpu = self.data.iter().map(|d| d.cpu_usage).fold(0.0, f64::max);
        let peak_memory = self.data.iter().map(|d| d.memory_usage).fold(0.0, f64::max);

        report.push_str("\n=== Peak Values ===\n");
        report.push_str(&format!("Peak CPU Usage: {:.1}%\n", peak_cpu));
        report.push_str(&format!("Peak Memory Usage: {:.1}%\n", peak_memory));

        // Network summary
        let total_rx: u64 = self.data.iter().map(|d| d.network_rx).sum();
        let total_tx: u64 = self.data.iter().map(|d| d.network_tx).sum();

        report.push_str("\n=== Network Summary ===\n");
        report.push_str(&format!(
            "Total Received: {} bytes ({:.2} MB)\n",
            total_rx,
            total_rx as f64 / 1024.0 / 1024.0
        ));
        report.push_str(&format!(
            "Total Transmitted: {} bytes ({:.2} MB)\n",
            total_tx,
            total_tx as f64 / 1024.0 / 1024.0
        ));

        report.push_str("\n=== Generated Files ===\n");
        report.push_str("- cpu_usage.png: CPU usage over time\n");
        report.push_str("- memory_usage.png: Memory usage over time\n");
        report.push_str("- network_traffic.png: Network traffic analysis\n");
        report.push_str("- system_dashboard.png: System overview dashboard\n");

        fs::write(&path, report)?;
        println!("✅ Summary report saved to: {}", path);
        Ok(())
    }

    /// Helper methods for chart generation
    fn get_time_range(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        let min_time = self
            .data
            .iter()
            .map(|d| d.timestamp)
            .min()
            .unwrap_or(Utc::now());
        let max_time = self
            .data
            .iter()
            .map(|d| d.timestamp)
            .max()
            .unwrap_or(Utc::now());
        (min_time, max_time)
    }

    fn get_cpu_range(&self) -> (f64, f64) {
        let min_cpu = self
            .data
            .iter()
            .map(|d| d.cpu_usage)
            .fold(f64::INFINITY, f64::min);
        let max_cpu = self
            .data
            .iter()
            .map(|d| d.cpu_usage)
            .fold(f64::NEG_INFINITY, f64::max);
        (min_cpu.max(0.0), max_cpu.min(100.0))
    }

    fn get_memory_range(&self) -> (f64, f64) {
        let min_mem = self
            .data
            .iter()
            .map(|d| d.memory_usage)
            .fold(f64::INFINITY, f64::min);
        let max_mem = self
            .data
            .iter()
            .map(|d| d.memory_usage)
            .fold(f64::NEG_INFINITY, f64::max);
        (min_mem.max(0.0), max_mem.min(100.0))
    }

    /// Run the complete demo
    async fn run_demo(&mut self) -> Result<()> {
        println!("🚀 Starting MCP System Monitor Demo...");
        println!("📁 Output directory: {}", self.config.output_dir);

        // Collect metrics
        self.collect_metrics().await?;

        // Generate charts
        self.generate_cpu_chart()?;
        self.generate_memory_chart()?;
        self.generate_network_chart()?;
        self.generate_dashboard()?;
        self.generate_combined_dashboard()?;

        // Generate summary report
        self.generate_summary_report()?;

        println!("\n🎉 Demo completed successfully!");
        println!("📊 Generated files in: {}", self.config.output_dir);
        println!("📋 Check system_summary.txt for detailed report");

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎯 MCP System Monitor Demo Visualization");
    println!("========================================");

    // Configure demo
    let config = DemoConfig::default();
    let mut collector = MetricsCollector::new(config);

    // Run demo
    collector.run_demo().await?;

    Ok(())
}
