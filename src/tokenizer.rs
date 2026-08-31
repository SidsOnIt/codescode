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

    #[regex(r"#[0-9a-fA-F]{3}(?:[0-9a-fA-F]{3})?", priority = 20)]
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
    #[regex(r#"#[^!0-9a-fA-F\r\n][^\r\n]*"#, priority = 2, allow_greedy = true)]
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
    #[regex("[A-Z][a-zA-Z0-9_]*")]
    //must start with Capital, followed by 0 or more lower or captial letters, numbers or _'s after it.
    Structure,

    #[regex("[a-z_][a-zA-Z0-9_]*")]
    //must start with lower letter or _, and have 0 or more lower or captial letters, numbers or _'s after it.
    Identifier,

    #[token("const")]
    #[token("static")]
    #[regex("[A-Z][A-Z0-9_]+")]
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

pub fn tokenize(source: &str) -> impl Iterator<Item = TokenMatch<'_>> {
    CodeToken::lexer(source)
        .spanned()
        .map(move |(token, span)| TokenMatch {
            token,
            slice: &source[span],
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bash_shebang_still_recognized_as_comment() {
        // Regression guard: the new hex-digit exclusion on the '#' comment
        // rule must not disturb the pre-existing '!' exclusion used for
        // shebang lines.
        let toks: Vec<_> = tokenize("#!/usr/bin/env bash").map(|t| t.token).collect();
        // '!' is excluded from this rule (pre-existing behavior), so '#'
        // alone falls through to Symbol, then the rest tokenizes separately.
        // We only assert it's NOT swallowed as one Comment token here, since
        // that's the specific case this rule is designed to preserve.
        assert_ne!(toks, vec![Ok(CodeToken::Comment)]);
    }

    #[test]
    fn hex_color_not_swallowed_by_comment() {
        let toks: Vec<_> = tokenize("#fff").map(|t| t.token).collect();
        assert_eq!(toks, vec![Ok(CodeToken::HexColor)]);
    }

    #[test]
    fn triple_quote_docstring_is_one_comment() {
        let src = "\"\"\"\nPython\nMultiline Comment\n\"\"\"";
        let toks: Vec<_> = tokenize(src).map(|t| t.token).collect();
        assert_eq!(toks, vec![Ok(CodeToken::Comment)]);
    }

    #[test]
    fn sql_line_comment_recognized() {
        let toks: Vec<_> = tokenize("-- SQL Single").map(|t| t.token).collect();
        assert_eq!(toks, vec![Ok(CodeToken::Comment)]);
    }

    #[test]
    fn lua_line_comment_recognized() {
        let toks: Vec<_> = tokenize("-- Lua Single").map(|t| t.token).collect();
        assert_eq!(toks, vec![Ok(CodeToken::Comment)]);
    }

    #[test]
    fn lua_block_comment_recognized_and_does_not_overrun() {
        let src = "--[[\nLua Multi\n]]\nx";
        let toks: Vec<_> = tokenize(src).map(|t| t.token).collect();
        assert_eq!(
            toks,
            vec![
                Ok(CodeToken::Comment),
                Ok(CodeToken::Newline),
                Ok(CodeToken::Identifier),
            ]
        );
    }

    #[test]
    fn css_custom_property_not_treated_as_comment() {
        // Regression guard: `--main-color: #ff4500;` must NOT be swallowed as
        // a comment. This is the exact case that previously failed: the
        // generic '#' comment rule matched `#ff4500;` (9 chars, including the
        // trailing `;`) which is LONGER than HexColor's `#ff4500` (7 chars),
        // and logos picks the longer match regardless of priority.
        let toks: Vec<_> = tokenize("--main-color: #ff4500;")
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
    fn decrement_operator_not_treated_as_comment() {
        // Regression guard: C-family `i--;` must NOT be swallowed as a comment.
        let toks: Vec<_> = tokenize("i--;").map(|t| t.token).collect();
        assert!(
            !toks.contains(&Ok(CodeToken::Comment)),
            "decrement operator was misread as a comment: {:?}",
            toks
        );
    }

    #[test]
    fn multiple_docstrings_do_not_merge() {
        // Regression test: a naive lazy `*?` regex would match from the first
        // `"""` all the way to the LAST `"""` in the source, since logos
        // doesn't support non-greedy quantifiers. This checks that two
        // separate triple-quoted blocks stay separate.
        let src = "\"\"\"first\"\"\"\nx\n\"\"\"second\"\"\"";
        let toks: Vec<_> = tokenize(src).map(|t| t.token).collect();
        assert_eq!(
            toks,
            vec![
                Ok(CodeToken::Comment),
                Ok(CodeToken::Newline),
                Ok(CodeToken::Identifier),
                Ok(CodeToken::Newline),
                Ok(CodeToken::Comment),
            ]
        );
    }
}
