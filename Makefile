RUSTC = rustc

OUT_DIR ?= tmp

MAIN_SOURCE = src/termutils.rs
OTHER_SOURCES = src/hexes.rs src/ios.rs src/util.rs src/trie.rs
ifdef CURSES
OTHER_SOURCES += src/info/curses.rs
CFG = --cfg curses
else
OTHER_SOURCES += src/info/builtin.rs
CFG =
endif
TESTS = bin/termios bin/termios2 bin/termios3 bin/rl bin/password bin/attrs bin/tput bin/keys bin/scroll

all: build tests

build: $(OUT_DIR)/built

check: build
	$(RUSTC) $(CFG) -L $(OUT_DIR) --test $(MAIN_SOURCE) -o TEST
	./TEST
	@rm -f TEST

tests: $(TESTS)

bin/%: test/%.rs $(OUT_DIR)/built
	@mkdir -p bin
	$(RUSTC) --out-dir bin -L $(OUT_DIR) -L lib $<

$(OUT_DIR)/built: $(MAIN_SOURCE) $(OTHER_SOURCES) $(OUT_DIR)/libtermios_wrapper.a $(OUT_DIR)/libio_helper.a
	@mkdir -p lib
	$(RUSTC) $(CFG) --out-dir lib -L $(OUT_DIR) $(MAIN_SOURCE) && touch $(OUT_DIR)/built

clibs: $(OUT_DIR)/libtermios_wrapper.a $(OUT_DIR)/libio_helper.a

$(OUT_DIR)/libtermios_wrapper.a: $(OUT_DIR)/termios_wrapper.o
	ar cr $@ $<

$(OUT_DIR)/termios_wrapper.o: src/termios_wrapper.c
	@mkdir -p $(OUT_DIR)
	cc -fPIC -c -o $@ $<

$(OUT_DIR)/libio_helper.a: $(OUT_DIR)/io_helper.o
	ar cr $@ $<

$(OUT_DIR)/io_helper.o: src/io_helper.c
	@mkdir -p $(OUT_DIR)
	cc -fPIC -c -o $@ $<

clean:
	-@rm -rf lib/ bin/ $(OUT_DIR)/

.PHONY: all build check tests clibs clean
