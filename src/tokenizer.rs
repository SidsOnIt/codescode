use logos::Logos;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Logos, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum CodeToken {
    #[regex(r"[ \t]+")]
    // (1) or more (spaces or tabs)
    Whitespace,

    #[regex(r"\r?\n")]
    // \n for posix & \r\n for windows
    Newline,

    #[regex(r"#[0-9a-fA-F]{3}(?:[0-9a-fA-F]{3})?", priority = 10)]
    // match if starts with a #, followed by (3) or (6) hex chars,
    // if (3) => #RGB; if (6) => #RRGGBB
    HexColor,

    #[regex(r#"//[^\r\n]*"#, allow_greedy = true)]
    // starts with // followed by (0) or more of anything until a newline
    #[regex(r#"/\*[^*]*\*+([^/*][^*]*\*+)*/"#, allow_greedy = true)]
    // delimitation: /* content */
    // [^*]* = match zero or more non-asterisk characters in a row
    // \*+ = consume 1 or more consecutive * chars
    // ( content )* run the following (0) or more times
    // ---> [^/*] = match (1) non-asterisk non-/ character
    // ---> [^*]* = match zero or more non-asterisk characters in a row
    // ---> \*+ = consume 1 or more consecutive * chars
    #[regex(r#"#[^!0-9a-fA-F\r\n][^\r\n]*"#, priority = 5, allow_greedy = true)]
    // delimitation: # content \n
    // #[^!0-9a-fA-F\r\n] = match # followed by any single non ! or hex char
    // [^\r\n]* = match on (0) or more chars until a newline,
    #[regex(r#"<!--(?:[^-]|-[^-]|--[^>])*-->"#, allow_greedy = true)]
    // delimitation: <!-- content -->
    // any number of chars where:
    // ---> not a -
    // ---> OR
    // ---> a - followed by a non - char
    // ---> OR
    // ---> a -- not followed by >
    #[regex(
        r#""""(?:[^"]|"[^"]|""[^"])*"""|'''(?:[^']|'[^']|''[^'])*'''"#,
        priority = 10
    )]
    // delimitation: """content""" or '''content'''
    // each main branch filters for the following:
    // not a "
    // OR
    // a " not followed by another "
    // OR
    // a "" not followed by another "
    #[regex(r"--[ \t][^\r\n]*", priority = 6, allow_greedy = true)]
    // delimitation: -- content \n
    // -- followed by a space or tab
    // then followed by anything until a newline
    #[regex(r"--\[\[(?:[^\]]|\][^\]])*\]\]", priority = 12)]
    // delimited as --[[ content ]],
    // any number of chars where:
    // ---> not a ]
    // ---> OR
    // ---> a ] not followed by a ]
    Comment,

    // --- Strings ---
    #[regex(r#""([^"\\\n]|\\.)*""#)]
    //delimited with ", ([does not contain ", \ or \n] or does contain \withanythingrightafter) 0 or more times
    #[regex(r#"'([^'\\\n]|\\.)*'"#)]
    //delimited with ', ([does not contain ', \ or \n] or does contain \withanythingrightafter) 0 or more times
    #[regex(r#"`([^`\\]|\\.)*`"#)]
    //delimited with `, ([does not contain ` or \ ] or does contain \withanythingrightafter) 0 or more times
    StringLiteral,

    // --- Structural Declarations & Names ---
    #[token("class")]
    #[token("struct")]
    #[token("interface")]
    #[token("enum")]
    #[token("trait")]
    #[token("type")]
    #[token("record")]
    #[token("protocol")]
    #[token("impl")]
    #[token("defprotocol")]
    #[token("defmodule")]
    #[token("defstruct")]
    #[token("dataclass")]
    #[regex("[A-Z][a-zA-Z0-9_]*", priority = 3)]
    //must start with Capital, followed by 0 or more lower or captial letters, numbers or _'s after it.
    Structure,

    #[regex("[a-z_][a-zA-Z0-9_]*" priority = 1)]
    //must start with lower letter or _, and have 0 or more lower or captial letters, numbers or _'s after it.
    Identifier,

    #[token("const")]
    #[token("static")]
    #[regex("[A-Z][A-Z0-9_]+", priority = 2)]
    //must start with capital letter, and have 1 or more captial letters, numbers or _'s after it.
    Constant,

    #[token("true")]
    #[token("false")]
    Boolean,

    #[token("null")]
    #[token("nil")]
    #[token("none")]
    #[token("void")]
    #[token("undefined")]
    Void,

    #[token("select")]
    #[token("insert")]
    #[token("update")]
    #[token("delete")]
    #[token("create")]
    #[token("alter")]
    #[token("drop")]
    #[token("truncate")]
    #[token("concatenate")]
    #[token("commit")]
    #[token("rollback")]
    #[token("get")]
    #[token("post")]
    #[token("put")]
    #[token("patch")]
    #[token("head")]
    Verb,

    #[token("print")]
    #[token("println")]
    #[token("echo")]
    #[token("log")]
    Action,

    #[token("async")]
    #[token("await")]
    #[token("suspend")]
    #[token("go")]
    #[token("defer")]
    #[token("spawn")]
    Async,

    #[token("try")]
    #[token("catch")]
    #[token("except")]
    #[token("rescue")]
    #[token("finally")]
    #[token("ensure")]
    #[token("throw")]
    #[token("throws")]
    #[token("raise")]
    #[token("rethrow")]
    #[token("reraise")]
    Exception,

    #[token("import")]
    #[token("include")]
    #[token("require")]
    #[token("use")]
    #[token("mod")]
    #[token("crate")]
    #[token("extern")]
    #[token("package")]
    #[token("namespace")]
    #[token("using")]
    #[token("from")]
    #[token("export")]
    Import,

    #[token("fn")]
    #[token("func")]
    #[token("function")]
    #[token("def")]
    #[token("return")]
    Function,

    #[token("let")]
    #[token("mut")]
    #[token("pub")]
    #[token("public")]
    #[token("var")]
    #[token("val")]
    #[token("local")]
    #[token("private")]
    #[token("protected")]
    #[token("final")]
    #[token("abstract")]
    #[token("virtual")]
    #[token("override")]
    #[token("comptime")]
    #[token("factory")]
    #[token("data")]
    Declaration,

    #[regex("[0-9]+")]
    // (1) or more number chars
    Number,

    #[token("=")]
    #[token("+")]
    #[token("-")]
    #[token("*")]
    #[token("/")]
    #[token("%")]
    #[token("^")]
    #[token("**")]
    Operator,

    #[token("where")]
    #[token("join")]
    #[token("on")]
    #[token("group")]
    #[token("order")]
    #[token("having")]
    #[token("limit")]
    #[token("if")]
    #[token("else")]
    #[token("elif")]
    #[token("elsif")]
    #[token("switch")]
    #[token("case")]
    #[token("match")]
    #[token("default")]
    #[token("then")]
    #[token("guard")]
    #[token("with")]
    #[token("begin")]
    #[token("end")]
    #[token("by")]
    #[token("desc")]
    #[token("asc")]
    #[token("over")]
    #[token("partition")]
    Condition,

    #[token("==")]
    #[token("!=")]
    #[token(">")]
    #[token("<")]
    #[token(">=")]
    #[token("<=")]
    #[token("not")]
    #[token("in")]
    #[token("like")]
    #[token("is null")]
    Comparitor,

    #[token("|")]
    #[token("&")]
    #[token("~")]
    #[token("||")]
    #[token("&&")]
    #[token("??")]
    #[token("?.")]
    #[token("or")]
    #[token("and")]
    Concatenator,

    #[token("while")]
    #[token("for")]
    #[token("foreach")]
    #[token("loop")]
    #[token("repeat")]
    #[token("until")]
    #[token("do")]
    #[token("break")]
    #[token("continue")]
    #[token("next")]
    #[token("yield")]
    #[token("pass")]
    Loop,

    #[token(":")]
    Colon,

    #[token("=>")]
    #[token("->")]
    #[token("as")]
    #[token("alias")]
    Assignment,

    #[token(".")]
    #[token("..")]
    #[token("?..")]
    #[token("::")]
    #[token("<<")]
    #[token(">>")]
    #[token("(")]
    #[token(")")]
    Call,

    #[token("}")]
    #[token("{")]
    Map,

    #[token("[")]
    #[token("]")]
    Array,

    #[token("@")]
    #[token("!")]
    #[token("?")]
    #[token("$")]
    #[token("#")]
    #[token("\\")]
    Symbol,

    #[token(",")]
    #[token(";")]
    Separator,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TokenMatch<'a> {
    pub token: Result<CodeToken, ()>,
    pub slice: &'a str,
}

