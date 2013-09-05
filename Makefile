RUST_FLAGS = -L . -O

LIBS := libhttp

ALL_SOURCES := $(wildcard src/*.rs)
BINARIES := build/server

ALL_BINARIES := $(BINARIES) $(ALL_SOURCES:%.rs=lib%)

BINARIES := $(filter-out $(LIBS), $(BINARIES))


all: $(LIBS) $(BINARIES)

run: $(BINARIES)
	build/server

build/server: $(ALL_SOURCES)
	rustc $< -L ./rust-http/build/ -o $@

libhttp:
	cd rust-http; $(MAKE) $(MFLAGS)

lib%: %.rs
	rustc $(RUST_FLAGS) $<
	@touch $@

clean:
	@echo "Cleaning ..."
	@rm -f *.so $(ALL_BINARIES)
