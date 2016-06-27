use nom::IResult::*;
use nom::*;


#[derive(Debug)]
pub struct Address<'a> {
    pub addr_name: &'a [u8],
    pub addr_adl: &'a [u8],
    pub addr_mailbox: &'a [u8],
    pub addr_host: &'a [u8],
}

#[derive(Debug)]
pub struct Envelope<'a> {
    pub date: &'a [u8],
    pub subject: &'a [u8],
    pub from: Option<Vec<Address<'a>>>,
    pub senter: Option<Vec<Address<'a>>>,
    pub reply_to: Option<Vec<Address<'a>>>,
    pub to: Option<Vec<Address<'a>>>,
    pub cc: Option<Vec<Address<'a>>>,
    pub bcc: Option<Vec<Address<'a>>>,
    pub in_reply_to: &'a [u8],
    pub message_id: &'a [u8],
}


named!(pub nil,
    alt!(
        tag!(b"NIL")
        | tag!(b"nil")
    )
);

named!(pub resp_specials, tag!("]"));

named!(pub quoted_specials,
    alt!(
        tag!("\"")
        | tag!("\\")
    )
);

named!(pub list_wildcards,
    alt!(
        tag!("%")
        | tag!("*")
    )
);

named!(pub atom_specials,
    alt!(
        tag!("(")
        | tag!(")")
        | tag!("{")
        | tag!(" ")
        | tag!("(")
        | list_wildcards
        | resp_specials
        | quoted_specials
    )
);

pub fn atom_char(bytes: &[u8]) -> IResult<&[u8], &[u8]> {
    cond_reduce!(bytes,
                 !atom_specials(bytes).is_done(),
                 chain!(
        alpha: peek!(alphanumeric) ~
        _____: take!(1) ,
        || { &alpha[..1] }
    ))
}

pub fn atom(bytes: &[u8]) -> IResult<&[u8], &[u8]> {
    let mut offset = 0;
    loop {
        if atom_char(&bytes[offset..]).is_done() {
            offset += 1;
        } else {
            break;
        }
    }
    take!(bytes, offset)
}

named!(pub nstring,
       alt!(
           delimited!(
               char!('"'),
               is_not!("\""),
               char!('"')
           )
           | nil
       )
   );

named!(pub astring_char,
   alt!(
       atom_char
       | resp_specials
   )
);

#[inline_always]
named!(many_addr_or_nil <&[u8], Option<Vec<Address> > >,
    alt!(
        chain!(
            char!('(') ~
            addresses: many1!(address) ~
            char!(')'),
            || {
                Some(addresses)
            }
        )
        | chain!(
            nil,
            || {
                None
            }
        )
    )
);

named!(pub env_bcc <&[u8], Option<Vec<Address> > >,
    chain!(
        maon: many_addr_or_nil,
        || {
            maon
        })
);

named!(pub env_cc <&[u8], Option<Vec<Address> > >,
    chain!(
        maon: many_addr_or_nil,
        || {
            maon
        })
);

named!(pub env_from <&[u8], Option<Vec<Address> > >,
    chain!(
        maon: many_addr_or_nil,
        || {
            maon
        })
);

named!(pub env_date,
    chain!(
        ns: nstring,
        || {
            ns
        })
);

named!(pub env_in_reply_to,
    chain!(
        ns: nstring,
        || {
            ns
        })
);

named!(pub env_message_id,
    chain!(
        ns: nstring,
        || {
            ns
        })
);

named!(pub env_reply_to <&[u8], Option<Vec<Address> > >,
    chain!(
        maon: many_addr_or_nil,
        || {
            maon
        })
);

named!(pub env_sender <&[u8], Option<Vec<Address> > >,
    chain!(
        maon: many_addr_or_nil,
        || {
            maon
        })
);

named!(pub env_subject,
    chain!(
        ns: nstring,
        || {
            ns
        })
);

named!(pub env_to <&[u8], Option<Vec<Address> > >,
    chain!(
        maon: many_addr_or_nil,
        || {
            maon
        })
);

named!(pub address <&[u8], Address>,
    chain!(
        char!('(') ~
        addr_name: nstring ~
        char!(' ') ~
        addr_adl: nstring ~
        char!(' ') ~
        addr_mailbox: nstring ~
        char!(' ') ~
        addr_host: nstring ~
        char!(')'),
        || {
            Address {
                addr_name: addr_name,
                addr_adl: addr_adl,
                addr_mailbox: addr_mailbox,
                addr_host: addr_host,
            }
        }
    )
);

named!(pub envelope <&[u8], Envelope>,
    chain!(
        char!('(') ~
        date: env_date ~
        char!(' ') ~
        subject: env_subject ~
        char!(' ') ~
        from: env_from ~
        char!(' ') ~
        senter: env_sender ~
        char!(' ') ~
        reply_to: env_reply_to ~
        char!(' ') ~
        to: env_to ~
        char!(' ') ~
        cc: env_cc ~
        char!(' ') ~
        bcc: env_bcc ~
        char!(' ') ~
        in_reply_to: env_in_reply_to ~
        char!(' ') ~
        message_id: env_message_id ~

        char!(')'),
        || {
            Envelope {
                date: date,
                subject: subject,
                from: from,
                senter: senter,
                reply_to: reply_to,
                to: to,
                cc: cc,
                bcc: bcc,
                in_reply_to: in_reply_to,
                message_id: message_id,
            }
        }
    )
);

#[cfg(test)]
mod tests {
    use nom::IResult::*;
    use nom::*;
    use super::*;
    use std::str;

    #[test]
    fn test_address_parse() {
        let addr = b"(\"name\" \"adl\" \"mailbox\" \"host\")";
        match address(&addr[..]) {
            Done(remaining, addr) => {
                assert_eq!(remaining.len(), 0);
                assert_eq!(addr.addr_name, b"name");
                assert_eq!(addr.addr_adl, b"adl");
                assert_eq!(addr.addr_mailbox, b"mailbox");
                assert_eq!(addr.addr_host, b"host");
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_atom() {
        let s = b"abcd";

        match atom(&s[..]) {
            Done(remaining, atom) => {
                assert_eq!(atom, s);
            }
            _ => panic!("NOT DONE"),
        }
    }
}
