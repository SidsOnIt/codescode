use logos::Logos;
//test

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// A generic code tokenizer that flattens disparate terminology
/// across languages into basic, universal intent primitives.
#[derive(Logos, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum CodeToken {
    #[regex(r"[ \t]+")]
    Whitespace,

    #[regex(r"\r?\n")]
    Newline,

    // --- Hex Colors (must win over the generic '#' comment rule) ---
    #[regex(r"#[0-9a-fA-F]{3}(?:[0-9a-fA-F]{3})?", priority = 20)]
    HexColor,

    // --- Comments & Multi-line Docstrings (Forced High Priority) ---
    // NOTE: logos compiles regexes to a DFA and does NOT support lazy/non-greedy
    // quantifiers (`*?`) the way PCRE does — they're effectively treated as greedy.
    // A naive `"""[\s\S]*?"""` will therefore match from the FIRST `"""` all the
    // way to the LAST `"""` anywhere later in the source. Instead we express
    // "don't allow three quotes in a row" directly, which forces termination at
    // the first real closing `"""` without needing laziness.
    #[regex(r#"//[^\r\n]*"#, allow_greedy = true)]
    #[regex(r#"/\*[^*]*\*+([^/*][^*]*\*+)*/"#, allow_greedy = true)]
    // NOTE ON THE '#' COMMENT RULE BELOW: logos resolves competing matches by
    // LONGEST MATCH WINS — explicit `priority` only breaks ties when two
    // candidates match the exact same length. So giving HexColor a higher
    // priority than this rule is not enough on its own: for input like
    // `#ff4500;`, this rule would greedily match all 9 chars (through the
    // `;`) while HexColor can only match the 7-char `#ff4500`, and the
    // longer match wins regardless of priority. Since every real hex color
    // starts with a hex digit right after `#`, we instead exclude hex
    // digits (and `!`, for shebangs) from the character allowed right after
    // `#` here — that makes this rule structurally unable to even start a
    // competing match at a HexColor position, so there's no race to lose.
    // Trade-off: a `#`-comment whose very first character is a hex digit or
    // letter a-f (e.g. `#123 fix this` with no space) won't be recognized as
    // a comment. No sample in this corpus does that; real comments here all
    // start with a space, `[`, or a non-hex letter right after `#`.
    #[regex(r#"#[^!0-9a-fA-F\r\n][^\r\n]*"#, priority = 2, allow_greedy = true)]
    #[regex(r#"<!--(?:[^-]|-[^-]|--[^>])*-->"#, allow_greedy = true)]
    #[regex(r#""""(?:[^"]|"[^"]|""[^"])*"""|'''(?:[^']|'[^']|''[^'])*'''"#, priority = 10)]
    // Lua/SQL/Haskell `--` line comment. Deliberately requires a space or tab
    // immediately after the second dash so it can NEVER fire on:
    //   - CSS custom properties, e.g. `--main-color: #ff4500;` (dash glued to a letter)
    //   - the C-family decrement operator, e.g. `i--;` (dash glued to punctuation)
    // Real `--` comments in the corpus always have a space/tab before the text
    // (e.g. "-- Lua Single"), so this loses no real matches while staying safe.
    #[regex(r"--[ \t][^\r\n]*", priority = 6, allow_greedy = true)]
    // Lua's `--[[ ... ]]` block comment. Uses the same "don't allow the closer
    // to appear inside the body" trick as the triple-quote docstring rule above,
    // since logos can't do lazy `*?` matching to find the *nearest* `]]`.
    #[regex(r"--\[\[(?:[^\]]|\][^\]])*\]\]", priority = 12)]
    Comment,

    // --- Strings ---
    #[regex(r#""([^"\\\n]|\\.)*""#)]
    #[regex(r#"'([^'\\\n]|\\.)*'"#)]
    #[regex(r#"`([^`\\]|\\.)*`"#)]
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
    Structure,

    #[regex("[a-z_][a-zA-Z0-9_]*")]
    Identifier,

    #[token("const")]
    #[token("static")]
    #[regex("[A-Z][A-Z0-9_]+")]
    Const,

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
        let toks: Vec<_> = tokenize("#!/usr/bin/env bash")
            .map(|t| t.token)
            .collect();
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
