#!/usr/bin/env bats
# Runs the commands in languages/bats/tasks.json with the variables Zed
# exports to a task. The Rust task tests cover how Zed fills them in.

setup() {
  root=$(cd "$BATS_TEST_DIRNAME/.." && pwd)
  export ZED_FILE=$root/test/fixtures/example.bats
}

run_task() {
  local command
  command=$(jq -r --arg tag "$1" '.[] | select(.tags | index($tag)) | .command' "$root/languages/bats/tasks.json")
  run bash --norc --noprofile -c "$command"
}

# Runs the bats-test task with the name a run button captures.
run_test_named() {
  export ZED_CUSTOM_BATS_TEST_NAME=$1
  run_task bats-test
}

@test "runs a double-quoted test" {
  run_test_named '"greets by name"'
  [ "$status" -eq 0 ]
  [ "$output" = $'1..1\nok 1 greets by name' ]
}

@test "runs a test whose name is a prefix of another's alone" {
  run_test_named '"greets"'
  [ "$status" -eq 0 ]
  [ "$output" = $'1..1\nok 1 greets' ]
}

@test "matches regex metacharacters in a single-quoted name literally" {
  run_test_named "'matches a.b* (literally) [x] {y} ^\$ | +? \\ too'"
  [ "$status" -eq 0 ]
  [ "$output" = $'1..1\nok 1 matches a.b* (literally) [x] {y} ^$ | +? \\ too' ]
}

@test "runs an unquoted multi-word test" {
  run_test_named 'unquoted name with words'
  [ "$status" -eq 0 ]
  [ "$output" = $'1..1\nok 1 unquoted name with words' ]
}

@test "filters on a name's expansions as written" {
  run_test_named '"matches $HOME literally"'
  [ "$status" -eq 0 ]
  [ "$output" = $'1..1\nok 1 matches '"$HOME"' literally' ]
}

@test "runs a function marked as a test" {
  run_test_named marked_by_comment
  [ "$status" -eq 0 ]
  [ "$output" = $'1..1\nok 1 marked_by_comment' ]
}

@test "runs every test in the file" {
  run_task bats-file
  [ "$status" -eq 0 ]
  [ "${lines[0]}" = 1..8 ]
}
