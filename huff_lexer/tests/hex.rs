use huff_lexer::*;
use huff_utils::prelude::*;
use std::ops::Deref;

#[test]
fn parses_single_hex() {
    let source = "0xa57B";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source);

    // The first and only token should be lexed as Literal(0xa57B)
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Literal(str_to_bytes32("a57B")), Span::new(2..6, None)));
    assert_eq!(lexer.current_span().deref(), &Span::new(2..6, None));

    // We covered the whole source
    lexer.next();
    assert_eq!(lexer.current_span().end, source.len());
    assert!(lexer.eof);
}

#[test]
fn parses_bool() {
    let source = "false true";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source);

    // The first token should be lexed as a Literal representing 0x00
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Literal(str_to_bytes32("0")), Span::new(0..5, None)));
    assert_eq!(lexer.current_span().deref(), &Span::new(0..5, None));

    let _ = lexer.next(); // Whitespace

    // The second token should be lexed as a Literal representing 0x01
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Literal(str_to_bytes32("1")), Span::new(6..10, None)));
    assert_eq!(lexer.current_span().deref(), &Span::new(6..10, None));

    // We covered the whole source
    lexer.next();
    assert_eq!(lexer.current_span().end, source.len());
    assert!(lexer.eof);
}

#[test]
fn parses_odd_len_hex() {
    let source = "0x1";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source);

    // The first and only token should be lexed as Literal(0x1)
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Literal(str_to_bytes32("1")), Span::new(2..3, None)));
    assert_eq!(lexer.current_span().deref(), &Span::new(2..3, None));

    // We covered the whole source
    lexer.next();
    assert_eq!(lexer.current_span().end, source.len());
    assert!(lexer.eof);
}

// TODO: This doesn't exactly belong here.
#[test]
fn converts_literal_to_hex_string() {
    let sources = [
        "00",
        "01",
        "1000",
        "010101",
        "a57b",
        "8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925",
    ];

    for source in sources {
        assert_eq!(format!("0x{source}"), bytes32_to_string(&str_to_bytes32(source), true));
    }
}
