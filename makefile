# $@ = target file
# $< = first dependency
# $^ = all dependencies

OBJECTS := $(patsubst %.asm,%.o,$(shell find asm/ -name '*.asm'))

default: build/kernel

$(OBJECTS): %.o: %.asm
	nasm $< -f elf64 -o $@

build/kernel: $(OBJECTS)
# Build the libaxol.a
	cargo build --release --target x86_64-axol.json -Z build-std=core -Z build-std-features=compiler-builtins-mem
# Build the kernel.bin
	ld -n -o build/isofiles/boot/kernel.bin -T build/linker.ld $(OBJECTS) target/x86_64-axol/release/libaxol.a
# Build the kernel.iso
	grub-mkrescue --compress=xz -o build/kernel.iso build/isofiles

clean:
	cargo clean
	rm -r $(OBJECTS) build/isofiles/boot/kernel.bin build/kernel.iso