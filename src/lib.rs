pub mod bencode;

pub use bencode::parse;
pub use bencode::BValue;


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;


    #[test]
    fn test_parse_number() {
        assert_eq!(parse(&b"i3228e"[..]), Ok((&b""[..], BValue::BNumber(3228))));
        assert_eq!(parse(&b"i-3228e"[..]), Ok((&b""[..], BValue::BNumber(-3228))));
    }


    #[test]
    fn test_parse_bytes() {
        assert_eq!(
            parse(&b"12:Hello World!"[..]),
            Ok((&b""[..], BValue::BBytes("Hello World!".as_bytes().to_vec())))
        );
    }


    #[test]
    fn test_parse_list() {
        let expected = BValue::BList(
            vec![
                BValue::BBytes("spam".as_bytes().to_vec()),
                BValue::BBytes("eggs".as_bytes().to_vec())
            ]
        );
        assert_eq!(parse(&b"l4:spam4:eggse"[..]), Ok((&b""[..], expected)));
    }


    #[test]
    fn test_parse_dict() {
        let mut expected: HashMap<String, BValue> = HashMap::new();
        expected.entry("cow".to_string()).or_insert(BValue::BBytes("moo".as_bytes().to_vec()));
        expected.entry("spam".to_string()).or_insert(BValue::BBytes("eggs".as_bytes().to_vec()));

        assert_eq!(parse(&b"d3:cow3:moo4:spam4:eggse"[..]), Ok((&b""[..], BValue::BDict(expected))));
    }
}
