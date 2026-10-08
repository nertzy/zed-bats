; Highlights for Bats test files.
;
; Covers the Bash constructs from tree-sitter-bash plus Bats test blocks,
; hooks, helpers, and variables. Parts follow the MIT-licensed
; queries/highlights.scm of tree-sitter-bash (Copyright (c) 2017 Max Brunsfeld).
;
; No two patterns capture the same node, so the result does not depend on
; whether a highlighter prefers the first or the last matching pattern.
; Patterns that share a node type split it with complementary predicates;
; keep each pair's regex identical.

; Comments

; An interpreter line is a directive wherever it appears, so the two
; patterns split every comment on one regex; `#! note` stays a comment.
((comment) @keyword.directive
  (#match? @keyword.directive "^#![ \t]*/"))

((comment) @comment
  (#not-match? @comment "^#![ \t]*/"))

; Keywords

[
  "@test"
  "case"
  "declare"
  "do"
  "done"
  "elif"
  "else"
  "esac"
  "export"
  "fi"
  "for"
  "function"
  "if"
  "in"
  "local"
  "readonly"
  "select"
  "then"
  "typeset"
  "unset"
  "unsetenv"
  "until"
  "while"
] @keyword

; Commands

((command_name) @function.builtin
  (#match? @function.builtin
    "^(\\.|:|alias|bg|bind|break|builtin|caller|cd|command|compgen|complete|compopt|continue|dirs|disown|echo|enable|eval|exec|exit|false|fc|fg|getopts|hash|help|history|jobs|kill|let|logout|mapfile|popd|printf|pushd|pwd|read|readarray|return|set|shift|shopt|source|suspend|test|times|trap|true|type|ulimit|umask|unalias|wait|run|load|skip|bats_load_library|bats_require_minimum_version|bats_pipe)$"))

((command_name) @function
  (#not-match? @function
    "^(\\.|:|alias|bg|bind|break|builtin|caller|cd|command|compgen|complete|compopt|continue|dirs|disown|echo|enable|eval|exec|exit|false|fc|fg|getopts|hash|help|history|jobs|kill|let|logout|mapfile|popd|printf|pushd|pwd|read|readarray|return|set|shift|shopt|source|suspend|test|times|trap|true|type|ulimit|umask|unalias|wait|run|load|skip|bats_load_library|bats_require_minimum_version|bats_pipe)$"))

((command
  argument: (word) @constant)
  (#match? @constant "^-"))

; Functions and Bats hooks

(function_definition
  name: (word) @function.builtin
  (#match? @function.builtin
    "^(setup|teardown|setup_file|teardown_file|setup_suite|teardown_suite)$"))

(function_definition
  name: (word) @function
  (#not-match? @function
    "^(setup|teardown|setup_file|teardown_file|setup_suite|teardown_suite)$"))

; Variables: positional parameters and the variables Bats sets are special

((variable_name) @variable.special
  (#match? @variable.special
    "^([0-9]+|status|output|lines|stderr|stderr_lines|BATS_[A-Za-z0-9_]*)$"))

((variable_name) @variable
  (#not-match? @variable
    "^([0-9]+|status|output|lines|stderr|stderr_lines|BATS_[A-Za-z0-9_]*)$"))

(special_variable_name) @variable.special

; Expansions and substitutions

[
  (command_substitution)
  (process_substitution)
  (expansion)
] @embedded

; Strings

[
  (string)
  (raw_string)
  (ansi_c_string)
  (translated_string)
  (heredoc_body)
] @string

; An unquoted test name is several words; a `name:` field pattern matches
; only the first, and test_block has no other word children.
(test_block
  (word) @string)

[
  (heredoc_start)
  (heredoc_end)
] @string.special

[
  (regex)
  (extglob_pattern)
] @string.regex

; Case patterns. Strings, numbers, and expansions in a pattern keep their own
; captures above, so only the otherwise uncaptured value nodes are patterns.
(case_item
  value: [
    (word)
    (concatenation)
  ] @string.regex)

; Numbers

[
  (number)
  (file_descriptor)
] @number

; Operators

(test_operator) @operator

[
  "!"
  "!="
  "#"
  "##"
  "%"
  "%%"
  "%="
  "&&"
  "&="
  "&>"
  "&>>"
  "*"
  "**"
  "**="
  "*="
  "+"
  "++"
  "+="
  ","
  ",,"
  "-"
  "--"
  "-="
  "-a"
  "-o"
  "/"
  "/#"
  "/%"
  "//"
  "/="
  ":"
  ":+"
  ":-"
  ":="
  ":?"
  "<"
  "<&"
  "<&-"
  "<<"
  "<<-"
  "<<<"
  "<<="
  "<="
  "="
  "=="
  "=~"
  ">"
  ">&"
  ">&-"
  ">="
  ">>"
  ">>="
  ">|"
  "?"
  "@"
  "^"
  "^="
  "^^"
  "|"
  "|&"
  "|="
  "||"
  "~"
] @operator

; Punctuation

[
  "$"
  "${"
  "$("
  "$(("
  "$["
  "$`"
  "`"
  "``"
  "<("
  ">("
] @punctuation.special

[
  "("
  ")"
  "(("
  "))"
  "["
  "]"
  "[["
  "]]"
  "{"
  "}"
] @punctuation.bracket

[
  ";"
  ";;"
  ";&"
  ";;&"
  "&"
  ".."
] @punctuation.delimiter
