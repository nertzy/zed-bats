; Tests and functions are Vim's functions: `af`/`if`, `[m`/`]m`.
(test_block
  body: (compound_statement
    "{"
    (_)* @function.inside
    "}")) @function.around

(function_definition
  body: (_
    "{"
    (_)* @function.inside
    "}")) @function.around

(function_definition
  body: (subshell
    "("
    (_)* @function.inside
    ")")) @function.around

[(comment) (test_marker_comment)]+ @comment.around
