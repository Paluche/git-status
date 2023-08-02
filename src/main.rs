use git2::Repository;
use std::env;

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

fn main() {
    let cwd = get_current_working_dir();
    let repo = match Repository::init(cwd) {
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

    println!("Hello, world! HEAD shortname is {}", shortname);
}
