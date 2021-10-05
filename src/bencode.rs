use std::collections::HashMap;
use std::str::from_utf8;

use nom::{
    IResult,
    character::complete::{char, digit1},
    combinator::{map_res, opt, recognize, map},
    sequence::{preceded, terminated, pair},
    branch::alt,
    bytes::complete::take,
    multi::many1,
};


/// Bencode value representation class (ADT)
/// ## Constructors
/// You can parse byte array (`&[u8]`) of Bencoded object using static `from_bytes` fabric
/// method.
/// 
/// You can parse string of Bencoded object using static `from_string` fabric method.
/// ## Methods
/// You can convert `BValue` to any of variants using methods:
/// - `get_number` -> `BNumber`
/// - `get_bytes` -> `BBytes`
/// - `get_list` -> `BList`
/// - `get_dict` -> `BDict`
/// - and `get_string` to convert bytes directly to string representation (`&str`)
/// ## ADT variants
/// ### `BNumber`
/// Simple `i64` value.
/// ### `BBytes`
/// Vector of `u8` values.
/// ### `BList`
/// Vector with some `BValue` elements.
/// ### `BDict`
/// `HashMap` with string keys and `BValue` values.
#[derive(Debug,PartialEq)]
pub enum BValue {
    BNumber(i64),
    BBytes(Vec<u8>),
    BList(Vec<BValue>),
    BDict(HashMap<String, BValue>),
}


impl BValue {
    /// Parse array of Bencode bytes to `BValue` object
    /// ## Arguments
    /// - `i` bytes of bencode object
    /// ## Example
    /// ```rust
    /// use torcode::bencode::BValue;
    /// assert_eq!(BValue::from_bytes(&b"i3e"[..]), Ok((&b""[..], BValue::BNumber(3))));
    /// ```
    pub fn from_bytes(i: &[u8]) -> IResult<&[u8], BValue> {
        let bnumber = map(parse_number, BValue::BNumber);
        let bbytes = map(parse_bytes, BValue::BBytes);
        let blist = map(parse_list, BValue::BList);
        let bdict = map(parse_dict, BValue::BDict);
        alt((bnumber, bbytes, blist, bdict))(i)
    }


    /// Parse string of Bencode to `BValue` object
    /// ## Arguments
    /// - `s` string, which represent Bencoded object to parse
    /// ## Example
    /// ```rust
    /// use torcode::bencode::BValue;
    /// assert_eq!(BValue::from_string("i3228e"), Ok((&b""[..], BValue::BNumber(3228))));
    /// ```
    pub fn from_string(s: &str) -> IResult<&[u8], BValue> {
        BValue::from_bytes(s.as_bytes())
    }


    /// Returns `i64` value if object has `BNumber` type, else `None`
    pub fn get_number(&self) -> Option<&i64> {
        match self {
            BValue::BNumber(n) => Some(n),
            _ => None
        }
    }


    /// Returns byte vector (`Vec<u8>`) if object has `BBytes` type, else `None`
    pub fn get_bytes(&self) -> Option<&Vec<u8>> {
        match self {
            BValue::BBytes(bytes) => Some(bytes),
            _ => None
        }
    }


    /// Returns list of `BValue`s if object has `BList` type, else `None`
    pub fn get_list(&self) -> Option<&Vec<BValue>> {
        match self {
            BValue::BList(list) => Some(list),
            _ => None
        }
    }


    /// Returns dictionary `String`->`BValue` if object has `BDict` type, else `None`
    pub fn get_dict(&self) -> Option<&HashMap<String, BValue>> {
        match self {
            BValue::BDict(map) => Some(map),
            _ => None
        }
    }


    /// Returns string representation of ASCII bytes if object has `BBytes` type, else `None`
    pub fn get_string(&self) -> Option<&str> {
        match self.get_bytes() {
            Some(bytes) => Some(from_utf8(bytes).unwrap()),
            _ => None
        }
    }
}


fn parse_number(i: &[u8]) -> IResult<&[u8], i64> {
    let signed_digit = recognize(pair(opt(char('-')), digit1));
    let parsed_num = map_res(signed_digit, |s: &[u8]| from_utf8(s).unwrap().parse::<i64>());

    terminated(preceded(char('i'), parsed_num), char('e'))(i)
}


fn parse_length(i: &[u8]) -> IResult<&[u8], usize> {
    let len = terminated(digit1, char(':'));
    map_res(len, |s: &[u8]| from_utf8(s).unwrap().parse::<usize>())(i)
}


fn parse_string(i: &[u8]) -> IResult<&[u8], String> {
    map_res(parse_bytes, String::from_utf8)(i)
}


fn parse_bytes(i: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let (left, len) = parse_length(i)?;
    let result = take(len);
    map(result, |s: &[u8]| s.to_vec())(left)
}


fn parse_list(i: &[u8]) -> IResult<&[u8], Vec<BValue>> {
    let values = many1(BValue::from_bytes);
    preceded(char('l'), terminated(values, char('e')))(i)
}


fn parse_dict(i: &[u8]) -> IResult<&[u8], HashMap<String, BValue>> {
    let kv = pair(parse_string, BValue::from_bytes);
    let kv = many1(kv);
    let kv = terminated(preceded(char('d'), kv), char('e'));
    map(kv, |s| s.into_iter().collect())(i)
}
