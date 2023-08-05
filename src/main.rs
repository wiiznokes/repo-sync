use clap::Parser;
use notify::{Config, Event, RecommendedWatcher, Watcher};
use std::{process::exit, thread, time::Duration};
use tokio::sync::mpsc::{self, Sender};

use crate::{git::is_repo, settings::SettingsArg};

mod git;
mod settings;

fn main() {
    let mut settings_file = settings::SettingsFile::init_settings_file();
    let settings_arg = SettingsArg::parse();
    settings_file.merge_arg(settings_arg);

    if !is_repo(&settings_file.repo_path) {
        eprintln!(
            "{} is not a git repo",
            settings_file.repo_path.to_string_lossy()
        );
        exit(1);
    }

    dbg!(settings_file.clone());

    let (tx, mut rx) = mpsc::channel(100);

    let mut watcher = async_watcher(tx.clone(), map_notify_event).unwrap();

    watcher
        .watch(&settings_file.repo_path, notify::RecursiveMode::Recursive)
        .unwrap();

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs_f32(settings_file.tpull));
        tx.blocking_send(CustomEvent::Pull).unwrap()
    });

    while let Some(event) = rx.blocking_recv() {

        // if modif -> lancer un tache qui sleep pendant x second en envoi Push a la fin
        // si modif pendant que cette meme tache est en cours, relancer cette tache
        dbg!(event);
    }
}

#[derive(Debug, Clone)]
enum CustomEvent {
    Modif,
    Pull,
    Push
}

fn map_notify_event(event: Event) -> Option<CustomEvent> {
    match event.kind {
        notify::EventKind::Create(_)
        | notify::EventKind::Modify(_)
        | notify::EventKind::Remove(_) => Some(CustomEvent::Modif),
        _ => None,
    }
}

pub fn async_watcher<T>(
    tx: Sender<T>,
    f: fn(Event) -> Option<T>,
) -> notify::Result<RecommendedWatcher>
where
    T: 'static + Sync + Send,
{
    let watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Some(msg) = f(res.unwrap()) {
                tx.blocking_send(msg)
                    .expect("can't send event message to the app")
            }
        },
        Config::default(),
    )?;

    Ok(watcher)
}
