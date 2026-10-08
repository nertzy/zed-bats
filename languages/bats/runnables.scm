; A run button on each test. Zed exports each capture other than @run to the
; task as ZED_CUSTOM_<capture>, so @BATS_TEST_NAME gives the bats-test task
; the name to filter on: the text Bats reads it from.

; @test "name" { ... }
((test_block
  "@test" @run
  name: (test_name) @BATS_TEST_NAME) @_bats-test
  (#set! tag bats-test))

; A function Bats runs as a test, under its own name, because its opening
; line ends in `# @test`:
;   name() { # @test
((function_definition
  name: (word) @run @BATS_TEST_NAME
  body: (compound_statement
    .
    (test_marker_comment))) @_bats-test
  (#set! tag bats-test))

; The whole file, from a button on its first line. Zed drops any runnable
; whose @run reaches the end of the buffer, so capture the first child rather
; than the program. Capturing the program too would export the whole file to
; every task as a variable.
((program
  .
  (_) @run)
  (#set! tag bats-file))
