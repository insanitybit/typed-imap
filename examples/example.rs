extern crate typed_imap;

use typed_imap::client::*;
use typed_imap::connection::IMAPConnection;

fn main() {
    let client = {
        let conn = IMAPConnection::new_tls("outlook.office365.com", 993).unwrap();
        NotAuthenticated::new(conn).unwrap()
    };

    let client = client.login("username", "passwd").unwrap();

    let mut client = client.select("inbox").unwrap();

    // We need to act on the mailbox in a new scope to prevent a situation where
    // a client has a mailbox but moves to a new state.
    {
        let mut mailbox = client.borrow_mailbox();
        let emails = mailbox.fetch("2:4", "BODY").unwrap();
    }

    client.logout();
}
