use std::collections::BTreeMap;

use clap::{
    builder::{
        styling::{AnsiColor, Effects, Styles},
        BoolishValueParser,
    },
    ArgAction, Parser,
};

use anyhow::Result as AnyResult;

use crate::pargen::{generate_markdown, parse_jobs, Job};

fn styles() -> Styles {
    Styles::styled()
        .usage(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .valid(AnsiColor::BrightGreen.on_default() | Effects::BOLD)
        .invalid(AnsiColor::Red.on_default() | Effects::BOLD)
        .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::BrightBlue.on_default())
}

#[derive(Parser, Debug)]
#[clap(version, about, long_about)]
#[clap(disable_help_subcommand = true)]
#[clap(help_template = r#"
{name} {version} - {about-with-newline}
{before-help}{usage-heading} {usage}

{all-args}{after-help}"#)]
#[clap(styles = styles())]
pub(crate) struct Cli {
    #[clap(help = "YAML files", required = true)]
    pub(crate) files: Vec<String>,
    #[clap(
        short = 'H',
        long,
        help = "Only consider hidden jobs (i.e. the ones used as templates)"
    )]
    pub(crate) only_hidden: bool,
    #[clap(short = 'd', long, help = "Only consider documented jobs")]
    pub(crate) only_documented: bool,
    #[clap(
        short,
        long = "no-warn",
        action = ArgAction::SetFalse,
        help = "Do not warn about missing documentation for jobs"
    )]
    pub(crate) warn: bool,
    #[clap(long, env = "GLABCIDOC_DEBUG", value_parser = BoolishValueParser::new(), default_value = "false", hide = true)]
    pub(crate) debug: bool,
}

impl Cli {
    pub(crate) fn run(&self) -> AnyResult<()> {
        macro_rules! debug {
            ($val:ident) => {
                if self.debug {
                    println!("{:#?}\n", $val);
                }
            };
        }

        debug!(self);

        let mut jobs = BTreeMap::new();
        for file in &self.files {
            let file_jobs = parse_jobs(file)?;
            for job in file_jobs {
                jobs.insert(job.name.clone(), job);
            }
        }

        debug!(jobs);

        let jobs = jobs
            .values()
            .into_iter()
            .filter(|job| {
                if self.only_hidden && !job.is_hidden() {
                    return false;
                }

                if self.only_documented && !job.is_documented() {
                    return false;
                }

                true
            })
            .collect::<Vec<&Job>>();

        jobs.iter().for_each(|job| {
            if !job.is_documented() && self.warn {
                eprintln!("Warning: `{}` is not documented", job.name);
            }
        });

        if jobs.is_empty() {
            eprintln!("Nothing to generate");
            return Ok(());
        }

        println!("{}", generate_markdown(jobs)?);

        Ok(())
    }
}
