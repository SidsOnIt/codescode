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

    #[token("select")] // Matches database SELECT verbs
    #[token("insert")] // Matches database INSERT verbs
    #[token("update")] // Matches database UPDATE verbs
    #[token("delete")] // Matches database DELETE verbs
    #[token("create")] // Matches database CREATE verbs
    #[token("alter")] // Matches database ALTER verbs
    #[token("drop")] // Matches database DROP verbs
    #[token("trunicate")] // Matches truncation command verbs
    #[token("concatenate")] // Matches string or stream concatenation verbs
    #[token("commit")] // Matches transaction commit verbs
    #[token("rollback")] // Matches transaction rollback verbs
    #[token("get")] // Matches HTTP GET verbs
    #[token("post")] // Matches HTTP POST verbs
    #[token("put")] // Matches HTTP PUT verbs
    #[token("patch")] // Matches HTTP PATCH verbs
    #[token("head")] // Matches HTTP HEAD verbs
    Verb,

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

    #[token("let")] // Matches immutable variable bindings
    #[token("mut")] // Matches mutable bindings
    #[token("pub")] // Matches public visibility modifiers
    #[token("var")] // Matches variable declarations
    #[token("val")] // Matches immutable value declarations
    #[token("local")] // Matches local variable scopes
    #[token("private")] // Matches private access modifiers
    #[token("protected")] // Matches protected access modifiers
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

    #[token("where")] // Matches query condition filters
    #[token("join")] // Matches table join clauses
    #[token("on")] // Matches relation conditions for joins
    #[token("group")] // Matches grouping clauses
    #[token("order")] // Matches sorting order clauses
    #[token("having")] // Matches filtered grouping clauses
    #[token("limit")] // Matches pagination limit clauses
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

    #[token("while")] // Matches while loop controls
    #[token("for")] // Matches for loop iterations
    #[token("loop")] // Matches infinite loop constructs
    #[token("repeat")] // Matches repeat-until loop blocks
    #[token("do")] // Matches do-while loop constructs
    #[token("break")] // Matches loop break statements
    #[token("continue")] // Matches loop continue statements
    #[token("yield")] // Matches generator yield controls
    Loop,

    #[token(":")] // Matches type annotation colons
    #[token("=>")] // Matches mapping arrow tokens
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

    #[token(",")] // Matches structural item separator commas
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
