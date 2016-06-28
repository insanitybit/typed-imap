[![License](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/insanitybit/imap-rs/blob/master/LICENSE)

# typed-imap
A typesafe client library for the IMAP protocol.

Currently in the fledgling stages. Entirely *not* working, no stable API.

# Example

```rust
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

```

See [examples/](https://github.com/insanitybit/typed-imap/tree/master/examples) for more.
