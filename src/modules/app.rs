use crate::modules::{config::AppConfig, datetime, misskey};
use async_trait::async_trait;
use chrono::{DateTime, FixedOffset};
use reqwest::Result;
use std::io::{stdin, stdout, Write};
use clap::Parser;

pub struct App<'a> {
    pub config: &'a AppConfig,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct RunArgs {
    /// Date to fetch (from) yyyy-mm-dd
    #[arg(short, long)]
    since: String,
    /// Date to fetch (from) yyyy-mm-dd
    #[arg(short, long, default_value_t = String::from(""))]
    until: String,
    /// is interactive mode ON
    #[arg(short, long, default_value_t = false)]
    interactive: bool,
}

impl App<'_> {
    pub async fn run(self, arguments: &RunArgs) -> Result<()> {
        let misskey_api_config = misskey::MisskeyApiConfig {
            base_url: self.config.misskey_api_url.clone(),
            token: self.config.misskey_access_token.clone(),
        };
        let interactive = arguments.interactive;
        let since_date = datetime::str_to_datetime(&arguments.since);
        let until_date = if arguments.until == "" {
            None
        } else {
            Some(datetime::str_to_datetime(&arguments.until))
        };

        let runner: Box<dyn Runner> = if interactive {
            Box::new(InteractiveRunner {})
        } else {
            Box::new(AutomaticRunner {})
        };

        let api = misskey::MisskeyApi::new(&misskey_api_config);
        let user = api.i().await?;
        println!("{:?}", user.pinnedNoteIds);

        runner.run(&api, &user, since_date, until_date).await?;

        Ok(())
    }
}

#[async_trait]
trait Runner {
    async fn run<'a>(
        &self,
        api: &'a misskey::MisskeyApi<'a>,
        user: &misskey::User,
        since_date: DateTime<FixedOffset>,
        until_date: Option<DateTime<FixedOffset>>,
    ) -> Result<()>;
}

struct InteractiveRunner;
struct AutomaticRunner;

#[async_trait]
impl Runner for InteractiveRunner {
    async fn run<'a>(
        &self,
        api: &'a misskey::MisskeyApi<'a>,
        user: &misskey::User,
        since_date: DateTime<FixedOffset>,
        until_date: Option<DateTime<FixedOffset>>,
    ) -> Result<()> {
        let params = misskey::ListNotesRequestParams {
            user_id: user.id.clone(),
            since_date,
            limit: 10,
        };
        let notes = api.user_notes(params).await?;
        if notes.is_empty() {
            println!("No notes found.");
            return Ok(());
        }
        let filtered = notes.iter().filter(|note| {
            if let Some(date) = until_date {
                note.createdAt <= date.to_rfc3339()
            } else {
                true
            }
        }).collect::<Vec<&misskey::Note>>();
        if filtered.is_empty() {
            println!("No notes found.");
            return Ok(());
        }

        for (i, note) in filtered.iter().enumerate() {
            println!("{:?}", (i + 1, note));
        }
        print!("Enter number to delete note > ");
        stdout().flush().unwrap();
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();
        println!("your choice: {}", user_input);
        let n = user_input
            .trim()
            .parse::<usize>()
            .expect("Failed to parse input.")
            - 1;
        if let Some(note) = notes.get(n) {
            delete_unless_pinned(&api, &user.pinnedNoteIds, &note).await?;
        } else {
            println!("{} note not found.", n + 1);
        }

        Ok(())
    }
}

#[async_trait]
impl Runner for AutomaticRunner {
    async fn run<'a>(
        &self,
        api: &'a misskey::MisskeyApi<'a>,
        user: &misskey::User,
        since_date: DateTime<FixedOffset>,
        until_date: Option<DateTime<FixedOffset>>,
    ) -> Result<()> {
        let params = misskey::ListNotesRequestParams {
            user_id: user.id.clone(),
            since_date,
            limit: 1,
        };
        let notes = api.user_notes(params).await?;
        if notes.is_empty() {
            println!("No notes found.");
            return Ok(());
        }
        if let Some(note) = notes.first() {
            if let Some(date) = until_date {
                if note.createdAt > date.to_rfc3339() {
                    println!("Note is newer than until_date. Skip.");
                    return Ok(());
                }
            }
            delete_unless_pinned(&api, &user.pinnedNoteIds, &note).await?;
        }

        Ok(())
    }
}

async fn delete_unless_pinned(
    api: &misskey::MisskeyApi<'_>,
    pinned_note_ids: &Vec<String>,
    note: &misskey::Note,
) -> Result<()> {
    if pinned_note_ids.contains(&note.id) {
        println!("Note is pinned. Unpin it first.");
    } else {
        api.delete_note(&note.id).await?;
        println!("Deleted note: {:?}", note);
    }

    Ok(())
}
