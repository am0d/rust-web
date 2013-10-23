RUST_FLAGS = -L . -O

LIBS := libhttp libpcre
LINK_FLAGS := -L rust-http/build/ -L rust-pcre/ -L build/

#ALL_SOURCES := $(wildcard src/*.rs)
ALL_SOURCES := src/utils.rs src/models.rs src/views.rs src/router.rs src/todo_controller.rs
BINARIES := build/server

ALL_OBJS := $(ALL_SOURCES:src/%.rs=build/%)

all: $(LIBS) $(BINARIES)

run: $(BINARIES)
	build/server

build/%: src/%.rs $(LIBS)
	@echo Compiling $<
	@rustc $< $(LINK_FLAGS) --lib --out-dir build/
	@touch $@

build/server: src/main.rs $(ALL_OBJS)
	@echo Compiling $<
	@rustc src/main.rs $(LINK_FLAGS) -o $@

libhttp:
	@echo Compiling libhttp
	@cd rust-http; $(MAKE) $(MFLAGS)

libpcre:
	@echo Compiling libpcre
	@cd rust-pcre; $(MAKE) $(MFLAGS)

clean:
	@echo "Cleaning ..."
	@rm -f build/*.so build/*.o $(BINARIES) $(ALL_OBJS) build/lib*

cleanall: clean
	@cd rust-http; $(MAKE) $(MFLAGS) clean
	@cd rust-pcre; $(MAKE) $(MFLAGS) clean
