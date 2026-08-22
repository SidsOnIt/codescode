use logos::Logos;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// A generic code tokenizer that flattens disparate terminology
/// across languages into basic, universal intent primitives.
#[derive(Logos, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum CodeToken {
    #[regex(r"[ \t]+")] // Matches whitespace and horizontal tabs
    Whitespace,

    #[regex(r"\r?\n")] // Matches carriage returns and newlines
    Newline,

    // --- Comments (Including Python Triple-Quotes & HTML) ---
    #[regex(r#"//[^\r\n]*"#, allow_greedy = true)]
    #[regex(r#"/\*[^*]*\*+([^/*][^*]*\*+)*/"#, allow_greedy = true)]
    #[regex(r#"#[^!\r\n][^\r\n]*"#, allow_greedy = true)]
    #[regex(r#"<!--[\s\S]*?-->"#, allow_greedy = true)]
    #[regex(r#"""""[\s\S]*?"""|'''[\s\S]*?'''"#, allow_greedy = true)]
    Comment,

    // --- Strings & Interpolation ---
    // The \n is excluded here so unclosed quotes don't eat the whole file
    #[regex(r#""([^"\\\n]|\\.)*""#)]
    #[regex(r#"'([^'\\\n]|\\.)*'"#)]
    #[regex(r#"`([^`\\]|\\.)*`"#)] // Backticks remain greedy for multi-line strings
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
    #[regex("[A-Z][a-zA-Z0-9_]*")] // Matches capitalized PascalCase names, structs, and types
    Structure,

    #[regex("[a-z_][a-zA-Z0-9_]*")] // Matches lowercase identifiers and variable names
    Identifier,

    #[token("const")] // Matches constant declarations
    #[token("static")] // Matches static variable declarations
    #[regex("[A-Z][A-Z0-9_]+")] // Matches SCREAMING_SNAKE_CASE constant patterns
    Const,

    #[token("true")] // Matches boolean true literal
    #[token("false")] // Matches boolean false literal
    Boolean,

    #[token("null")] // Matches null values
    #[token("nil")] // Matches nil pointer/value indicators
    #[token("none")] // Matches none values
    #[token("void")] // Matches void return types
    #[token("undefined")] // Matches undefined type values
    Void,

    // --- Database & Protocol Verbs ---
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

    // --- General Actions & I/O ---
    #[token("print")]
    #[token("println")]
    #[token("echo")]
    #[token("log")]
    Action,

    // --- Concurrency & Asynchrony ---
    #[token("async")]
    #[token("await")]
    #[token("suspend")]
    #[token("go")]
    #[token("defer")]
    #[token("spawn")]
    Async,

    // --- Error Handling & Exceptions ---
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

    #[token("import")] // Matches import statements
    #[token("include")] // Matches file or header inclusion
    #[token("require")] // Matches module requirement loading
    #[token("use")] // Matches namespace usage declarations
    #[token("mod")] // Matches inline or external module declarations
    #[token("crate")] // Matches crate root references
    #[token("extern")] // Matches external linkage declarations
    #[token("package")] // Matches package declarations
    #[token("namespace")] // Matches namespace scoping blocks
    #[token("using")] // Matches namespace alias provisions
    #[token("from")] // Matches source routing for imports
    #[token("export")] // Matches module exports
    Import,

    #[token("fn")] // Matches Rust-style function declarations
    #[token("func")] // Matches general function declarations
    #[token("function")] // Matches full function keyword declarations
    #[token("def")] // Matches Python-style function definitions
    #[token("return")] // Matches return execution control
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

    #[regex("[0-9]+")] // Matches numeric digit literals
    Number,

    #[token("=")] // Matches standard assignment operators
    #[token("+")] // Matches addition arithmetic operators
    #[token("-")] // Matches subtraction arithmetic operators
    #[token("*")] // Matches multiplication arithmetic operators
    #[token("/")] // Matches division arithmetic operators
    #[token("%")] // Matches modulo remainder operators
    #[token("^")] // Matches bitwise XOR or power operators
    #[token("**")] // Matches exponentiation operators
    Operator,

    // --- Conditions & Control Flow ---
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

    #[token("==")] // Matches equality comparison operators
    #[token("!=")] // Matches inequality comparison operators
    #[token(">")] // Matches greater-than operators
    #[token("<")] // Matches less-than operators
    #[token(">=")] // Matches greater-than-or-equal operators
    #[token("<=")] // Matches less-than-or-equal operators
    #[token("not")] // Matches logical negation operators
    #[token("in")] // Matches collection membership operators
    #[token("like")] // Matches pattern matching operators
    #[token("is null")] // Matches null evaluation checks
    Comparitor,

    #[token("|")] // Matches bitwise OR operators
    #[token("&")] // Matches bitwise AND operators
    #[token("~")] // Matches bitwise NOT operators
    #[token("||")] // Matches logical OR boolean operators
    #[token("&&")] // Matches logical AND boolean operators
    #[token("??")] // Matches nullish coalescing operators
    #[token("?.")] // Matches optional chaining operators
    #[token("or")] // Matches word-form logical OR operators
    #[token("and")] // Matches word-form logical AND operators
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

    #[token(":")] // Matches type annotation colons
    #[token("=>")] // Matches mapping arrow tokens
    #[token("->")] // Matches return type arrow tokens
    #[token("as")] // Matches type casting clauses
    #[token("alias")] // Matches custom namespace or type aliases
    Assignment,

    #[token(".")] // Matches member access dots
    #[token("..")] // Matches Dart cascade operators
    #[token("?..")] // Matches Dart null-aware cascade operators
    #[token("::")] // Matches namespacing scope resolution operators
    #[token("<<")] // Matches bitwise shift left or stream push operators
    #[token(">>")] // Matches bitwise shift right or stream pull operators
    #[token("(")] // Matches opening call parentheses
    #[token(")")] // Matches closing call parentheses
    Call,

    #[token("}")] // Matches closing map block delimiters
    #[token("{")] // Matches opening map block delimiters
    Map,

    #[token("[")] // Matches opening array collection brackets
    #[token("]")] // Matches closing array collection brackets
    Array,

    // --- General Symbols ---
    #[token("@")]
    #[token("!")]
    #[token("?")]
    #[token("$")]
    #[token("#")]
    #[token("\\")] // Matches backslash
    Symbol,

    #[token(",")] // Matches structural item separator commas
    #[token(";")] // Matches statement terminator semicolons
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
