CFLAGS = -g -Wall -Wextra -Wconversion
CPPFLAGS = -Iinclude
LDFLAGS = -Ltarget/debug
LDLIBS = -limproved_system

.PHONY: all check phony

all:
	cargo build

check: helloworld
	RUST_LOG=info ./helloworld

helloworld: tests/helloworld.o
	$(CC) $^ -o $@ $(LDFLAGS) $(LDLIBS) -Wl,-rpath,'$$ORIGIN/target/debug'

helloworld.o: tests/helloworld.c
	$(CC) $(CFLAGS) $(CPPFLAGS) -c $< -o $@

clean:
	rm -f tests/helloworld.o
	rm -f helloworld
	rm -rf .improved
