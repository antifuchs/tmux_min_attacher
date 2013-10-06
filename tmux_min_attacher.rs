extern mod std;

use std::run;
use std::str;
use std::option::*;
use std::trie::*;
use std::libc::{execvp, perror};
use std::c_str::*;
use std::vec;
use std::ptr;
use std::os;

fn detached_session_number(line: &str) -> Option<uint> {
    if line.ends_with("(attached)") {
        None
    } else {
        line.split_iter(':').next().and_then(|s| from_str(s))
    }
}

fn detached_sessions(output: ~str) -> TrieSet {
    output.line_iter().filter_map(|line| {
            detached_session_number(line)
    }).collect()
}

#[fixed_stack_segment]
fn exec_program(program: &str, args: &[~str]) {
    do program.with_c_str() |c_program| {
        // I don't much care about the ownership of the strings here
        // at this point, so let's just fail if execvp isn't working.
        unsafe {
            let mut c_args = args.map(|arg| { arg.to_c_str().unwrap() });
            c_args.push(ptr::null());
            execvp(c_program, vec::raw::to_ptr(c_args));
            perror("Running tmux failed:".to_c_str().unwrap());
        }
        fail!(fmt!("Couldn't exec %s", program));
    }
}

fn prepare_environment() {
    let path = match os::getenv("PATH") {
        Some(path) => path + ":/usr/local/bin",
        _ => ~"/bin:/usr/bin:/usr/local/bin"
    };
    os::setenv("PATH", path);
}

fn start_server() {
    let proc = run::process_output("tmux", &[~"start-server"]);
    if proc.status != 0 {
        fail!(fmt!("Could not start tmux server: exited with an error status: %d", proc.status));
    }
}

fn main() {
    prepare_environment();

    start_server();
    let proc = run::process_output("tmux", &[~"list-sessions"]);
    if proc.status != 0 {
        fail!(fmt!("Tmux exited with an error status: %d", proc.status));
    }
    let output = str::from_utf8(proc.output);
    let sessions = detached_sessions(output);
    match sessions.iter().next() {
        Some(n) => {
            let session = n.to_str();
            exec_program("tmux", &[~"tmux", ~"attach-session", ~"-t", session]);
        }
        _ => { exec_program("tmux", &[~"tmux"]); }
    }
}

#[test]
fn test_session_number_with_numbers(){
    match(detached_session_number("11: 1 windows (created Sat Sep 14 17:11:29 2013) [130x65]")) {
        Some(11) => (),
        Some(n) => fail!(fmt!("Should have returned 11, got %u!", n)),
        None => fail!("Should have returned something, got nothing"),
    }
}

#[test]
fn test_session_number_with_strings(){
    match(detached_session_number("oink: 1 windows (created Sat Sep 14 17:11:29 2013) [130x65]")) {
        Some(n) => fail!(fmt!("Should have returned None, got %u!", n)),
        None => ()
    }
}

#[test]
fn test_session_number_with_attached_session(){
    match(detached_session_number("1: 1 windows (created Sat Sep 14 17:11:29 2013) [130x65] (attached)")) {
        Some(n) => fail!(fmt!("Should have returned None, got %u!", n)),
        None => ()
    }
}


#[test]
fn test_detached_sessions() {
    let set = detached_sessions(~"3: foo (attached)\n2: bar\n15: oink\n4: baz (attached)\nfoo: baz");
    // attached sessions:
    assert!(!set.contains(&3));
    assert!(!set.contains(&4));

    // detached sessions should be present:
    assert!(set.contains(&15));
    assert!(set.contains(&2));

    // missing session:
    assert!(!set.contains(&1));
}
