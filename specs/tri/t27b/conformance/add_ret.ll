; LLVM IR milestone fixture: one function, one add, one ret.
; Parsed by the t27b LLVM IR importer behind import_door into IRModule.
; Safe subset: integer function, pure, no dynamic allocation, no closures, no async.

define i32 @add(i32 %a, i32 %b) {
entry:
  %r = add i32 %a, %b
  ret i32 %r
}
