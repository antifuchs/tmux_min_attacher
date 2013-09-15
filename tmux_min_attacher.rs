extern mod std;

use std::run;
use std::str;
use std::option::*;
use std::trie::*;

fn session_is_attached(line: &str) -> bool {
    line.ends_with("(attached)")
}

fn session_number(line: &str) -> Option<uint> {
    line.split_iter(':').next().and_then(|s| from_str(s))
}

fn detached_sessions(output: ~str) -> TrieSet {
    let mut set = TrieSet::new();
    for line in output.line_iter(){
        if !session_is_attached(line) {
            match session_number(line) {
                Some(n) => { set.insert(n); }
                None => ()
            }
        }
    }
    set
}

fn main() {
    let proc = run::process_output("tmux", &[~"list-sessions"]);
    if proc.status != 0 {
        fail!(fmt!("Tmux exited with an error status: %d", proc.status));
    }
    let output = str::from_utf8(proc.output);
    let sessions = detached_sessions(output);
    match sessions.iter().next() {
        Some(n) => { println(fmt!("Can attach to %u", n)); }
        _ => { println("Have no sessions to attach to"); }
    }
}

#[test]
fn test_session_is_attached() {
    assert!(session_is_attached("11: 1 windows (created Sat Sep 14 17:11:29 2013) [130x65] (attached)"));
    assert!(!session_is_attached("20: 1 windows (created Sat Sep 14 17:11:29 2013) [130x65]"));
}

#[test]
fn test_session_number_with_numbers(){
    match(session_number("11: 1 windows (created Sat Sep 14 17:11:29 2013) [130x65] (attached)")) {
        Some(11) => (),
        Some(n) => fail!(fmt!("Should have returned 11, got %u!", n)),
        None => fail!("Should have returned something, got nothing"),
    }
}

#[test]
fn test_session_number_with_strings(){
    match(session_number("oink: 1 windows (created Sat Sep 14 17:11:29 2013) [130x65] (attached)")) {
        Some(n) => fail!(fmt!("Should have returned None, got %u!", n)),
        None => ()
    }
}

#[test]
fn test_detached_sessions() {
    let set = detached_sessions(~"3: foo (attached)\n2: bar\n4: abz (attached)\nfoo: baz");
    assert!(set.contains(&2));

    // attached sessions:
    assert!(!set.contains(&3));
    assert!(!set.contains(&4));

    // missing session:
    assert!(!set.contains(&1));
}
