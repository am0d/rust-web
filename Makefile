RUST_FLAGS = -L . -O

LIBS := libhttp

#ALL_SOURCES := $(wildcard src/*.rs)
ALL_SOURCES := src/utils.rs src/todo_controller.rs
BINARIES := build/server

ALL_OJBS := $(ALL_SOURCES:src/%.rs=build/%.o)

all: $(LIBS) $(BINARIES)

run: $(BINARIES)
	build/server

build/%.o: src/%.rs
	rustc $< -L rust-http/build -L build/ --lib -o $@

build/server: src/main.rs $(ALL_OJBS)
	rustc src/main.rs -L ./rust-http/build/ -L build/ -o $@

libhttp:
	cd rust-http; $(MAKE) $(MFLAGS)

lib%: %.rs
	rustc $(RUST_FLAGS) $<
	@touch $@

clean:
	@echo "Cleaning ..."
	@rm -f build/*.so build/*.o $(ALL_BINARIES)
