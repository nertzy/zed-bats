; @test "name" { ... }. An unquoted name is several words, each its own node;
; Zed joins the @name captures.
(test_block
  "@test" @context
  .
  [
    (string)
    (raw_string)
    (word)
  ]+ @name) @item

; Functions by bare name, as bash-language-server lists them, whether written
; `name()` or `function name`. A function Bats runs as a test because its
; opening line ends in `# @test` shows the marker as context: `name # @test`.
(function_definition
  name: (word) @name
  body: (_
    .
    (test_marker_comment)? @context)) @item
