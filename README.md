# CatScript Compiler

This is a unfished small toy compiler written in rust for unix-like systems with a python like syntax, and simple staticaly typed stack oriented semantics! :D

It is broken into several parts:
1. Lexical Analysis, Transforms the raw-text(code) into a series of tokens(`Vec<Token>`)
2. Syntax Analysis, Transforms tokens into a Abstract-Syntax-Tree(`AST`)
