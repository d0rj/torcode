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


    #[allow(dead_code)]
    pub fn get_number(&self) -> &i64 {
        match self {
            BValue::BNumber(n) => n,
            _ => panic!("Failed to parse number")
        }
    }


    #[allow(dead_code)]
    pub fn get_bytes(&self) -> &Vec<u8> {
        match self {
            BValue::BBytes(bytes) => bytes,
            _ => panic!("Failed to parse bytes")
        }
    }


    #[allow(dead_code)]
    pub fn get_list(&self) -> &Vec<BValue> {
        match self {
            BValue::BList(list) => list,
            _ => panic!("Failed to parse list")
        }
    }


    #[allow(dead_code)]
    pub fn get_dict(&self) -> &HashMap<String, BValue> {
        match self {
            BValue::BDict(map) => map,
            _ => panic!("Failed to parse dict")
        }
    }


    #[allow(dead_code)]
    pub fn get_string(&self) -> &str {
        from_utf8(self.get_bytes()).unwrap()
    }
}


#[allow(dead_code)]
fn parse_number(i: &[u8]) -> IResult<&[u8], i64> {
    let signed_digit = recognize(pair(opt(char('-')), digit1));
    let parsed_num = map_res(signed_digit, |s: &[u8]| from_utf8(s).unwrap().parse::<i64>());

    terminated(preceded(char('i'), parsed_num), char('e'))(i)
}


#[allow(dead_code)]
fn parse_length(i: &[u8]) -> IResult<&[u8], usize> {
    let len = terminated(digit1, char(':'));
    map_res(len, |s: &[u8]| from_utf8(s).unwrap().parse::<usize>())(i)
}


#[allow(dead_code)]
fn parse_string(i: &[u8]) -> IResult<&[u8], String> {
    map_res(parse_bytes, String::from_utf8)(i)
}


#[allow(dead_code)]
fn parse_bytes(i: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let (left, len) = parse_length(i)?;
    let result = take(len);
    map(result, |s: &[u8]| s.to_vec())(left)
}


#[allow(dead_code)]
fn parse_list(i: &[u8]) -> IResult<&[u8], Vec<BValue>> {
    let values = many1(BValue::from_bytes);
    preceded(char('l'), terminated(values, char('e')))(i)
}


#[allow(dead_code)]
fn parse_dict(i: &[u8]) -> IResult<&[u8], HashMap<String, BValue>> {
    let kv = pair(parse_string, BValue::from_bytes);
    let kv = many1(kv);
    let kv = terminated(preceded(char('d'), kv), char('e'));
    map(kv, |s| s.into_iter().collect())(i)
}
