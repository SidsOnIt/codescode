use logos::Logos;

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

    // --- Comments (Using explicit allow_greedy for Logos compliance) ---
    #[regex(r#"//[^\r\n]*"#, allow_greedy = true)]
    #[regex(r#"/\*[^*]*\*+([^/*][^*]*\*+)*/"#, allow_greedy = true)]
    #[regex(r#"#[^!\r\n][^\r\n]*"#, allow_greedy = true)]
    Comment,

    // --- Strings & Interpolation ---
    #[regex(r#""([^"\\]|\\.)*""#)]
    #[regex(r#"'([^'\\]|\\.)*'"#)]
    #[regex(r#"`([^`\\]|\\.)*`"#)]
    StringLiteral,

    #[regex(r#"\$[a-zA-Z_][a-zA-Z0-9_]*"#)]
    #[regex(r#"#\{[^}]+\}"#)]
    Interpolation,

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
    #[token("trunicate")]
    #[token("concatenate")]
    #[token("commit")]
    #[token("rollback")]
    #[token("get")]
    #[token("post")]
    #[token("put")]
    #[token("patch")]
    #[token("head")]
    Verb,

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
    #[token("var")]
    #[token("val")]
    #[token("local")]
    #[token("private")]
    #[token("protected")]
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
    #[token("loop")]
    #[token("repeat")]
    #[token("do")]
    #[token("break")]
    #[token("continue")]
    #[token("yield")]
    Loop,

    #[token(":")]
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
