use git2::{Error as GitError, Repository};
use std::io::Error as IoError;
use std::{env, fs, process};

const BRANCH_NAMES: [&str; 2] = ["main", "master"];

fn get_branch_name() -> Result<String, GitError> {
    let repo = Repository::open(".")?;
    let head = repo.head()?;
    Ok(head.shorthand().unwrap_or("").to_string())
}

fn update_commit_message(commit_msg_file: &str, branch_name: &str) -> Result<(), IoError> {
    let commit_msg = fs::read_to_string(commit_msg_file)?;
    let new_commit_msg = format!("{}: {}", branch_name, commit_msg);
    fs::write(commit_msg_file, new_commit_msg)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <commit_msg_file>", args[0]);
        process::exit(1);
    }

    let commit_msg_file = &args[1];

    let branch_name = match get_branch_name() {
        Ok(name) => name,
        Err(err) => {
            eprintln!("Failed to get branch name: {}", err);
            return;
        }
    };

    if branch_name.is_empty() || BRANCH_NAMES.contains(&branch_name.as_str()) {
        println!("No branch name found or on default branch. Skipping commit message update.");
        return;
    }

    match update_commit_message(commit_msg_file, &branch_name) {
        Ok(()) => println!("Commit message updated with branch name."),
        Err(err) => eprintln!("Failed to update commit message: {}", err),
    }
}

#[cfg(test)]
#[test]
fn test_get_branch_name() {
    let branch_name = get_branch_name().unwrap();
    assert!(!branch_name.is_empty());
}

#[test]
fn test_update_commit_message() {
    let commit_msg_file = "test_commit_msg.txt";
    let branch_name = "test-branch";
    let commit_msg = "Test commit message";
    fs::write(commit_msg_file, commit_msg).unwrap();
    update_commit_message(commit_msg_file, branch_name).unwrap();
    let new_commit_msg = fs::read_to_string(commit_msg_file).unwrap();
    assert_eq!(new_commit_msg, format!("{}: {}", branch_name, commit_msg));
    fs::remove_file(commit_msg_file).unwrap();
}
