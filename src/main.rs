mod colored_string;

//use git2;
use clap::Parser;
use colored_string::Color;
use colored_string::ColoredString;
use std::env;

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
        Err(_) => "FAILED".to_string(),
    }
}

fn status_to_string(
    &ret: ColoredString,
    status: git2::Status,
    submodule_status: Option<git2::SubmoduleStatus>,
    colored: bool,
) {
    // Print a status as git status porcelain=v2 does.

    if status.is_ignored() {
        ret.push('!');
    } else if status.is_wt_new() {
        ret.push('?');
    } else if status.is_conflicted() {
        ret.push('u');
    } else if status.is_index_renamed() || status.is_wt_renamed() {
        ret.push('2');
    } else {
        ret.push('1');
    }

    ret.push(' ');

    if status.is_index_new() {
        ret.push('?');
    } else if status.is_index_modified() {
        ret.push('M');
    } else if status.is_index_deleted() {
        ret.push('D');
    } else if status.is_index_renamed() {
        ret.push('R');
    } else if status.is_index_typechange() {
        ret.push('T');
    } else if colored {
        ret.push(' ');
    } else {
        ret.push('.');
    }

    if status.is_wt_modified() {
        ret.push('M');
    } else if status.is_wt_deleted() {
        ret.push('D');
    } else if status.is_wt_typechange() {
        ret.push('T');
    } else if status.is_wt_renamed() {
        ret.push('R');
    } else if colored {
        ret.push(' ');
    } else {
        ret.push('.');
    }

    ret.push(' ');

    match submodule_status {
        None => ret.push_str("N..."),
        Some(submodule_status) => submodule_status_to_string(ret, submodule_status, colored),
    }
}

fn submodule_status_to_string(
    &ret: ColoredString,
    submodule_status: git2::SubmoduleStatus,
    colored: bool,
) {
    ret.set_fg_color(Color::BLACK);
    ret.push('S');

    if submodule_status.is_in_head() {
        ret.push('H');
    } else if submodule_status.is_in_index() {
        ret.push('I');
    } else if submodule_status.is_in_config() {
        ret.push('C');
    } else if submodule_status.is_in_wd() {
        ret.push('W');
    } else if colored {
        ret.push(' ');
    } else {
        ret.push('.');
    }

    if submodule_status.is_index_added() {
        ret.push('A');
    } else if submodule_status.is_index_deleted() {
        ret.push('D');
    } else if submodule_status.is_index_modified() {
        ret.push('M');
    } else {
        ret.push('.');
    }

    if submodule_status.is_wd_uninitialized() {
        ret.push('U');
    } else if submodule_status.is_wd_added() {
        ret.push('A');
    } else if submodule_status.is_wd_deleted() {
        ret.push('D');
    } else if submodule_status.is_wd_modified() {
        ret.push('M');
    } else if submodule_status.is_wd_wd_modified() {
        ret.push('W');
    } else if submodule_status.is_wd_untracked() {
        ret.push('U');
    } else if colored {
        ret.push(' ');
    } else {
        ret.push('.');
    }
}

fn main() {
    let args = Args::parse();

    let mut ret = ColoredString::new();

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

    let submodule_ignore = git2::SubmoduleIgnore::All;

    let statuses = match repo.statuses(Some(&mut status_options)) {
        Ok(statuses) => statuses,
        Err(e) => panic!("failed to get repository status {}", e),
    };

    for status in statuses.iter() {
        let path = match status.path() {
            Some(path) => path,
            None => continue,
        };

        let submodule_status = match repo.submodule_status(path, submodule_ignore) {
            Ok(submodule_status) => Some(submodule_status),
            Err(_e) => None,
        };

        status_to_string(&ret, status.status(), submodule_status, true);

        print!("{} {}\n", ret.colored(), path)
    }

    println!("Hello, world! HEAD shortname is {}", shortname);
}
