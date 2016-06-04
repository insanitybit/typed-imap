use connection::IMAPConnection;
use imaperror::IMAPError;
use response::{Response, parse_responses};
use validate_helpers::*;
use mailbox::{Mailbox, ReadOnly};
use rand::thread_rng;
use rand::Rng;
use std::fmt;
use std::mem;
use std::ptr;

#[derive(Debug)]
pub struct Tag {
    tag_prefix: String,
    tag: u32,
}

#[derive(Debug)]
pub struct NotAuthenticated {
    connection: IMAPConnection,
    parsed_responses: Vec<Response>,
    tag: Tag,
}

#[derive(Debug)]
pub struct Authenticated {
    connection: IMAPConnection,
    parsed_responses: Vec<Response>,
    tag: Tag,
}

#[derive(Debug)]
pub struct Selected {
    connection: IMAPConnection,
    parsed_responses: Vec<Response>,
    mailbox: Mailbox,
    tag: Tag,
}

#[derive(Debug)]
pub struct LoggedOut {
    connection: IMAPConnection,
    parsed_responses: Vec<Response>,
    tag: Tag,
}

impl Tag {
    fn new() -> Tag {
        let mut rng = thread_rng();
        let rstr: String = rng.gen_ascii_chars()
                              .take(4)
                              .collect();
        Tag {
            tag_prefix: rstr,
            tag: rng.gen_ascii_chars().next().unwrap() as u32,
        }
    }

    /// Increments and then returns the tag.
    pub fn next_tag(mut self) -> Self {
        self.tag += 1;
        self
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{:04}", self.tag_prefix, self.tag)
    }
}

impl NotAuthenticated {
    pub fn new(connection: IMAPConnection) -> Result<NotAuthenticated, IMAPError> {
        let mut connection = connection;
        let raw_res = try!(connection.command(""));
        let greeting = validate_greeting(&raw_res[..]);

        Ok(NotAuthenticated {
            connection: connection,
            parsed_responses: parse_responses(&raw_res[..]),
            tag: Tag::new(),
        })

    }

    pub fn login(mut self,
                 username: &str,
                 passwd: &str)
                 -> Result<Authenticated, (NotAuthenticated, IMAPError)> {

        let login_response = try_imap!(self,
                                       self.connection
                                           .command(&format!("{} LOGIN {} {}",
                                                             self.tag,
                                                             username,
                                                             passwd)));

        let _ = try_imap!(self, validate_login(&login_response[..]));

        Ok(Authenticated {
            connection: self.connection,
            parsed_responses: parse_responses(&login_response[..]),
            tag: self.tag.next_tag(),
        })

    }

    pub fn logout(mut self) -> Result<LoggedOut, (NotAuthenticated, IMAPError)> {
        let logout_response = try_imap!(self, self.connection.command("fake login command"));

        Ok(LoggedOut {
            connection: self.connection,
            parsed_responses: parse_responses(&logout_response[..]),
            tag: self.tag.next_tag(),
        })
    }
}

impl Authenticated {
    pub fn select(mut self, mailbox: &str) -> Result<Selected, (Authenticated, IMAPError)> {
        let select_response = try_imap!(self,
                                        self.connection
                                            .command(&format!("{} SELECT {}", self.tag, mailbox)));

        let _ = try_imap!(self, validate_select(&select_response[..]));

        // This is unsfae due to the use of mem::uninitialized::<T>()
        // Because we can not borrow &mut self.connection, as it is moved,
        // we have to temporarily create an unsafe Selected, and then create
        // a reference to *its* connection, to initialize the mailbox
        let mut selected = unsafe {
            let mut selected = Selected {
                connection: self.connection,
                parsed_responses: parse_responses(&select_response[..]),
                mailbox: mem::uninitialized(),
                tag: self.tag.next_tag(),
            };
            ptr::write(&mut selected.mailbox,
                       ReadOnly::new(&mut selected.connection));
            selected
        };

        Ok(selected)
    }

    pub fn logout(mut self) -> Result<LoggedOut, (Authenticated, IMAPError)> {
        let logout_response = try_imap!(self, self.connection.command("fake login command"));

        Ok(LoggedOut {
            connection: self.connection,
            parsed_responses: parse_responses(&logout_response[..]),
            tag: self.tag.next_tag(),
        })
    }
}

impl Selected {
    pub fn borrow_mailbox(&mut self) -> &mut Mailbox {
        &mut self.mailbox
    }

    pub fn logout(mut self) -> Result<LoggedOut, (Selected, IMAPError)> {
        let logout_response = try_imap!(self, self.connection.command("fake logout command"));

        Ok(LoggedOut {
            connection: self.connection,
            parsed_responses: parse_responses(&logout_response[..]),
            tag: self.tag.next_tag(),
        })
    }
}

impl LoggedOut {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
