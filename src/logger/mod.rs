use std::io::Write;

use colorize::AnsiColor;
pub use log::*;

#[derive(Debug)]
pub struct Filter {
    pub filter: Vec<&'static str>,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            filter: vec!["wgpu_core", "wgpu_hal", "naga", "calloop", "winit"],
        }
    }
}

impl Filter {
    pub fn new(filter: &[&'static str]) -> Self {
        Self {
            filter: filter.into(),
        }
    }

    #[allow(clippy::all)]
    pub fn add(mut self, filter: &'static str) -> Self {
        self.filter.push(filter);
        self
    }

    pub fn clear(mut self) -> Self {
        self.filter.clear();
        self
    }
}

//

pub fn init(filter: Filter) {
    let mut builder = env_logger::Builder::default();

    builder.filter_module("", LevelFilter::max());

    for filter in filter.filter {
        builder.filter_module(filter, LevelFilter::Off);
    }

    builder.format(|buf, record| {
        let mut out: Vec<char> = record.level().as_str().to_lowercase().chars().collect();
        out[0] = out[0].to_uppercase().next().unwrap();
        let mut out: String = out.iter().collect();

        match record.level() {
            log::Level::Error => out = out.red().bold(),
            log::Level::Warn => out = out.yellow().bold(),
            log::Level::Info => out = out.green().bold(),
            log::Level::Debug => out = out.grey().bold(),
            log::Level::Trace => out = out.cyan().bold(),
        }

        let mut out0: String = String::new();
        out0 += " <";
        out0 += &record.module_path().unwrap_or("?");
        out0 += "(";
        out0 += &record.line().unwrap_or(0).to_string();
        out0 += ")";
        out0 += ">:";
        out0 = out0.grey();

        out += out0.as_str();

        writeln!(buf, "{} {}", out, record.args())
    });

    builder.init();
}
