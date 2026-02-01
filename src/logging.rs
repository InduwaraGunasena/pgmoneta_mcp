// Copyright (C) 2026 The pgmoneta community
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::constant::{LogLevel, LogType};
use crate::constant::LogMode;
use anyhow::Context;
use std::fs::OpenOptions;
use syslog_tracing::Syslog;
use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::time::ChronoUtc;
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer, Registry};

pub struct Logger;

impl Logger {
    pub fn init(
        log_level: &str,
        log_type: &str,
        log_format: &str,
        log_path: &str,
        log_mode: &str,
    ) -> Option<WorkerGuard> {
        let (writer, guard) = Self::make_writer(log_type, log_path, log_mode).unwrap_or_else(|e| {
            eprintln!(
                "Failed to initialize logging: {:?} \nDefault logging to stderr",
                e
            );
            (BoxMakeWriter::new(std::io::stderr), None)
        });
        let level = Self::get_level(log_level);
        let targets = Targets::new()
            .with_target("pgmoneta_mcp", level)
            .with_target("tokio", LevelFilter::WARN)
            .with_target("rmcp", LevelFilter::WARN)
            .with_default(LevelFilter::OFF);
        Registry::default()
            .with(
                tracing_subscriber::fmt::layer()
                    .with_line_number(true)
                    .with_timer(ChronoUtc::new(log_format.to_string()))
                    .with_writer(writer)
                    .with_ansi(false)
                    .with_filter(targets),
            )
            .init();
        guard
    }

    fn get_level(log_level: &str) -> LevelFilter {
        match log_level {
            LogLevel::TRACE => LevelFilter::TRACE,
            LogLevel::DEBUG => LevelFilter::DEBUG,
            LogLevel::INFO => LevelFilter::INFO,
            LogLevel::WARN => LevelFilter::WARN,
            LogLevel::ERROR => LevelFilter::ERROR,
            _ => LevelFilter::INFO,
        }
    }

    fn make_writer(
        log_type: &str,
        log_path: &str,
        log_mode: &str,
    ) -> anyhow::Result<(BoxMakeWriter, Option<WorkerGuard>)> {
        match log_type {
            LogType::CONSOLE => Ok((BoxMakeWriter::new(std::io::stderr), None)),
            LogType::FILE => match log_mode {
                LogMode::CREATE => {
                    let file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(log_path)
                        .context(format!("Failed to open log file: {}", log_path))?;

                    let (writer, _guard) = tracing_appender::non_blocking(file);
                    Ok((BoxMakeWriter::new(writer), Some(_guard)))
                }
                LogMode::APPEND => {
                    let file_appender = rolling::never(".", log_path);
                    let (writer, _guard) = tracing_appender::non_blocking(file_appender);
                    Ok((BoxMakeWriter::new(writer), Some(_guard)))
                }
                _ => Err(anyhow::anyhow!("Invalid log mode: {}", log_mode)),
            },
            LogType::SYSLOG => {
                let identity = c"pgmoneta-mcp";
                let (options, facility) = Default::default();
                let syslog = Syslog::new(identity, options, facility).unwrap();
                Ok((BoxMakeWriter::new(syslog), None))
            }
            _ => Err(anyhow::anyhow!("Invalid log type: {}", log_type)),
        }
    }
}
