use super::BufferedNonAsciiWsPreprocessedInputStream;

const _: () = {
    assert!(BufferedNonAsciiWsPreprocessedInputStream::from_str(" ")
        .get_next()
        .is_none());

    assert!(BufferedNonAsciiWsPreprocessedInputStream::from_str("  ")
        .get_next()
        .is_none());

    assert!(
        BufferedNonAsciiWsPreprocessedInputStream::from_str(" \r \r\n \n")
            .get_next()
            .is_none()
    );
    {
        let mut s = BufferedNonAsciiWsPreprocessedInputStream::from_str(" id");
        assert!(matches!(
            s.next(),
            Some(ch) if ch.to_char() == 'i'
        ));
        assert!(matches!(
            s.next(),
            Some(ch) if ch.to_char() == 'd'
        ));
        assert!(s.next().is_none());
    }

    {
        let s = BufferedNonAsciiWsPreprocessedInputStream::from_str(" ");
        let s = s.into_ascii_ws();
        assert!(s.as_full().is_none());
        assert!(s.get_next().is_none())
    }

    {
        let s = BufferedNonAsciiWsPreprocessedInputStream::from_str(" id");
        let mut s = s.into_ascii_ws();

        assert!(matches!(
            s.as_full(),
            Some(full) if matches!(full.to_str_trim_trailing_option(None).as_bytes(), b"id")
        ));

        assert!(matches!(
            s.next(),
            Some(ch) if ch.to_char() == 'i'
        ));
        assert!(matches!(
            s.next(),
            Some(ch) if ch.to_char() == 'd'
        ));
        assert!(s.next().is_none());
    }
};
