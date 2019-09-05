use crate::process::ProcessInfo;
use crate::{column_default, Column};
use std::cmp;
use std::collections::HashMap;

pub struct VmPte {
    header: String,
    unit: String,
    fmt_contents: HashMap<i32, String>,
    raw_contents: HashMap<i32, u64>,
    width: usize,
}

impl VmPte {
    pub fn new() -> Self {
        let header = String::from("VmPte");
        let unit = String::from("[bytes]");
        VmPte {
            fmt_contents: HashMap::new(),
            raw_contents: HashMap::new(),
            width: 0,
            header,
            unit,
        }
    }
}

impl Column for VmPte {
    fn add(&mut self, proc: &ProcessInfo) {
        let (raw_content, fmt_content) = if let Some(ref curr_status) = proc.curr_status {
            if let Some(val) = curr_status.vmpte {
                let val = val.saturating_mul(1024);
                let (size, unit) = unbytify::bytify(val);
                (
                    val,
                    format!("{}{}", size, unit.replace("i", "").replace("B", "")),
                )
            } else {
                (0, String::from(""))
            }
        } else {
            (0, String::from(""))
        };

        self.fmt_contents.insert(proc.pid, fmt_content);
        self.raw_contents.insert(proc.pid, raw_content);
    }

    column_default!(u64);
}
