#include <stdint.h>
#include <stdlib.h>

#include <improved.h>

int main() {
  init(".improved");
  int ret = commit();
  return (ret == 0) ? EXIT_SUCCESS : EXIT_FAILURE;
}
