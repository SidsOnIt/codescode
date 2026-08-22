use logos::Logos;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(Logos, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum CodeToken {
    #[regex(r"[ \t]+")]
    Whitespace,

    #[regex(r"\r?\n")]
    Newline,

    // --- Comments & Multi-line Docstrings (Prioritized) ---
    #[regex(r#"//[^\r\n]*"#, allow_greedy = true)]
    #[regex(r#"/\*[^*]*\*+([^/*][^*]*\*+)*/"#, allow_greedy = true)]
    #[regex(r#"#[^!\r\n][^\r\n]*"#, allow_greedy = true)]
    #[regex(r#"<!--(?:[^-]|-[^-]|--[^>])*-->"#, allow_greedy = true)]
    #[regex(r#"""""[^*]*\*+(?:[^"*][^*]*\*+)*"""|'''[^*]*\*+(?:[^'*][^*]*\*+)*'''|""""""|''''''"#, allow_greedy = true)]
    Comment,

    // --- Hex Colors ---
    #[regex(r"#[0-9a-fA-F]{3}(?:[0-9a-fA-F]{3})?")]
    HexColor,

    // --- Strings & Interpolation ---
    #[regex(r#""([^"\\\n]|\\.)*""#)]
    #[regex(r#"'([^'\\\n]|\\.)*'"#)]
    #[regex(r#"`([^`\\]|\\.)*`"#)]
    StringLiteral,

    #[regex(r#"\$[a-zA-Z_][a-zA-Z0-9_]*"#)]
    #[regex(r#"#\{[^}]+\}"#)]
    Interpolation,

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
