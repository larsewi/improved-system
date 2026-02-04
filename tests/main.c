#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include <improved.h>

int main(int argc, char *argv[]) {
  if (argc < 3) {
    return EXIT_FAILURE;
  }

  int ret = isys_init(argv[1]);
  if (ret != 0) {
    return EXIT_FAILURE;
  }

  if (strcmp(argv[2], "commit") == 0) {
    ret = isys_commit();
  }
  else {
    return EXIT_FAILURE;
  }

  return (ret == 0) ? EXIT_SUCCESS : EXIT_FAILURE;
}
