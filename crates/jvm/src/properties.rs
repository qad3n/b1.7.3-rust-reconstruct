use crate::io::{InputStream, IoResult};
use std::collections::HashMap;

// java.util.Properties for load and getProperty, keys and values decoded the way Properties.load(InputStream) does in java 6
#[derive(Default)]
pub struct Properties
{
    // lookup only, nothing iterates the table
    m_table: HashMap<String, String>,
}

fn isWhitespace(ch: char) -> bool { ch == ' ' || ch == '\t' || ch == '\u{c}' }

impl Properties
{
    pub fn new() -> Properties { Properties::default() }

    pub fn getProperty(&self, key: &str, defaultValue: &str) -> String
    {
        self.m_table.get(key).cloned().unwrap_or_else(|| defaultValue.to_string())
    }

    pub fn getPropertyOrNull(&self, key: &str) -> Option<String> { self.m_table.get(key).cloned() }

    pub fn setProperty(&mut self, key: &str, value: &str) { self.m_table.insert(key.to_string(), value.to_string()); }

    pub fn load<S: InputStream>(&mut self, input: &mut S) -> IoResult<()>
    {
        let mut bytes: Vec<u8> = Vec::new();
        let mut buffer: [u8; 8192] = [0; 8192];
        loop
        {
            let count: i32 = input.readInto(&mut buffer, 0, 8192)?;
            if count < 0
            {
                break;
            }
            bytes.extend_from_slice(&buffer[..count as usize]);
        }
        // the stream is read as ISO-8859-1, one char per byte
        let text: Vec<char> = bytes.iter().map(|&byte| byte as char).collect();
        for line in logicalLines(&text)
        {
            self.loadLine(&line)?;
        }
        Ok(())
    }

    fn loadLine(&mut self, line: &[char]) -> IoResult<()>
    {
        let limit: usize = line.len();
        let mut keyLen: usize = 0;
        let mut valueStart: usize = limit;
        let mut hasSep: bool = false;
        let mut precedingBackslash: bool = false;
        while keyLen < limit
        {
            let c: char = line[keyLen];
            if (c == '=' || c == ':') && !precedingBackslash
            {
                valueStart = keyLen + 1;
                hasSep = true;
                break;
            }
            else if isWhitespace(c) && !precedingBackslash
            {
                valueStart = keyLen + 1;
                break;
            }
            if c == '\\'
            {
                precedingBackslash = !precedingBackslash;
            }
            else
            {
                precedingBackslash = false;
            }
            keyLen += 1;
        }
        while valueStart < limit
        {
            let c: char = line[valueStart];
            if !isWhitespace(c)
            {
                if !hasSep && (c == '=' || c == ':')
                {
                    hasSep = true;
                }
                else
                {
                    break;
                }
            }
            valueStart += 1;
        }
        let key: String = loadConvert(&line[..keyLen])?;
        let value: String = loadConvert(&line[valueStart.min(limit)..])?;
        self.m_table.insert(key, value);
        Ok(())
    }
}

// splits into logical lines like Properties.LineReader, dropping comments, blank lines and leading whitespace, joining continuations
fn logicalLines(text: &[char]) -> Vec<Vec<char>>
{
    let mut lines: Vec<Vec<char>> = Vec::new();
    let mut current: Vec<char> = Vec::new();
    let mut skipWhiteSpace: bool = true;
    let mut isCommentLine: bool = false;
    let mut isNewLine: bool = true;
    let mut appendedLineBegin: bool = false;
    let mut precedingBackslash: bool = false;
    let mut skipLF: bool = false;
    let mut index: usize = 0;
    while index < text.len()
    {
        let c: char = text[index];
        index += 1;
        if skipLF
        {
            skipLF = false;
            if c == '\n'
            {
                continue;
            }
        }
        if skipWhiteSpace
        {
            if c == ' ' || c == '\t' || c == '\u{c}'
            {
                continue;
            }
            if !appendedLineBegin && (c == '\r' || c == '\n')
            {
                continue;
            }
            skipWhiteSpace = false;
            appendedLineBegin = false;
        }
        if isNewLine
        {
            isNewLine = false;
            if c == '#' || c == '!'
            {
                isCommentLine = true;
                continue;
            }
        }
        if c != '\n' && c != '\r'
        {
            current.push(c);
            if c == '\\'
            {
                precedingBackslash = !precedingBackslash;
            }
            else
            {
                precedingBackslash = false;
            }
        }
        else
        {
            if isCommentLine || current.is_empty()
            {
                isCommentLine = false;
                isNewLine = true;
                skipWhiteSpace = true;
                current.clear();
                continue;
            }
            if precedingBackslash
            {
                current.pop();
                skipWhiteSpace = true;
                appendedLineBegin = true;
                precedingBackslash = false;
                if c == '\r'
                {
                    skipLF = true;
                }
            }
            else
            {
                lines.push(std::mem::take(&mut current));
                isNewLine = true;
                skipWhiteSpace = true;
            }
        }
    }
    if !isCommentLine && !current.is_empty()
    {
        if precedingBackslash
        {
            current.pop();
        }
        lines.push(current);
    }
    lines
}

fn loadConvert(chars: &[char]) -> IoResult<String>
{
    let mut out: Vec<u16> = Vec::with_capacity(chars.len());
    let mut index: usize = 0;
    while index < chars.len()
    {
        let mut c: char = chars[index];
        index += 1;
        if c == '\\'
        {
            c = chars.get(index).copied().unwrap_or('\0');
            index += 1;
            if c == 'u'
            {
                let mut value: u32 = 0;
                for _ in 0..4
                {
                    let digit: char = chars.get(index).copied().unwrap_or('\0');
                    index += 1;
                    match digit.to_digit(16)
                    {
                        Some(nibble) => value = (value << 4) + nibble,
                        None =>
                        {
                            crate::throwable::throw(&crate::throwable::ILLEGAL_ARGUMENT, "Malformed \\uxxxx encoding.")
                        }
                    }
                }
                out.push(value as u16);
                continue;
            }
            c = match c
            {
                't' => '\t',
                'r' => '\r',
                'n' => '\n',
                'f' => '\u{c}',
                other => other,
            };
        }
        out.push(c as u16);
    }
    Ok(String::from_utf16_lossy(&out))
}
