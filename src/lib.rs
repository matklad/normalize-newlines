pub struct IsolatedCR;

/// Replaces `\r\n` with `\n` in-place in `src`.
///
/// Returns error if there's a lone `\r` in the string
pub fn normalize_newlines(src: &mut String) -> Result<(), IsolatedCR> {
    if !src.as_bytes().contains(&b'\r') {
        return Ok(());
    }

    // We replace `\r\n` with `\n` in-place, which doesn't break utf-8 encoding.
    // While we *can* call `as_mut_vec` and do surgery on the live string
    // directly, let's rather steal the contents of `src`. This makes the code
    // safe even if a panic occurs.

    let mut buf = std::mem::replace(src, String::new()).into_bytes();
    let mut has_lone_cr = false;
    let mut gap_len = 0;
    let mut tail = buf.as_mut_slice();
    loop {
        let idx = match find_cr_nl(&mut has_lone_cr, &tail[gap_len..]) {
            None => tail.len(),
            Some(idx) => idx + gap_len,
        };
        tail.copy_within(gap_len..idx, 0);
        tail = &mut tail[idx - gap_len..];
        if tail.len() == gap_len {
            break;
        }
        gap_len += 1;
    }

    // Account for removed `\r`.
    // After `set_len`, `buf` is guaranteed to contain utf-8 again.
    let new_len = buf.len() - gap_len;
    unsafe {
        buf.set_len(new_len);
        *src = String::from_utf8_unchecked(buf);
    }

    return if has_lone_cr { Err(IsolatedCR) } else { Ok(()) };

    fn find_cr_nl(has_lone_cr: &mut bool, src: &[u8]) -> Option<usize> {
        let mut search_idx = 0;
        while let Some(idx) = find_cr(&src[search_idx..]) {
            if src[search_idx..].get(idx + 1) != Some(&b'\n') {
                *has_lone_cr = true;
                search_idx += idx + 1;
                continue;
            }
            return Some(search_idx + idx);
        }
        None
    }

    fn find_cr(src: &[u8]) -> Option<usize> {
        src.iter()
            .enumerate()
            .find_map(|(idx, &b)| if b == b'\r' { Some(idx) } else { None })
    }
}
