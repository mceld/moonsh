use std::collections::HashSet;
use std::path::Path;
use regex::Regex;
use walkdir::WalkDir;

/**
 * Transition matrix defines tokenization scheme
 *
 * States and their numbers:
 * 0: Normal
 * 1: Kleene
 * 2: Single
 * 3: GroupStart
 * 4: GroupEnd
 * 5: Group
 * E: ParseError (of some kind)
 *
 * Character types (correspond to enum values):
 * a: normal (any character other than defined wildcards)
 * *: kleene star (regex '.*')
 * ?: single character (regex '.')
 * [: opening square bracket
 * ]: closing square bracket
 *
 * Unclosed group error (i.e. [, [abc) handled by 'parse_arg'
 *
 *   a * ? [ ]
 * 0 0 1 2 3 E
 * 1 0 1 2 3 E
 * 2 0 1 2 3 E
 * 3 5 5 5 E 4
 * 4 0 1 2 3 E
 * 5 5 5 5 5 4
 *
 */
const TM: [[Result<TokenKind, ParseError>; 5]; 6] = [
  /* Outer lists are states, inner lists are character types given by get_kind */
  // Normal
  [
    Ok(TokenKind::Normal),
    Ok(TokenKind::Kleene),
    Ok(TokenKind::Single),
    Ok(TokenKind::GroupStart),
    Err(ParseError::NoGroupStart),
  ],
  // Kleene
  [
    Ok(TokenKind::Normal),
    Ok(TokenKind::Kleene),
    Ok(TokenKind::Single),
    Ok(TokenKind::GroupStart),
    Err(ParseError::NoGroupStart),
  ],
  // Single
  [
    Ok(TokenKind::Normal),
    Ok(TokenKind::Kleene),
    Ok(TokenKind::Single),
    Ok(TokenKind::GroupStart),
    Err(ParseError::NoGroupStart),
  ],
  // GroupStart
  [
    Ok(TokenKind::Group),
    Ok(TokenKind::Group),
    Ok(TokenKind::Group),
    Err(ParseError::NoGroupNest),
    Ok(TokenKind::GroupEnd),
  ],
  // GroupEnd
  [
    Ok(TokenKind::Normal),
    Ok(TokenKind::Kleene),
    Ok(TokenKind::Single),
    Ok(TokenKind::GroupStart),
    Err(ParseError::NoGroupStart),
  ],
  // Group
  [
    Ok(TokenKind::Group),
    Ok(TokenKind::Group),
    Ok(TokenKind::Group),
    Ok(TokenKind::Group),
    Ok(TokenKind::GroupEnd),
  ],
];

const ERR_MSGS: [&'static str; 3] = [
  "No start to group ended with \"]\".", // NoGroupStart
  "Cannot nest groups.", // NoGroupNest
  "Group has no matching \"]\".", // GroupNotEnded
];

#[derive(PartialEq, Copy, Clone, Debug)]
enum TokenKind {
  Normal,
  Kleene, // *
  Single, // ?
  GroupStart, // [
  GroupEnd, // ]
  Group,
  // Pipe, // |
}

#[derive(Copy, Clone)]
enum ParseError {
  NoGroupStart,
  NoGroupNest,
  GroupNotEnded,
}

#[derive(Debug)]
pub struct Token {
  kind: TokenKind,
  value: String,
}

impl ToString for Token {
  fn to_string(&self) -> String {
    self.value.to_string()
  }
}

fn get_kind(ch: char) -> TokenKind {
  match ch {
    '*' => { TokenKind::Kleene }
    '?' => { TokenKind::Single }
    '[' => { TokenKind::GroupStart }
    ']' => { TokenKind::GroupEnd }
    _ => { TokenKind::Normal }
  }
}

