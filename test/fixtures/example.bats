#!/usr/bin/env bats

# bats file_tags=fixture
bats_require_minimum_version 1.5.0

setup_file() {
  export GREETING="hello"
}

setup() {
  SCRATCH="$BATS_TEST_TMPDIR/scratch"
}

helper() {
  echo "$GREETING, $1"
}

@test "greets by name" {
  run helper world
  [ "$status" -eq 0 ]
  [ "$output" = "hello, world" ]
}

# bats test_tags=regex
@test 'matches a.b* (literally) [x] {y} ^$ | +? \ too' {
  [[ "abc" =~ ^a ]]
}

@test unquoted name with words {
  case start in
    start|stop) true ;;
    *) false ;;
  esac
}

@test "greets" {
  [ "$(helper you)" = "hello, you" ]
}

@test "matches $HOME literally" {
  true
}

@test "one line" { true; }

@test "empty" { }

function marked_by_comment { # @test
  if true; then
    echo yes
  else
    echo no
  fi
}

teardown() {
  rm -rf "$BATS_TEST_TMPDIR/scratch"
}
