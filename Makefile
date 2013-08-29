RUST_FLAGS = -L . -O

LIBS := libhttp

ALL_SOURCES := $(wildcard *.rs)
BINARIES := $(ALL_SOURCES:.rs=)

ALL_BINARIES := $(BINARIES) $(ALL_SOURCES:%.rs=lib%)

BINARIES := $(filter-out $(LIBS), $(BINARIES))


all: $(LIBS) $(BINARIES)

%: %.rs 
	rustc $(RUST_FLAGS) $<

libhttp:
	cd rust-http; $(MAKE) $(MFLAGS)

lib%: %.rs
	rustc $(RUST_FLAGS) $<
	@touch $@

clean:
	@echo "Cleaning ..."
	@rm -f *.so $(ALL_BINARIES)
