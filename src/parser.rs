use nom::IResult::*;
use nom::*;


#[derive(Debug)]
pub struct Address {
    addr_name: Vec<u8>,
    addr_adl: Vec<u8>,
    addr_mailbox: Vec<u8>,
    addr_host: Vec<u8>,
}

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

named!(pub nstring,
       alt!(
           delimited!(
               char!('"'),
               is_not!("\""),
               char!('"')
           )
           | tag!(b"nil")
       )
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
                addr_name: Vec::from(addr_name),
                addr_adl: Vec::from(addr_adl),
                addr_mailbox: Vec::from(addr_mailbox),
                addr_host: Vec::from(addr_host),
            }
        }
    ));


// named!(address, delimited!(
//    char!('('),
//    ,
//    char!(')')
//    )
// );

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_parse() {
        let addr = b"\"hello\"";
        println!("{:?}", nstring(&addr[..]));
    }
}
