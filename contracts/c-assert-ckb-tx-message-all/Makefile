# We cannot use $(shell pwd), which will return unix path format on Windows,
# making it hard to use.
cur_dir = $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

TOP := $(cur_dir)
MODE := release
# Tweak this to change the clang version to use for building C code. By default
# we use a bash script with somes heuristics to find clang in current system.
CLANG := $(shell $(TOP)/scripts/find_clang)
OBJCOPY := $(subst clang,llvm-objcopy,$(CLANG))
CLANG_FORMAT := $(subst clang,clang-format,$(CLANG))
# When this is set to some value, the generated binaries will be copied over
BUILD_DIR :=
# Generated binaries to copy. By convention, a Rust crate's directory name will
# likely match the crate name, which is also the name of the final binary.
# However if this is not the case, you can tweak this variable. As the name hints,
# more than one binary is supported here.
BINARY := $(notdir $(shell pwd))

ifeq (release,$(MODE))
	MODE_ARGS := -Os
endif

default: build

build:
	$(CLANG) --target=riscv64 -march=rv64imc_zba_zbb_zbc_zbs \
		-Wall -Werror -fdata-sections -ffunction-sections \
		$(MODE_ARGS) -g -nostdlib -nostdinc \
		-I $(TOP)/deps/ckb-c-stdlib -I $(TOP)/deps/ckb-c-stdlib/libc \
		-Wl,-static -Wl,--gc-sections \
		main.c -o $(BINARY)
	cp $(BINARY) $(TOP)/$(BUILD_DIR)/$(BINARY).debug
	cp $(BINARY) $(TOP)/$(BUILD_DIR)/$(BINARY)
	$(OBJCOPY) --strip-debug --strip-all $(TOP)/$(BUILD_DIR)/$(BINARY)

fmt:
	$(CLANG_FORMAT) -i --style=Google *.c *.h

clean:
	rm -rf $(BINARY) *.o

.PHONY: build fmt clean