pub fn parse_arg(arg: &str) -> Result<Vec<Token>, &'static str> {
  let mut tokens: Vec<Token> = Vec::<Token>::new();
  let mut token_buf: String = String::new();
  let mut curr: TokenKind = TokenKind::Normal;
  let length: usize = arg.len();
  let arg_vec: Vec<char> = arg.chars().collect();

  for ch in 0..length {
    let next: TokenKind = get_kind(arg_vec[ch]);
    let transition = &TM[curr as usize][next as usize]; // Transition based on current and next

    match transition {
      Err(err) => {
        return Err(ERR_MSGS[*err as usize]);
      }
      Ok(val) => {
        // If the token type is changing
        if curr != *val {
          // Create a token with the current token type and buffer and add it to 'tokens'
          tokens.push(Token { kind: curr, value: token_buf.clone() });
          token_buf = "".to_string(); // Reset the buffer
          curr = *val;
        }
        // Add to continue the previous token or start a new one
        token_buf.push(arg_vec[ch]);
      }
    }
    
    // If ending on a Group state - return an Error
    if ch == (length - 1) {
      match transition {
        Ok(val) => {
          match val {
            TokenKind::GroupStart | TokenKind::Group => {
              return Err(ERR_MSGS[ParseError::GroupNotEnded as usize]);
            }
            _ => {}
          }
        }
        _ => {} // Errors should be filtered by the match above
      }
    }
  }

  tokens.push(Token { kind: curr, value: token_buf.clone() });

  Ok(tokens)
}

pub fn build_regex(token_lists: Vec<Vec<Token>>) -> Vec<String> {
  let mut re_vec: Vec<String> = Vec::new();

  // list of list of tokens
  for list in token_lists {

    // build a regex out of the list of tokens
    let mut re: String = String::new();

    for tok in list {

      match tok.kind {

        TokenKind::Normal => {
          // TODO if its a regex special character - escape it...
          // does not apply to alphabetic characters
          // . + * ? ^ $ ( ) [ ] { } | \
          re.push_str(&tok.value);
        }

        TokenKind::Kleene => {
          re.push_str(".*");
        }

        TokenKind::Single => {
          re.push_str(".");
        }

        TokenKind::Group => {
          let char_vec: Vec<char> = tok.value.chars().collect();
          let mut char_set: HashSet<char> = HashSet::new();

          for item in char_vec {
            char_set.insert(item);
          }

          let set_size: usize = char_set.len();
          let mut iter: usize = 0;

          re.push_str("(");
          for ch in char_set {
            iter += 1;
            re.push(ch); // push a char
            if iter != set_size {
              re.push_str("|");
            }
          }
          re.push_str(")");
        }

        _ => {} // GroupStart + GroupEnd can be skipped
      }
    }
    re_vec.push(re);
  }
  re_vec
}

pub fn valid_paths(re_vec: Vec<String>) -> Result<Vec<Vec<String>>, String> {
  // vector containing vectors with all matching paths
  let mut paths: Vec<Vec<String>> = Vec::new();
  
  for re_str in re_vec {
    let mut matches: Vec<String> = Vec::new();
    let mut prefix: &str = "./"; // current directory by default
    match re_str.chars().nth(0) {
      Some('/') => {
        prefix = "";
      }
      _ => {}
    }
  
    let combined_re = String::from(prefix) + &re_str;
    let re: Regex;

    match Regex::new(&combined_re) {
      Err(e) => {
        return Err(e.to_string());
      }
      Ok(regex) => {
        re = regex;
      }
    }

    let max_depth: usize = combined_re.matches('/').count();
  
    for entry in WalkDir::new(prefix).max_depth(max_depth) {
      match entry {
        Ok(entry) => {
          // if the entry's path matches re add it to a temp vec
          let path: String = entry.path().to_path_buf().into_os_string().into_string().unwrap();
          match re.is_match(&path) {
            true => {
              // TODO strip the prefix... for the result
              matches.push(path);
            }
            false => {}
          }
        }
        Err(e) => {
          return Err(e.to_string());
        }
      }
    } 
    // add the temp vec to the upper vec
    paths.push(matches);

  }
  Ok(paths)
}

//fn match_combos(re_str: &str) -> Vec<&str> {
//  let re: Regex = Regex::new(re_str).unwrap();
//
//  // List all files that match regex ? how should this be done?
//  // Should the tokenizer split up path tokens and add them to the path that should be listed and
//  // searched for regex matches?
//
//}

// TODO 
// Filter out commands that contain wildcards in caller
// Construct regex for token vectors
// Provide list of possible arguments and execute each as separate command
