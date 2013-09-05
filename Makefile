RUST_FLAGS = -L . -O

LIBS := libhttp

ALL_SOURCES := $(wildcard src/*.rs)
BINARIES := src/main

ALL_BINARIES := $(BINARIES) $(ALL_SOURCES:%.rs=lib%)

BINARIES := $(filter-out $(LIBS), $(BINARIES))


all: $(LIBS) $(BINARIES)

run: $(BINARIES)
	cd src; ./main

%: %.rs 
	rustc src/main.rs -L ./rust-http/build/

libhttp:
	cd rust-http; $(MAKE) $(MFLAGS)

lib%: %.rs
	rustc $(RUST_FLAGS) $<
	@touch $@

clean:
	@echo "Cleaning ..."
	@rm -f *.so $(ALL_BINARIES)
