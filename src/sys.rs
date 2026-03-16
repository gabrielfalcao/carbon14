use copypasta::{ClipboardContext, ClipboardProvider};
use std::io::{stdin, IsTerminal};
use std::io::read_to_string;


pub fn clipboard_ops() -> Option<String> {
    ClipboardContext::new().map(|c| Some(c)).unwrap_or(None).map(|mut c| c.get_contents().map(|s| Some(s)).unwrap_or(None)).unwrap_or(None)
}

pub fn stdin_ops() -> Option<String> {
    if stdin().is_terminal() {
        None }
    else {
        read_to_string(stdin().lock()).map(|s| Some(s)).unwrap_or(None)
    }
}

pub fn clipboard_lines() -> Option<Vec<String>> {
    oplines(clipboard_ops())
}

pub fn stdin_lines() -> Option<Vec<String>> {
    oplines(stdin_ops())
}

pub fn oplines(ops: Option<String>) -> Option<Vec<String>> {
    ops.map(|s| s.lines().map(String::from).collect::<Vec<String>>()).filter(|t| !t.is_empty())
}
