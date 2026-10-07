; A run button on each test. The task reads the test's line from the saved
; file (ZED_ROW is the row of the @run capture), so the button belongs on the
; line Bats parses the test name from.

; @test "name" { ... }
((test_block
  "@test" @run) @_bats-test
  (#set! tag bats-test))

; A function Bats runs as a test because its opening line ends in `# @test`:
;   name() { # @test
((function_definition
  name: (word) @run
  body: (compound_statement
    .
    (comment) @_marker)) @_bats-test
  (#match? @_marker "^#[ \t]*@test[ \t]*$")
  (#set! tag bats-test))

; The whole file.
((program) @run
  (#set! tag bats-file))
