use std::format;
use std::fs;

const TM = &[
    // normal
    [
        token_kind::normal,
        token_kind::kleene,
        token_kind::single,
        token_kind::group_start,
        parse_error::no_group_start,
    ],
    // kleene
    [
        token_kind::normal,
        token_kind::kleene,
        token_kind::kleene,
        token_kind::group_start,
        parse_error::no_group_start,
    ],
    // single
    [
        token_kind::normal,
        token_kind::kleene,
        token_kind::single,
        token_kind::group_start,
        parse_error::no_group_start,
    ],
    // group_start
    [
        token_kind::group_start,
        token_kind::group_start,
        token_kind::group_start,
        parse_error::no_group_nest,
        token_kind::group_end,
    ],
    // group_end
    [
        token_kind::normal,
        token_kind::kleene,
        token_kind::single,
        token_kind::group_start,
        parse_error::no_group_start,
    ],
];

const ERR_MSGS = &[
    "No start to group ended with \"]\".", // no_group_start
    "Cannot nest groups defined by brackets.", // no_group_nest
];

enum token_kind {
    normal,
    kleene, // *
    single, // ?
    group_start, // [
    group_end, // ]
    //pipe, // |
}

enum parse_error {
    no_group_start,
    no_group_nest
}

struct token {
    kind: token_kind
    value: char
}

/**
 * '*' matches any number of normal characters between starting and ending normal characters in
 * an expression.  Reading a Kleene operator will override other wildcards and simply match all,
 * not transitioning state until a normal character is read.
 * '?' matches any single character between start and end normal characters
 * '[...]' matches any single character from the characters listed between square brackets and
 * interprets them literally (wildcards '?' and '*' are read as their ASCII equivalents)
*/
fn get_kind(ch: char) -> token_kind {
    match ch {
        "*" => token_kind::kleene
        "?" => token_kind::single
        "[" => token_kind::group_start
        "]" => token_kind::group_end
        _ => token_kind::normal
    }
}

fn eat(ch: char) -> token {
    token { get_kind(ch), ch }
}
    
//fn parse_arg(arg: &str) -> token {
//    let curr: token_kind = token_kind::normal; 
//
//    for ch in arg.chars() {
//        let next: token = eat(ch);
//
//    }
//}

fn error(token: &str) {
    format!("Incorrect use of token: {}", token);
}
