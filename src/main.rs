use git2;
use std::env;

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

fn status_to_string(status : git2::Status) -> String {
    let mut ret = String::with_capacity(6);

    if status.is_ignored() {
        ret.push_str("?? ????");
    }
    else if status.is_conflicted() {
        ret.push_str("CC     ");
    }
    else {
        if status.is_index_new() {
            ret.push_str("N");
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
            ret.push_str(" ");
        }

        if status.is_wt_new() {
            ret.push_str("N");
        }
        else if status.is_wt_modified() {
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
            ret.push_str(" ");
        }
    }

    return ret
}

fn main() {
    let cwd = get_current_working_dir();
    let repo = match git2::Repository::init(cwd) {
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
    let statuses = match repo.statuses(Some(&mut status_options)) {
        Ok(statuses) => statuses,
        Err(e) => panic!("failed to get repository status {}", e),
    };

    for status in statuses.iter() {
        let path = match status.path() {
            Some(path) => path,
            None => continue,
        };
        print!("{} {}\n", status_to_string(status.status()), path)
    }

    println!("Hello, world! HEAD shortname is {}", shortname);
}
