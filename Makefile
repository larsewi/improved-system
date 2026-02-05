CFLAGS = -g -Wall -Wextra -Wconversion
CPPFLAGS = -Iinclude
LDFLAGS = -Ltarget/debug
LDLIBS = -limproved_system

.PHONY: all check diff commit

all: tests/.workdir/config.toml tests/isys

tests/.workdir:
	mkdir -p tests/.workdir

tests/.workdir/config.toml: tests/.workdir tests/config.toml
	cp tests/config.toml tests/.workdir/

tests/isys: tests/main.o
	$(CC) $^ -o $@ $(LDFLAGS) $(LDLIBS) -Wl,-rpath,'$$ORIGIN/../target/debug'

tests/main.o: tests/main.c
	$(CC) $(CFLAGS) $(CPPFLAGS) -c $< -o $@

commit: tests/isys tests/.workdir/config.toml
	./tests/isys tests/.workdir commit

diff: tests/isys tests/.workdir/config.toml
	./tests/isys tests/.workdir diff

clean:
	rm -f tests/main.o
	rm -f tests/isys
	rm -rf tests/.workdir
