use std::{fs::File, io::Write, os::fd::FromRawFd};

use crate::prof::Event;

use super::Outputter;

pub struct PlainOutput {
    fd: i32,
}

impl PlainOutput {
    pub fn new(fd: i32) -> Self {
        PlainOutput { fd }
    }
}

impl Outputter for PlainOutput {
    fn output(&mut self, events: &[Event]) {
        let mut file = unsafe { File::from_raw_fd(self.fd) };

        for event in events {
            if let Some(value) = event.value {
                write!(
                    file,
                    "{:.2}   {}: {} {}\n",
                    event.timestamp_millis,
                    event.description,
                    value,
                    event.unit
                )
                .unwrap();
            }
        }

        file.flush().unwrap();
    }
}
