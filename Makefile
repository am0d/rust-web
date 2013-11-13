RUST_FLAGS = -L . -O

LIBS := libhttp libpcre
LINK_FLAGS := -L rust-http/lib/x86_64-unknown-linux-gnu/ -L pcre/lib/x86_64-unknown-linux-gnu/ -L build/ #TODO use rustpkg and remove the need for these hardcoded paths

ALL_SOURCES := $(wildcard src/*.rs)
#ALL_SOURCES := src/utils.rs src/models.rs src/views.rs src/todo_controller.rs src/router.rs 
BINARIES := build/server

ALL_OBJS := $(ALL_SOURCES:src/%.rs=build/%.o)
ALL_TESTS := $(ALL_SOURCES:src/%.rs=build/%)

all: $(LIBS) $(BINARIES)

run: $(BINARIES)
	build/server

build/server: $(ALL_SOURCES) 
	@echo Compiling $@
	@rustc src/main.rs $(LINK_FLAGS) -o $@

libhttp:
	@echo Compiling libhttp
	@cd rust-http; $(MAKE) $(MFLAGS)

libpcre:
	@echo Compiling libpcre
	@cd pcre; $(MAKE) install $(MFLAGS)

check: libhttp libpcre build/test
	@./$<

quickcheck: build/test
	@./$<

build/test: src/test.rs $(ALL_SOURCES)
	@echo Compiling $@ in test mode
	@rustc $< $(LINK_FLAGS) --test --out-dir build/

clean:
	@echo "Cleaning ..."
	@rm -f build/* $(BINARIES)

cleanall: clean
	@cd rust-http; $(MAKE) $(MFLAGS) clean
	@cd rust-pcre; $(MAKE) $(MFLAGS) clean

.PHONY: check clean cleanall run
