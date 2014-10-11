//extern crate std;
extern crate libc;

use std::io::Command;
use std::str;
use std::option::Option;
use std::collections::TrieSet;
use libc::{execvp, perror};
//use std::c_str::ptr;
use std::ptr;
use std::os;

fn detached_session_number(line: &str) -> Option<uint> {
    if line.ends_with("(attached)") {
        None
    } else {
        line.split(':').next().and_then(|s| from_str(s))
    }
}

fn detached_sessions(output: &str) -> TrieSet {
    output.lines().filter_map(|line| {
            detached_session_number(line)
    }).collect()
}

fn exec_program(program: &str, args: &[&str]) {
    program.with_c_str(|c_program| {
            // I don't much care about the ownership of the strings here
            // at this point, so let's just fail if execvp isn't working.
            unsafe {
                let mut c_args = vec![];
                for &arg in args.iter() {
                    c_args.push(arg.to_c_str().unwrap());
                }
                c_args.push(ptr::null());
                execvp(c_program, c_args.as_mut_ptr());
                perror("Running tmux failed:".to_c_str().unwrap());
            }
        });
    fail!("Couldn't exec {:s}", program);
}

fn prepare_environment() {
    let path = match os::getenv("PATH") {
        Some(path) => path + ":/usr/local/bin",
        _ => "/bin:/usr/bin:/usr/local/bin".to_string()
    };
    os::setenv("PATH", path.as_slice());
}

fn start_server() {
    Command::new("tmux").arg("start-server").status().ok()
        .expect("Could not start tmux server: it exited with an error status.");
}

fn main() {
    prepare_environment();

    start_server();
    let session_output = Command::new("tmux").arg("list-sessions").output().ok()
        .expect("Running list-sessions command exited with an error status");

    let output = str::from_utf8(session_output.output.as_slice())
        .expect("Could not read the (expected) utf-8 from tmux");
    let sessions = detached_sessions(output);

    match sessions.iter().next() {
        Some(n) => {
            let my_str = n.to_string();
            let session = my_str.as_slice();
            let mut args: Vec<&str> = vec!["tmux", "attach-session", "-t"];
            args.push(session);
            exec_program("tmux", args.as_slice());
        }
        _ => { exec_program("tmux", ["tmux"]); }
    }
}

#[test]
fn test_session_number_with_numbers(){
    match(detached_session_number("11: 1 windows (created Sat Sep 14 17:11:29 2013) [130x65]")) {
        Some(11) => (),
        Some(n) => fail!(format!("Should have returned 11, got {:u}!", n)),
        None => fail!("Should have returned something, got nothing"),
    }
}

#[test]
fn test_session_number_with_strings(){
    match(detached_session_number("oink: 1 windows (created Sat Sep 14 17:11:29 2013) [130x65]")) {
        Some(n) => fail!(format!("Should have returned None, got {:u}!", n)),
        None => ()
    }
}

#[test]
fn test_session_number_with_attached_session(){
    match(detached_session_number("1: 1 windows (created Sat Sep 14 17:11:29 2013) [130x65] (attached)")) {
        Some(n) => fail!(format!("Should have returned None, got {:u}!", n)),
        None => ()
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
