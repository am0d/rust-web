RUST_FLAGS = -L . -O

LIBS := libhttp

#ALL_SOURCES := $(wildcard src/*.rs)
ALL_SOURCES := src/utils.rs src/models.rs src/views.rs src/todo_controller.rs
BINARIES := build/server

ALL_OBJS := $(ALL_SOURCES:src/%.rs=build/%)

all: $(LIBS) $(BINARIES)

run: $(BINARIES)
	build/server

build/%: src/%.rs
	@echo Compiling $<
	@rustc $< -L rust-http/build -L build/ --lib -o $@
	@touch $@

build/server: src/main.rs $(ALL_OBJS)
	@echo Compiling $<
	@rustc src/main.rs -L ./rust-http/build/ -L build/ -o $@

libhttp:
	@echo Compiling libhttp
	@cd rust-http; $(MAKE) $(MFLAGS)

clean:
	@echo "Cleaning ..."
	@rm -f build/*.so build/*.o $(BINARIES) $(ALL_OBJS) build/lib*

cleanall: clean
	@cd rust-http; $(MAKE) $(MFLAGS) clean
