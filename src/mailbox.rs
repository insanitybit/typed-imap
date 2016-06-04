use connection::IMAPConnection;
use imaperror::IMAPError;
use std::fmt;

pub struct Envelope;

pub enum DataItem {
    Err((IMAPError, Vec<u8>)),
    Envelope(Envelope),
}

/// A mailbox's permissions will be exposed at the type level through the Mailbox enum
/// Convenience functions that work on *either* type will be provided at the expense of incurring
/// potential runtime errors (ie: trying to modify server when ReadOnly).
#[derive(Debug)]
pub enum Mailbox {
    ReadOnly(ReadOnly), // ReadWrite(ReadWrite),
}

#[derive(Debug)]
pub struct ReadOnly {
    imap: IMAPConnection,
}

impl DataItem {
    pub fn is_err<'a>(&'a self) -> bool {
        match self {
            &DataItem::Err(_) => true,
            _ => false,
        }
    }

    pub fn as_err<'a>(&'a self) -> Option<&'a (IMAPError, Vec<u8>)> {
        match self {
            &DataItem::Err(ref e) => Some(e),
            _ => None,
        }
    }

    pub fn is_envelope<'a>(&'a self) -> bool {
        match self {
            &DataItem::Envelope(_) => true,
            _ => false,
        }
    }

    pub fn as_envelope<'a>(&'a self) -> Option<&'a Envelope> {
        match self {
            &DataItem::Envelope(ref e) => Some(e),
            _ => None,
        }
    }
}

impl ReadOnly {
    pub fn new(conn: &mut IMAPConnection) -> Mailbox {
        unimplemented!();
    }
}

impl Mailbox {
    pub fn fetch(&mut self,
                 sequence_set: &str,
                 data_item_labels: &str)
                 -> Result<Vec<DataItem>, IMAPError> {
        unimplemented!()
    }
}
