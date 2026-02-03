#include <stdint.h>
#include <stdlib.h>

#include <improved.h>

int main() {
  init();
  int ret = commit();
  if (ret == -1) {
    return EXIT_FAILURE;
  }
  return EXIT_SUCCESS;
}
