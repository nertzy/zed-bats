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

; setup() { ... }, function helper { ... }
(function_definition
  "function"? @context
  name: (word) @name) @item
