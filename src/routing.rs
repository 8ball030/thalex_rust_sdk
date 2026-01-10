#[inline(always)]
pub fn extract_id(bytes: &[u8]) -> Option<u64> {
    // Quick check: does it start with "{"id":"
    if !bytes.starts_with(b"{\"id\":") {
        return None;
    }

    // Start parsing after `"id":`
    let mut i = 6; // skip {"id":
    let start = i;

    // parse digits
    while let Some(&b) = bytes.get(i) {
        if !b.is_ascii_digit() {
            break;
        }
        i += 1;
    }

    std::str::from_utf8(&bytes[start..i]).ok()?.parse().ok()
}

#[inline(always)]
pub fn extract_channel(bytes: &[u8]) -> Option<&str> {
    if !bytes.starts_with(b"{\"channel_name\":\"") {
        return None;
    }
    let mut i = 17;
    let start = i;

    while i < bytes.len() {
        let b = bytes[i];
        if b == b'"' {
            break;
        }
        i += 1;
    }

    unsafe { Some(std::str::from_utf8_unchecked(&bytes[start..i])) }
}
