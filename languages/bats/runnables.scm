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

; The whole file, from a button on its first line. Zed drops any runnable
; whose @run reaches the end of the buffer, so capture the first child rather
; than the program.
((program
  .
  (_) @run) @_bats-file
  (#set! tag bats-file))
