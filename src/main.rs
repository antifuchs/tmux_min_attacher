//#![feature(convert)]

extern crate nix;

use nix::unistd::execvp;
use std::collections::BTreeSet;
use std::error::Error;
use std::option::Option;
use std::process::Command;
use std::str;
// use libc::{execvp, perror};
use std::env;
use std::ffi::CString;
use std::path::PathBuf;

const PROGRAM: &'static str = "tmux";

fn detached_session_number(line: &str) -> Option<usize> {
    if line.ends_with("(attached)") {
        None
    } else {
        line.split(':').next().and_then(|s| s.parse().ok())
    }
}

fn detached_sessions(output: &str) -> BTreeSet<usize> {
    output
        .lines()
        .filter_map(|line| detached_session_number(line))
        .collect()
}

// TODO: This should return a Result & let the caller decide to run a
// shell or something.
fn exec_program(program: &str, args: &[&str]) {
    // Each of these CStrings has to outlive pointers to it. Sadly,
    // the rust compiler doesn't scream when they don't (see
    // http://is.gd/nS339k), so we better make damn sure that the args
    // will stick around.
    let c_program_as_cstring = CString::new(program).unwrap();
    let args_as_cstring = args
        .iter()
        .map(|arg| CString::new(*arg).unwrap())
        .collect::<Vec<CString>>();

    match execvp(&c_program_as_cstring, &args_as_cstring) {
        Err(e) => {
            println!("execvp of {:?} failed: {:?}", program, e.description());
        }
        _ => {}
    }
}

fn prepare_environment() {
    if let Some(path) = env::var_os("PATH") {
        let mut paths = env::split_paths(&path).collect::<Vec<_>>();
        paths.push(PathBuf::from("/usr/local/bin"));
        let new_path = env::join_paths(paths.iter()).unwrap();
        env::set_var("PATH", &new_path);
    } else {
        let new_path = env::join_paths(
            ["/usr/bin", "/bin", "/usr/local/bin"]
                .iter()
                .map(|p| PathBuf::from(p))
                .collect::<Vec<_>>(),
        )
        .unwrap();
        env::set_var("PATH", &new_path);
    }
}

fn start_server() {
    Command::new(PROGRAM)
        .arg("start-server")
        .status()
        .ok()
        .expect("Could not start tmux server: it exited with an error status.");
}

fn main() {
    prepare_environment();

    start_server();
    let session_output = Command::new(PROGRAM)
        .arg("list-sessions")
        .output()
        .ok()
        .expect("Running list-sessions command exited with an error status");

    let output = str::from_utf8(&session_output.stdout)
        .ok()
        .expect("Could not read the (expected) utf-8 from tmux");
    let sessions = detached_sessions(output);

    match sessions.iter().next() {
        Some(n) => {
            let my_str = n.to_string();
            let session = my_str.as_ref();
            let mut args: Vec<&str> = vec![PROGRAM, "attach-session", "-t"];
            args.push(session);
            exec_program(PROGRAM, &args);
        }
        _ => {
            exec_program(PROGRAM, [PROGRAM].as_ref());
        }
    }
}

#[test]
fn test_session_number_with_numbers() {
    match detached_session_number("11: 1 windows (created Sat Sep 14 17:11:29 2013) [130x65]") {
        Some(11) => (),
        Some(n) => panic!(format!("Should have returned 11, got {}!", n)),
        None => panic!("Should have returned something, got nothing"),
    }
}

#[test]
fn test_session_number_with_strings() {
    match detached_session_number("oink: 1 windows (created Sat Sep 14 17:11:29 2013) [130x65]") {
        Some(n) => panic!(format!("Should have returned None, got {}!", n)),
        None => (),
    }
}

#[test]
fn test_session_number_with_attached_session() {
    match detached_session_number(
        "1: 1 windows (created Sat Sep 14 17:11:29 2013) [130x65] (attached)",
    ) {
        Some(n) => panic!(format!("Should have returned None, got {}!", n)),
        None => (),
    }
}

#[test]
fn test_detached_sessions() {
    let set = detached_sessions("3: foo (attached)\n2: bar\n15: oink\n4: baz (attached)\nfoo: baz");
    // attached sessions:
    assert!(!set.contains(&3));
    assert!(!set.contains(&4));

    // detached sessions should be present:
    assert!(set.contains(&15));
    assert!(set.contains(&2));

    // missing session:
    assert!(!set.contains(&1));
}
