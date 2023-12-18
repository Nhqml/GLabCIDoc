use std::{collections::VecDeque, fmt};

use anyhow::Result as AnyResult;

const GLOBAL_KEYWORDS: &'static [&str] = &["default", "include", "stages", "variables", "workflow"];

#[derive(Debug)]
pub(crate) struct Job {
    pub(crate) name: String,
    pub(crate) doc: Option<String>,
}

impl Job {
    pub(crate) fn is_hidden(&self) -> bool {
        self.name.starts_with('.')
    }

    pub(crate) fn is_documented(&self) -> bool {
        self.doc.is_some()
    }
}

impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "# `{}`", self.name)?;
        if let Some(doc) = &self.doc {
            write!(f, "\n\n{}", doc)
        } else {
            Ok(())
        }
    }
}

pub(crate) fn parse_jobs(path: &str) -> AnyResult<Vec<Job>> {
    let file_content = std::fs::read_to_string(path)?;

    let mut jobs = Vec::new();

    let mut doc_block = VecDeque::<String>::new();
    for line in file_content.lines() {
        // This is a part of a doc block
        if line.starts_with("#= ") {
            doc_block.push_back(line[3..].to_string());
        }
        // Empty lines, doc separators, job content and simple comments are simply ignored
        else if line.is_empty() || line == "---" || line.starts_with(" ") || line.starts_with("#")
        {
            continue;
        } else if GLOBAL_KEYWORDS
            .iter()
            .any(|&keyword| line.starts_with(keyword))
        {
            doc_block.clear();
        }
        // This is a job definition
        else {
            let (job_name, _) = line
                .rsplit_once(":")
                .ok_or(anyhow::anyhow!("Expected a job definition: got `{}`", line))?;

            let job_doc: String = doc_block.drain(..).collect::<Vec<String>>().join("\n");

            jobs.push(Job {
                name: job_name.to_string(),
                doc: if job_doc.is_empty() {
                    None
                } else {
                    Some(job_doc)
                },
            });
        }
    }

    Ok(jobs)
}

pub(crate) fn generate_markdown<'a>(jobs: impl IntoIterator<Item = &'a Job>) -> AnyResult<String> {
    Ok(jobs
        .into_iter()
        .map(|job| format!("{}", job))
        .collect::<Vec<String>>()
        .join("\n\n"))
}
