use std::collections::HashSet;
use std::str::Split;

use command::Command;
use command::Attribute::BodySection;
use folder::Folder;
use parser::{self, ParserResult};

use message::Flag::Seen;
use super::store::StoreName::Add;

/// Take the rest of the arguments provided by the client and parse them into a
/// `Command` object with `parser::fetch`.
pub fn fetch(args: Split<char>) -> ParserResult<Command> {
    let mut cmd = "FETCH".to_string();
    for arg in args {
        cmd.push(' ');
        cmd.push_str(arg);
    }

    parser::fetch(cmd.as_bytes())
}

/// Perform the fetch operation on each sequence number indicated and return
/// the response to be sent back to the client.
pub fn fetch_loop(parsed_cmd: &Command, folder: &mut Folder,
                  sequence_iter: &[usize], tag: &str, uid: bool) -> String {
    for attr in &parsed_cmd.attributes {
        if let BodySection(_, _) = *attr {
            let mut seen_flag_set = HashSet::new();
            seen_flag_set.insert(Seen);
            folder.store(sequence_iter.to_vec(), &Add, true, seen_flag_set,
                         false, tag);
            break;
        }
    }

    let mut res = String::new();
    for i in sequence_iter {
        let index = if uid {
            match folder.get_index_from_uid(i) {
                Some(index) => *index,
                None => {continue;}
            }
        } else {
            *i-1
        };
        res.push_str(&folder.fetch(index, &parsed_cmd.attributes)[..]);
    }
    res.push_str(tag);
    res.push_str(" OK ");
    if uid {
        res.push_str("UID ");
    }
    res.push_str("FETCH completed\r\n");
    res
}