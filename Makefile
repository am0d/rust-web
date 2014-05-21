RUST_FLAGS = -L . -O

LIBS := libhttp libpcre
LINK_FLAGS := -L rust-http/build/ -L rust-pcre/lib/ -L build/ #TODO use rustpkg and remove the need for these hardcoded paths
ifdef PCRE_LIBDIR
LINK_FLAGS += -L $(PCRE_LIBDIR)
endif

#TODO These two lines below should only need the first wildcard - work out why that isn't working ...
WEB_SOURCES := $(wildcard src/web/**/*.rs) $(wildcard src/web/*.rs) $(wildcard src/web/**/**/*.rs)
BINARIES := build/server

ALL_OBJS := $(ALL_SOURCES:src/%.rs=build/%.o)
ALL_TESTS := $(ALL_SOURCES:src/%.rs=build/%)

all: $(LIBS) $(BINARIES)

run: $(BINARIES)
	build/server

build/server: $(WEB_SOURCES)
	@echo Compiling $@
	@mkdir -p build/
	@rustc src/web/main.rs $(RUST_FLAGS) $(LINK_FLAGS) -o $@

libhttp:
	@echo Compiling libhttp
	@cd rust-http; ./configure && $(MAKE) $(MFLAGS)

libpcre:
	@echo Compiling libpcre
	cd rust-pcre; $(MAKE) install $(MFLAGS)

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
	@cd rust-pcre; $(MAKE) $(MFLAGS) clean

.PHONY: check clean cleanall run libhttp libpcre
