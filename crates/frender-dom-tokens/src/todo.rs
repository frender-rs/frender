const fn dom_tokens_count(s: &str) -> usize {
    assert_ascii(s);

    let s = s.as_bytes();

    let mut count = 0;

    let mut i = 0;
    let mut current_is_token = false;

    while i < s.len() {
        if is_space_char(s[i]) {
            current_is_token = false;
        } else {
            if !current_is_token {
                count += 1;
                current_is_token = true;
            }
        }
        i += 1;
    }

    count
}

const fn separate_dom_tokens<const N: usize>(s: &str) -> [&'static str; N] {
    assert_ascii(s);

    let bytes = s.as_bytes();

    let mut res = [""; N];

    let mut i = 0;
    let mut cur_token_start = None::<usize>;
    let mut i_of_res = 0;

    while i < bytes.len() {
        if is_space_char(bytes[i]) {
            if let Some(cur_token_start) = cur_token_start {
                let token = &bytes[cur_token_start..i];
                res[i_of_res] = "";
            }
            cur_token_start = None;
        } else {
            if let Some(cur_token_start) = cur_token_start {
            } else {
            }
        }

        i += 1;
    }

    // loop {
    //     res[i] = token;
    //     i += 1;
    // }

    // std::str::from_utf8(s);

    res
}

#[cfg(test)]
mod tests {

    #[test]
    fn space_count() {
        assert_eq!(super::dom_tokens_count(""), 0);
        assert_eq!(super::dom_tokens_count(" "), 0);
        assert_eq!(super::dom_tokens_count("\r"), 0);
        assert_eq!(super::dom_tokens_count("  "), 0);
        assert_eq!(super::dom_tokens_count(" \r"), 0);

        assert_eq!(super::dom_tokens_count("a"), 1);
        assert_eq!(super::dom_tokens_count("a "), 1);
        assert_eq!(super::dom_tokens_count(" a"), 1);
        assert_eq!(super::dom_tokens_count(" a "), 1);

        assert_eq!(super::dom_tokens_count("a b"), 2);
        assert_eq!(super::dom_tokens_count("a  b"), 2);
        assert_eq!(super::dom_tokens_count(" a b"), 2);
        assert_eq!(super::dom_tokens_count("a b "), 2);
        assert_eq!(super::dom_tokens_count(" a b "), 2);
    }
}