// Concrete list/slice representation of tokenized source
pub type TokenVec<'a> = Vec<TokenMatch<'a>>;

// Lazy iterator for tokenizing code on demand
pub fn tokenize_lazy(source: &str) -> impl Iterator<Item = TokenMatch<'_>> {
    CodeToken::lexer(source)
        .spanned()
        .map(move |(token, span)| TokenMatch {
            token,
            slice: &source[span],
        })
}

/// Eagerly tokenizes source text into a full contiguous vector.
pub fn tokenize(source: &str) -> TokenVec<'_> {
    tokenize_lazy(source).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bash_shebang_not_comment() {
        let toks: Vec<_> = tokenize_lazy("#!/usr/bin/env bash")
            .map(|t| t.token)
            .collect();
        assert_ne!(toks, vec![Ok(CodeToken::Comment)]);
    }

    #[test]
    fn hex_color3_not_comment() {
        let toks = tokenize("#fff");
        assert_eq!(toks.len(), 1);
        assert_eq!(toks[0].token, Ok(CodeToken::HexColor));
    }

    #[test]
    fn hex_color6_not_comment() {
        let toks = tokenize("#ffffff");
        assert_eq!(toks.len(), 1);
        assert_eq!(toks[0].token, Ok(CodeToken::HexColor));
    }

    #[test]
    fn css_property_not_comment() {
        let toks: Vec<_> = tokenize_lazy("--main-color: #ff4500;")
            .map(|t| t.token)
            .filter(|t| *t != Ok(CodeToken::Whitespace))
            .collect();
        assert_eq!(
            toks,
            vec![
                Ok(CodeToken::Operator),   // -
                Ok(CodeToken::Operator),   // -
                Ok(CodeToken::Identifier), // main
                Ok(CodeToken::Operator),   // -
                Ok(CodeToken::Identifier), // color
                Ok(CodeToken::Colon),      // :
                Ok(CodeToken::HexColor),   // #ff4500
                Ok(CodeToken::Separator),  // ;
            ]
        );
    }

    #[test]
    fn decrement_operator_not_comment() {
        let toks = tokenize("i--;");
        let has_comment = toks.iter().any(|t| t.token == Ok(CodeToken::Comment));
        assert!(
            !has_comment,
            "decrement operator was misread as a comment: {:?}",
            toks
        );
    }

    #[test]
    fn all_comment_types_recognized() {
        let comment_snippets = vec![
            "// C-style line comment",
            "/* C-style block comment */",
            "# Hash style comment",
            "<!-- HTML comment -->",
            "\"\"\" Triple double-quote docstring \"\"\"",
            "''' Triple single-quote docstring '''",
            "-- SQL or Lua line comment",
            "--[[ Lua multi-line block comment ]]",
        ];

        for snippet in comment_snippets {
            let toks = tokenize(snippet);
            let token_kinds: Vec<_> = toks.into_iter().map(|t| t.token).collect();
            assert_eq!(
                token_kinds,
                vec![Ok(CodeToken::Comment)],
                "Failed to recognize snippet as a single Comment token: {:?}",
                snippet
            );
        }
    }
}
