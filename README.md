# codescode
![Version](https://img.shields.io/badge/version-0.08.3-green) 
`year.month.update`

'version 1.0' = when the year slot of the version number contains a year such as 2026
This will mean:
  > The api is stable and every single character of the source code
  is written by or reviewed by a human deeply prior to acceptance

status of the project:

Core Enum: Alpha, usable for testing.
Tokenization Functions: Alpha, usable for testing.
Tests: Ephemeral Slop, these will be totally redone scorch earth prior to 1.0
Documentation: last task pre 1.0

# What is codescode?

a featherweight generic code tokenizer => Vec<CodeToken>

portable: windows linux macos windows ios android wasm

Use Cases:
  * Cloud or Client Side LLM Code Pre-Tokenizer to reduce latency
  * Rendering directly to styled_text from the collection using a match statement
  * As a lexer for an ide ready to become an ast  
