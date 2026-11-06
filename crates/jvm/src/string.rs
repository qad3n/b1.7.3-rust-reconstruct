use crate::locale::{self, Charset};
use crate::{character, special_casing};
use std::fmt;

#[derive(Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JString
{
    m_chars: Vec<u16>,
}

impl JString
{
    pub fn new() -> Self { Self { m_chars: Vec::new() } }

    pub fn fromUtf16(chars: Vec<u16>) -> Self { Self { m_chars: chars } }

    pub fn chars(&self) -> &[u16] { &self.m_chars }

    pub fn length(&self) -> i32 { self.m_chars.len() as i32 }

    pub fn charAt(&self, index: i32) -> u16
    {
        if index < 0 || index >= self.length()
        {
            crate::throwable::throw(&crate::throwable::STRING_INDEX, &format!("String index out of range: {index}"));
        }
        self.m_chars[index as usize]
    }

    pub fn hashCode(&self) -> i32
    {
        let mut hash: i32 = 0;
        for &ch in &self.m_chars
        {
            hash = hash.wrapping_mul(31).wrapping_add(ch as i32);
        }
        hash
    }

    pub fn substring(&self, begin: i32, end: i32) -> JString
    {
        if begin < 0 || end > self.length() || begin > end
        {
            crate::throwable::throw(
                &crate::throwable::STRING_INDEX,
                &format!(
                    "String index out of range: {}",
                    if begin < 0
                    {
                        begin
                    }
                    else if end > self.length()
                    {
                        end
                    }
                    else
                    {
                        end - begin
                    }
                ),
            );
        }
        JString::fromUtf16(self.m_chars[begin as usize..end as usize].to_vec())
    }

    // string concatenation of a possibly null reference
    pub fn display(value: Option<&JString>) -> String
    {
        match value
        {
            Some(text) => text.to_string(),
            None => "null".to_string(),
        }
    }
}

impl From<&str> for JString
{
    fn from(text: &str) -> Self { Self { m_chars: text.encode_utf16().collect() } }
}

impl fmt::Display for JString
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let text: String = char::decode_utf16(self.m_chars.iter().copied())
            .map(|decoded| decoded.unwrap_or(char::REPLACEMENT_CHARACTER))
            .collect();
        formatter.write_str(&text)
    }
}

impl fmt::Debug for JString
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { write!(formatter, "{:?}", self.to_string()) }
}

// String.toLowerCase, toUpperCase and getBytes of jdk 6
impl JString
{
    // regionMatches with ignoreCase compares each char pair through upper and then lower case
    pub fn equalsIgnoreCase(&self, other: Option<&JString>) -> bool
    {
        let Some(other) = other
        else
        {
            return false;
        };
        self.m_chars.len() == other.m_chars.len()
            && self.m_chars.iter().zip(&other.m_chars).all(|(&a, &b)| {
                let upperA: i32 = character::toUpperCase(a as i32) as u16 as i32;
                let upperB: i32 = character::toUpperCase(b as i32) as u16 as i32;
                a == b
                    || upperA == upperB
                    || character::toLowerCase(upperA) as u16 == character::toLowerCase(upperB) as u16
            })
    }

    pub fn toLowerCase(&self) -> JString { self.toLowerCaseIn(locale::defaultLanguage()) }

    pub fn toUpperCase(&self) -> JString { self.toUpperCaseIn(locale::defaultLanguage()) }

    // toLowerCase(Locale) for a locale with this language
    pub fn toLowerCaseIn(&self, language: &str) -> JString
    {
        let localeDependent: bool = isLocaleDependent(language);
        self.mapCase(|chars, index, codePoint| {
            let lowerChar: i32 = if localeDependent || codePoint == GREEK_CAPITAL_SIGMA
            {
                special_casing::toLowerCaseEx(chars, index, language)
            }
            else
            {
                character::toLowerCase(codePoint)
            };
            if lowerChar == character::ERROR
            {
                special_casing::toLowerCaseCharArray(chars, index, language)
            }
            else
            {
                character::toChars(lowerChar)
            }
        })
    }

    // toUpperCase(Locale) for a locale with this language
    pub fn toUpperCaseIn(&self, language: &str) -> JString
    {
        let localeDependent: bool = isLocaleDependent(language);
        self.mapCase(|chars, index, codePoint| {
            let upperChar: i32 = if localeDependent
            {
                special_casing::toUpperCaseEx(chars, index, language)
            }
            else
            {
                character::toUpperCaseEx(codePoint)
            };
            match (upperChar == character::ERROR, localeDependent)
            {
                (true, true) => special_casing::toUpperCaseCharArray(chars, index, language),
                (true, false) => character::toUpperCaseCharArray(codePoint),
                (false, _) => character::toChars(upperChar),
            }
        })
    }

    // java reads a surrogate pair as one code point only when it starts on the high surrogate
    fn mapCase(&self, mut map: impl FnMut(&[u16], usize, i32) -> Vec<u16>) -> JString
    {
        let chars: &[u16] = &self.m_chars;
        let mut result: Vec<u16> = Vec::with_capacity(chars.len());
        let mut index: usize = 0;
        while index < chars.len()
        {
            let codePoint: i32 = if character::isHighSurrogate(chars[index])
            {
                character::codePointAt(chars, index)
            }
            else
            {
                chars[index] as i32
            };
            result.extend(map(chars, index, codePoint));
            index += character::charCount(codePoint);
        }
        JString::fromUtf16(result)
    }

    // getBytes() in the platform charset, an unmappable or lone surrogate code point becomes one '?'
    pub fn getBytes(&self) -> Vec<u8>
    {
        let charset: Charset = locale::platformCharset();
        let chars: &[u16] = &self.m_chars;
        let mut bytes: Vec<u8> = Vec::with_capacity(chars.len());
        let mut index: usize = 0;
        while index < chars.len()
        {
            let codePoint: i32 = character::codePointAt(chars, index);
            index += character::charCount(codePoint);
            let surrogate: bool = (0xD800..=0xDFFF).contains(&codePoint);
            match charset
            {
                Charset::Utf8 if !surrogate =>
                {
                    let ch: char = char::from_u32(codePoint as u32).expect("a non surrogate code point is a char");
                    bytes.extend_from_slice(ch.encode_utf8(&mut [0; 4]).as_bytes());
                }
                Charset::Latin1 if codePoint <= 0xFF => bytes.push(codePoint as u8),
                Charset::Ascii if codePoint <= 0x7F => bytes.push(codePoint as u8),
                _ => bytes.push(b'?'),
            }
        }
        bytes
    }

    // the name java hands the operating system for a file path made from this string
    #[cfg(unix)]
    pub fn toOsString(&self) -> std::ffi::OsString
    {
        use std::os::unix::ffi::OsStringExt;
        std::ffi::OsString::from_vec(self.getBytes())
    }

    #[cfg(not(unix))]
    pub fn toOsString(&self) -> std::ffi::OsString { std::ffi::OsString::from(self.to_string()) }

    pub fn toPathBuf(&self) -> std::path::PathBuf { std::path::PathBuf::from(self.toOsString()) }
}

const GREEK_CAPITAL_SIGMA: i32 = 0x03A3;

fn isLocaleDependent(language: &str) -> bool { matches!(language, "tr" | "az" | "lt") }
