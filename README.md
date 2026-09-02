# codescode
![Version](https://img.shields.io/badge/version-0.9.2-green) 
`year.month.update`

status of the project:
Basically ready for v1 as of 9-2-2026, I just need to write
out the docs.

# What is codescode?
a featherweight generic code tokenizer that flattens disparate
language terminology into common terminology by it's intent.

Features:
  * Simple API
  * Near Zero Overhead
  * Eager and Lazy Tokenizer Funtions

portable: windows linux macos windows ios android wasm

Use Cases:
  * Cloud or Client Side LLM Code Pre-Tokenizer to reduce latency
  * Rendering directly to styled text from the collection using a match statement
  * As a lexer for an ide ready to become an ast
  
---

How to use:
Simply: a &str goes in and a Vec<CodeToken> comes out for you
to match against.
> I will put an eager and lazy example
demonstrating how to use it for code formatting in dioxus.

---
NOTE: You can group multple tokens into the same render output.
just because an option is there doesnt mean you have to use 100%
of them.

CodeToken enum variants include:
  * Whitespace
  * Newline
  * HexColor (3 and 6)
  * Comment
  * StringLiteral
  * Structure
  * Identifier
  * Constant
  * Boolean
  * Void
  * Verb
  * Action
  * Async
  * Exception
  * Import
  * Function
  * Declaration
  * Number
  * Operator
  * Condition
  * Comparitor
  * Concatenator
  * Loop
  * Colon
  * Assignment
  * Call
  * Map
  * Array
  * Symbol
  * Separator
