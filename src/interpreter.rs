use std::format;

enum token_kind {
    normal,
    kleene, // *
    single, // ?
    group_start, // [
    group_end // ]
}

struct token {
    kind: token_kind
    value: String
}

fn error(token: &str) {
    format!("Incorrect use of token: {}", token);
}
