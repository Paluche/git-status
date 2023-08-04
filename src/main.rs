use git2;
use std::env;
use clap::Parser;

/// Custom git-status
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    /// Path to the repository to get the status
    #[arg(short = 'C')]
    path: Option<String>,
}

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

fn status_to_string(status : git2::Status,
                    submodule_status : Option<&git2::SubmoduleStatus>) -> String {
    // Print a status as git status porcelain=v2 does.
    let mut ret = String::with_capacity(9);

    if status.is_ignored() {
        ret.push_str("!");
    }
    else if status.is_wt_new() {
        ret.push_str("?");
    }
    else if status.is_conflicted() {
        ret.push_str("u");
    }
    else if status.is_index_renamed() || status.is_wt_renamed() {
        ret.push_str("2");
    }
    else {
        ret.push_str("1");
    }

    ret.push_str(" ");

    if status.is_index_new() {
        ret.push_str("?");
    }
    else if status.is_index_modified() {
        ret.push_str("M");
    }
    else if status.is_index_deleted() {
        ret.push_str("D");
    }
    else if status.is_index_renamed() {
        ret.push_str("R");
    }
    else if status.is_index_typechange() {
        ret.push_str("T");
    }
    else {
        ret.push_str(".");
    }

    if status.is_wt_modified() {
        ret.push_str("M");
    }
    else if status.is_wt_deleted() {
        ret.push_str("D");
    }
    else if status.is_wt_typechange() {
        ret.push_str("T");
    }
    else if status.is_wt_renamed() {
        ret.push_str("R");
    }
    else {
        ret.push_str(".");
    }

    ret.push_str(" ");

    match submodule_status {
        None => ret.push_str("N..."),
        Some(_submodule_status) => ret.push_str("S..."), // TODO Print submodule status.
    }

    return ret
}

fn main() {
    let args = Args::parse();

    let path = match args.path {
        None => get_current_working_dir(),
        Some(path) => path,
    };

    let repo = match git2::Repository::init(path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };


    let head = match repo.head() {
        Ok(head) => head,
        Err(e) => panic!("failed to get head: {}", e),
    };

    let shortname = match head.shorthand() {
        Some(shortname) => shortname,
        None => panic!("failed to get head shortname"),
    };

    let mut status_options = git2::StatusOptions::new();
    status_options.include_untracked(true);
    status_options.include_ignored(true);
    let statuses = match repo.statuses(Some(&mut status_options)) {
        Ok(statuses) => statuses,
        Err(e) => panic!("failed to get repository status {}", e),
    };

    for status in statuses.iter() {
        let path = match status.path() {
            Some(path) => path,
            None => continue,
        };
        print!("{} {}\n", status_to_string(status.status(), None), path)
    }

    println!("Hello, world! HEAD shortname is {}", shortname);
}
