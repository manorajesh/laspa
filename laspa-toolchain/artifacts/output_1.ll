; ModuleID = 'main'
source_filename = "main"

define double @main() {
entry:
  %i = alloca double, align 8
  store double 1.000000e+01, ptr %i, align 8
  %d = alloca double, align 8
  store double 2.000000e+00, ptr %d, align 8
  %i1 = load double, ptr %i, align 8
  %d2 = load double, ptr %d, align 8
  %calltmp = call double @sum(double %i1, double %d2)
  %z = alloca double, align 8
  store double %calltmp, ptr %z, align 8
  %z3 = load double, ptr %z, align 8
  ret double %z3
  ret double %z3
}

define double @sum(double %0, double %1) {
entry:
  %x = alloca double, align 8
  store double %0, ptr %x, align 8
  %y = alloca double, align 8
  store double %1, ptr %y, align 8
  %x1 = load double, ptr %x, align 8
  %y2 = load double, ptr %y, align 8
  %addtmp = fadd double %x1, %y2
  ret double %addtmp
}
