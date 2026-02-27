//! Shortlog command for summarizing commit history by author.
//!
//! This module implements a `git shortlog`-style report used primarily for
//! release announcements and contributor overviews. It is structured as a
//! standard CLI command module, following the conventions used by other
//! commands in this crate:
//! 
//! - **Argument parsing** is handled by [`ShortlogArgs`], which defines the
//!   supported flags and options using `clap::Parser`. The key flags are:
//!   - `numbered` (`-n` / `--numbered`): sort authors by descending commit
//!     count rather than by name.
//!   - `summary` (`-s` / `--summary`): emit only per-author commit counts,
//!     suppressing individual commit subjects.
//!   - `email` (`-e` / `--email`): include the author email address in the
//!     report header.
//!   - `since` / `until`: restrict the set of commits by author timestamp,
//!     using the repository-wide date parser in [`parse_date`].
//!
//! - **Execution entrypoints**:
//!   - [`execute`] is the user-facing async entrypoint used by the CLI
//!     dispatcher. It writes human-readable output to `stdout`.
//!   - [`execute_to`] contains the core logic and is parameterized over an
//!     arbitrary `Write` implementor, which makes it easier to test and to
//!     reuse from other tooling without being tied to a specific output
//!     stream.
//!
//! - **Commit collection and filtering**:
//!   - [`get_commits_for_shortlog`] resolves the current [`Head`] and
//!     obtains the relevant list of [`Commit`] objects to be included in the
//!     report. The exact traversal strategy is delegated to the internal git
//!     engine.
//!   - [`passes_filter`] applies `since`/`until` constraints to each
//!     commit, converting user-supplied date strings via [`parse_date`] and
//!     comparing them against the commit author timestamp.
//!
//! - **Aggregation and formatting**:
//!   - Commits are grouped by author identity in an in-memory
//!     `HashMap<String, AuthorStats>`, where [`AuthorStats`] tracks the
//!     author name, optional email address, total commit count, and a list
//!     of commit subjects.
//!   - After aggregation, the authors are converted to a vector, optionally
//!     sorted by commit count (`numbered`) or left in deterministic order,
//!     and finally rendered to the provided writer in either detailed or
//!     summary form depending on the `summary` flag.
//!
//! The implementation is intentionally streaming-friendly at the output
//! layer (it writes directly to the provided `Write`), while still
//! aggregating per-author statistics in memory for predictable formatting.

use std::{collections::HashMap, io::Write};

use clap::Parser;
use git_internal::internal::object::commit::Commit;

use crate::internal::{head::Head, log::date_parser::parse_date};

#[derive(Parser, Debug)]
pub struct ShortlogArgs {
    /// Sort output according to the number of commits per author
    #[clap(short = 'n', long = "numbered")]
    pub numbered: bool,

    /// Suppress commit description and provide a commit count summary only
    #[clap(short = 's', long = "summary")]
    pub summary: bool,

    /// Show the email address of each author
    #[clap(short = 'e', long = "email")]
    pub email: bool,

    /// Show commits more recent than a specific date
    #[clap(long = "since")]
    pub since: Option<String>,

    /// Show commits older than a specific date
    #[clap(long = "until")]
    pub until: Option<String>,
}

struct AuthorStats {
    name: String,
    email: String,
    count: usize,
    subjects: Vec<String>,
}

impl AuthorStats {
    fn new(name: String, email: String) -> Self {
        Self {
            name,
            email,
            count: 0,
            subjects: Vec::new(),
        }
    }

    fn add_commit(&mut self, subject: String) {
        self.count += 1;
        self.subjects.push(subject);
    }
}

pub async fn execute_to(args: ShortlogArgs, writer: &mut impl Write) {
    if !crate::utils::util::check_repo_exist() {
        return;
    }

    let commits = get_commits_for_shortlog(&args).await;

    let mut author_map: HashMap<String, AuthorStats> = HashMap::new();

    for commit in commits {
        let author_name = commit.author.name.clone();
        let author_email = commit.author.email.clone();
        let key = format!("{} <{}>", author_name, author_email);

        let subject = commit
            .message
            .trim()
            .lines()
            .next()
            .unwrap_or("")
            .to_string();

        author_map
            .entry(key)
            .or_insert_with(|| AuthorStats::new(author_name.clone(), author_email.clone()))
            .add_commit(subject);
    }

    let mut authors: Vec<(&String, &AuthorStats)> = author_map.iter().collect();

    if args.numbered {
        // Sort by commit count (descending) and then by author name (ascending) to ensure deterministic output
        authors.sort_by(|a, b| {
            b.1.count
                .cmp(&a.1.count)
                .then_with(|| a.1.name.to_lowercase().cmp(&b.1.name.to_lowercase()))
        });
    } else {
        authors.sort_by(|a, b| a.1.name.to_lowercase().cmp(&b.1.name.to_lowercase()));
    }

    for (_key, stats) in authors {
        if args.email {
            writeln!(
                writer,
                "{:>4}  {} <{}>",
                stats.count, stats.name, stats.email
            )
            .unwrap();
        } else {
            writeln!(writer, "{:>4}  {}", stats.count, stats.name).unwrap();
        }
        if !args.summary {
            for subject in &stats.subjects {
                writeln!(writer, "      {}", subject).unwrap();
            }
        }
    }
}

pub async fn execute(args: ShortlogArgs) {
    execute_to(args, &mut std::io::stdout()).await;
}

async fn get_commits_for_shortlog(args: &ShortlogArgs) -> Vec<Commit> {
    use crate::command::log::get_reachable_commits;

    let head = Head::current().await;
    let commit_hash = match head {
        Head::Branch(name) => {
            let branch = crate::internal::branch::Branch::find_branch(&name, None)
                .await
                .map(|b| b.commit.to_string());
            match branch {
                Some(h) => h,
                None => {
                    eprintln!("fatal: current branch has no commits");
                    return Vec::new();
                }
            }
        }
        Head::Detached(hash) => hash.to_string(),
    };

    let mut commits: Vec<Commit> = get_reachable_commits(commit_hash, None)
        .await
        .into_iter()
        .filter(|c| passes_filter(c, args))
        .collect();

    commits.sort_by(|a, b| b.author.timestamp.cmp(&a.author.timestamp));

    commits
}

fn passes_filter(commit: &Commit, args: &ShortlogArgs) -> bool {
    if let Some(since_str) = &args.since
        && let Ok(since_ts) = parse_date(since_str)
    {
        let commit_ts = commit.author.timestamp as i64;
        if commit_ts < since_ts {
            return false;
        }
    }

    if let Some(until_str) = &args.until
        && let Ok(until_ts) = parse_date(until_str)
    {
        let commit_ts = commit.author.timestamp as i64;
        if commit_ts > until_ts {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args() {
        let args = ShortlogArgs::parse_from(["shortlog"]);
        assert!(!args.numbered);
        assert!(!args.summary);
        assert!(!args.email);

        let args = ShortlogArgs::parse_from(["shortlog", "-n", "-s", "-e"]);
        assert!(args.numbered);
        assert!(args.summary);
        assert!(args.email);

        let args = ShortlogArgs::parse_from(["shortlog", "--since", "2024-01-01"]);
        assert!(args.since.is_some());
    }
}
