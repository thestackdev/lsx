use chrono::{DateTime, Utc};
use clap::Parser;
use std::path::PathBuf;
use std::{fs, os::unix::fs::MetadataExt};
use strum::Display;
use tabled::{Table, Tabled, settings::Style};

#[derive(Parser)]
struct Args {
    path: Option<PathBuf>,

    #[arg(short, long)]
    all: bool,

    #[arg(short = 'S', long)]
    size: bool,

    #[arg(short = 't', long)]
    time: bool,

    #[arg(short, long)]
    reverse: bool,

    #[arg(short = 'n', long)]
    name: bool,
}

#[derive(Display)]
enum FileType {
    Dir,
    File,
    SymLink,
}

#[derive(Tabled)]
struct FileEntry {
    #[tabled{rename="Name"}]
    name: String,
    #[tabled{rename="Type"}]
    file_type: FileType,
    #[tabled{rename="Size"}]
    size: String,
    #[tabled{rename="Last Modified"}]
    modified_at: String,
    #[tabled{rename="Permissions"}]
    permissions: String,
}

fn main() {
    let args = Args::parse();

    let path = args.path.unwrap_or(PathBuf::from("."));

    if let Ok(exists) = fs::exists(&path) {
        if exists {
            let mut files = get_files(path);
            if !args.all {
                files.retain(|file| !file.name.starts_with("."));
            }

            let mut table = Table::new(files);
            table.with(Style::rounded());
            println!("{}", table);
        } else {
            println!("Path does not exists");
        }
    } else {
        println!("Failed to read path");
    }
}

fn get_files(path: PathBuf) -> Vec<FileEntry> {
    let mut files: Vec<FileEntry> = Vec::new();

    if let Ok(dir) = fs::read_dir(path) {
        for file in dir.flatten() {
            if let Ok(metadata) = file.metadata()
                && let Ok(system_time) = metadata.modified()
            {
                let name = file.file_name().to_string_lossy().to_string();
                let modified: DateTime<Utc> = system_time.into();
                let modified_at = modified.format("%b %d %H:%M").to_string();

                let size = get_human_readable_size(metadata.size());
                let file_type = if metadata.is_dir() {
                    FileType::Dir
                } else if metadata.is_symlink() {
                    FileType::SymLink
                } else {
                    FileType::File
                };

                let permissions = get_formatted_permissions(metadata.mode(), &file_type);

                files.push(FileEntry {
                    name,
                    modified_at,
                    size,
                    file_type,
                    permissions,
                });
            }
        }
    }

    files
}

fn get_formatted_permissions(mode: u32, file_type: &FileType) -> String {
    let file_char = match file_type {
        FileType::Dir => 'd',
        FileType::SymLink => 'l',
        FileType::File => '-',
    };

    let perms = [
        if mode & 0o400 != 0 { 'r' } else { '-' },
        if mode & 0o200 != 0 { 'w' } else { '-' },
        if mode & 0o100 != 0 { 'x' } else { '-' },
        if mode & 0o040 != 0 { 'r' } else { '-' },
        if mode & 0o020 != 0 { 'w' } else { '-' },
        if mode & 0o010 != 0 { 'x' } else { '-' },
        if mode & 0o004 != 0 { 'r' } else { '-' },
        if mode & 0o002 != 0 { 'w' } else { '-' },
        if mode & 0o001 != 0 { 'x' } else { '-' },
    ];

    format!("{}{}", file_char, perms.iter().collect::<String>())
}

fn get_human_readable_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes < KB {
        format!("{} B", bytes)
    } else if bytes < MB {
        format!("{} KB", bytes / KB)
    } else if bytes < GB {
        format!("{} MB", bytes / MB)
    } else {
        format!("{} GB", bytes / GB)
    }
}
