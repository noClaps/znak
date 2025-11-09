; See runtime/queries/ecma/README.md for more info.

; Definitions
;------------
; Javascript and Typescript Treesitter grammars deviate when defining the
; tree structure for parameters, so we need to address them in each specific
; language instead of ecma.

; (i)
(formal_parameters 
  (identifier) @local.definition.variable.parameter)

; (i = 1)
(formal_parameters 
  (assignment_pattern
    left: (identifier) @local.definition.variable.parameter))

; Scopes
;-------

[
  (statement_block)
  (arrow_function)
  (function_expression)
  (function_declaration)
  (method_definition)
  (for_statement)
  (for_in_statement)
  (catch_clause)
  (finally_clause)
] @local.scope

; Definitions
;------------

; i => ...
(arrow_function
  parameter: (identifier) @local.definition.variable.parameter)

; References
;------------

(identifier) @local.reference

