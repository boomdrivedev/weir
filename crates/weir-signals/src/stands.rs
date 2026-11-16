//! The one check of a reader's excerpt: it stands only as one unbroken run
//! of one raw file's text, every `\r\n` evened to `\n` in the file and the
//! excerpt both, beginning on the line of that file its position names as
//! `<path>:<line>` (26; core.16; record 0010). [`InboxAt::read_capture`]
//! calls it for each signal of a reading whose fields hold, and nothing
//! else does: a capture's own signal and an imported signal are not looked
//! for in raw material, and their positions stay free text.
//!
//! The check compares bytes and reads no meaning from them (core.4): no
//! whitespace or letter's case is evened, a lone `\r` stays as it is, and
//! nothing is decoded.
//!
//! [`InboxAt::read_capture`]: crate::InboxAt::read_capture

use weir_capture::RawFile;

/// `Ok` where `excerpt` is one unbroken run of the text of the raw file
/// `position` names, beginning on the line it names, line endings evened.
/// Otherwise the field to name, `position` or `excerpt`, and why, for the
/// first that fails: a position spelled otherwise than `<path>:<line>`, one
/// naming none of `raw`, one naming a line past its file's last, and an
/// excerpt that does not stand there.
pub(crate) fn stands(
    excerpt: &str,
    position: &str,
    raw: &[RawFile],
) -> Result<(), (&'static str, String)> {
    let Some((path, line)) = read_position(position) else {
        return Err((
            "position",
            format!("is `{position}`, which names no raw file and line as `<path>:<line>`"),
        ));
    };
    let Some(file) = raw.iter().find(|file| file.path == path) else {
        return Err((
            "position",
            format!("names `{path}`, which is none of the capture's raw files"),
        ));
    };
    let text = evened(&file.bytes);
    let Some((from, to)) = line_span(&text, line) else {
        return Err((
            "position",
            format!("names line {line}, past the end of `{path}`"),
        ));
    };
    let excerpt = evened(excerpt.as_bytes());
    if (from..to).any(|offset| text[offset..].starts_with(&excerpt)) {
        Ok(())
    } else {
        Err((
            "excerpt",
            format!(
                "does not stand in `{path}` as one run beginning on line {line}, \
                 line endings evened"
            ),
        ))
    }
}

/// The path and the line `position` names, read at its last `:`: the path
/// not empty, the line one or more ASCII digits, the first not `0`, that
/// read as a `usize`. None for any other position. The last `:` lets a
/// path hold one, as `meeting:12/transcript.txt:2` does.
fn read_position(position: &str) -> Option<(&str, usize)> {
    let (path, line) = position.rsplit_once(':')?;
    let well_spelled = !path.is_empty()
        && line.bytes().all(|byte| byte.is_ascii_digit())
        && line.bytes().next().is_some_and(|first| first != b'0');
    if !well_spelled {
        return None;
    }
    Some((path, line.parse().ok()?))
}

/// `bytes` with every `\r\n` evened to `\n`; a lone `\r` stays.
fn evened(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bytes.len());
    let mut rest = bytes;
    while let Some((&byte, after)) = rest.split_first() {
        if byte == b'\r' && after.first() == Some(&b'\n') {
            out.push(b'\n');
            rest = &after[1..];
        } else {
            out.push(byte);
            rest = after;
        }
    }
    out
}

/// The offsets of `text` an excerpt beginning on line `line` may begin at:
/// from the line's first byte up to the next line's first byte, or the
/// text's end. Line 1 begins at the first byte, and each later line just
/// after a `\n` that is not the text's last byte, so `a\nb\n` and `a\nb`
/// both have two lines and an empty text has none. None for a line past
/// the last.
fn line_span(text: &[u8], line: usize) -> Option<(usize, usize)> {
    let mut from = 0;
    for _ in 1..line {
        from += text.get(from..)?.iter().position(|&byte| byte == b'\n')? + 1;
    }
    if from >= text.len() {
        return None;
    }
    let to = text[from..]
        .iter()
        .position(|&byte| byte == b'\n')
        .map_or(text.len(), |at| from + at + 1);
    Some((from, to))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_position_reads_at_its_last_colon() {
        assert_eq!(read_position("a.txt:2"), Some(("a.txt", 2)));
        assert_eq!(read_position("m:12/a.txt:10"), Some(("m:12/a.txt", 10)));
        for spelled_otherwise in [
            "a.txt",
            ":2",
            "a.txt:",
            "a.txt:0",
            "a.txt:02",
            "a.txt:+2",
            "a.txt:2a",
            "a.txt: 2",
            "a.txt:99999999999999999999999",
        ] {
            assert_eq!(
                read_position(spelled_otherwise),
                None,
                "{spelled_otherwise}"
            );
        }
    }

    #[test]
    fn line_endings_are_evened_and_a_lone_return_kept() {
        assert_eq!(evened(b"a\r\nb\r\n"), b"a\nb\n");
        assert_eq!(evened(b"a\rb\r\r\n"), b"a\rb\r\n");
        assert_eq!(evened(b""), b"");
    }

    #[test]
    fn lines_are_counted_from_one() {
        assert_eq!(line_span(b"a\nb\n", 1), Some((0, 2)));
        assert_eq!(line_span(b"a\nb\n", 2), Some((2, 4)));
        assert_eq!(line_span(b"a\nb\n", 3), None);
        assert_eq!(line_span(b"a\nb", 2), Some((2, 3)));
        assert_eq!(line_span(b"a\nb", 3), None);
        assert_eq!(line_span(b"a\n\nb", 2), Some((2, 3)));
        assert_eq!(line_span(b"", 1), None);
        assert_eq!(line_span(b"\n", 1), Some((0, 1)));
        assert_eq!(line_span(b"\n", 2), None);
    }
}
