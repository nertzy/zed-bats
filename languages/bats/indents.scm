(compound_statement
  "}" @end) @indent

(subshell
  ")" @end) @indent

(do_group
  "done" @end) @indent

(if_statement
  "fi" @end) @indent

(case_statement
  "esac" @end) @indent

(case_item) @indent

(array
  ")" @end) @indent

(command_substitution
  ")" @end) @indent
