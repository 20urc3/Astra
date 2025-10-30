#include <stdint.h>
#include <stdio.h>
#include <sanitizer/coverage_interface.h>


void __sanitizer_cov_trace_pc_guard(uint32_t *guard) {
  if (!*guard) return;  
  void *PC = __builtin_return_address(0);
  char PcDescr[1024];
  __sanitizer_symbolize_pc(PC, "%p %F %L", PcDescr, sizeof(PcDescr));
  printf("guard: %p %x PC %s\n", guard, *guard, PcDescr);
}