; ModuleID = 'main'
source_filename = "main"

define double @main() {
entry:
  %tmp = call double @collatz(double 1.230000e+02)
  ret double %tmp
  ret double %tmp
}

define double @collatz(double %n) {
entry:
  %n1 = alloca double, align 8
  store double %n, ptr %n1, align 8
  br label %loop_cond

loop_cond:                                        ; preds = %end_if, %entry
  %n2 = load double, ptr %n1, align 8
  %gttmp = fcmp ogt double %n2, 1.000000e+00
  br i1 %gttmp, label %loop_body, label %loop_end

loop_body:                                        ; preds = %loop_cond
  br label %if_cond

loop_end:                                         ; preds = %loop_cond
  %n7 = load double, ptr %n1, align 8
  ret double %n7

if_cond:                                          ; preds = %loop_body
  %n3 = load double, ptr %n1, align 8
  %modtmp = frem double %n3, 2.000000e+00
  %eqttmp = fcmp oeq double %modtmp, 0.000000e+00
  br i1 %eqttmp, label %then_block, label %else_block

then_block:                                       ; preds = %if_cond
  %n4 = load double, ptr %n1, align 8
  %divtmp = fdiv double %n4, 2.000000e+00
  store double %divtmp, ptr %n1, align 8
  br label %end_if

else_block:                                       ; preds = %if_cond
  %n5 = load double, ptr %n1, align 8
  %multmp = fmul double 3.000000e+00, %n5
  %addtmp = fadd double %multmp, 1.000000e+00
  store double %addtmp, ptr %n1, align 8
  br label %end_if

end_if:                                           ; preds = %else_block, %then_block
  %n6 = load double, ptr %n1, align 8
  %printcall = call double @print_f64(double %n6)
  br label %loop_cond
}

declare double @print_f64(double)
