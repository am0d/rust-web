RUST_FLAGS = -L . -O

LIBS := libhttp libpcre
LINK_FLAGS := -L rust-http/build/ -L pcre/lib/ -L build/ #TODO use rustpkg and remove the need for these hardcoded paths

#TODO These two lines below should only need the first wildcard - work out why that isn't working ...
WEB_SOURCES := $(wildcard src/web/**/*.rs) $(wildcard src/web/*.rs) $(wildcard src/web/**/**/*.rs)
COMPILER_SOURCES := $(wildcard src/compiler/*.rs) $(wildcard src/compiler/**/*.rs)
BINARIES := build/compiler build/server

ALL_OBJS := $(ALL_SOURCES:src/%.rs=build/%.o)
ALL_TESTS := $(ALL_SOURCES:src/%.rs=build/%)

all: $(LIBS) $(BINARIES)

compiler: build/compiler

build/compiler: $(COMPILER_SOURCES)
	@echo Compiling $@
	@rustc src/compiler/main.rs $(LINK_FLAGS) -o $@

run: $(BINARIES)
	build/server

build/server: $(WEB_SOURCES) 
	@echo Compiling $@
	@rustc src/web/main.rs $(RUST_FLAGS) $(LINK_FLAGS) -o $@

libhttp:
	@echo Compiling libhttp
	@cd rust-http; $(MAKE) $(MFLAGS)

libpcre:
	@echo Compiling libpcre
	@cd pcre; $(MAKE) install $(MFLAGS)

check: build/test
	@./$<

build/test: src/web/test.rs $(WEB_SOURCES)
	@echo Compiling $@ in test mode
	@rustc $< $(LINK_FLAGS) $(RUST_FLAGS) --test --out-dir build/

clean:
	@echo "Cleaning ..."
	@rm -f build/* $(BINARIES)

cleanall: clean
	@cd rust-http; $(MAKE) $(MFLAGS) clean
	@cd pcre; $(MAKE) $(MFLAGS) clean

.PHONY: check clean cleanall run 
