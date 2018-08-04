
.SUFFIXES = .asm .bin .lst .rs .ld .img .sys

SRCDIR = src
BUILDDIR = build
OSNAME = dekoos

vpath %.asm $(SRCDIR)
vpath %.rs $(SRCDIR)
vpath %.o $(BUILDDIR)

TARGET=haribote.img
TARGET_TRIPLE=i686-unknown-none

.PHONY: run clean install build init

install:
	docker-compose up --abort-on-container-exit

build:
	@mkdir -p $(BUILDDIR)
	@make $(TARGET)

$(TARGET): $(BUILDDIR)/ipl10.bin $(BUILDDIR)/haribote.sys
	mformat -f 1440 -C -B $< -i $(TARGET)
	mcopy $(filter-out $<,$^) -i $(TARGET) ::

$(SRCDIR)/hankaku.rs: hankaku.rb hankaku.txt
	./hankaku.rb hankaku.txt > $@


$(BUILDDIR)/haribote.sys: $(BUILDDIR)/asmhead.bin $(BUILDDIR)/bootpack.bin
	cat $^ > $@

$(BUILDDIR)/osfunc.o: osfunc.asm Makefile
	nasm -f elf32 $(SRCDIR)/osfunc.asm -o $(BUILDDIR)/osfunc.o -l $(BUILDDIR)/osfunc.lst

target/$(TARGET_TRIPLE)/debug/libdekoos.a: bootpack.rs hankaku.rs Cargo.toml Makefile
	# RUSTFLAGS='-C relocation-model=dynamic-no-pic -Z external-macro-backtrace' RUST_TARGET_PATH=$(PWD) rustup run nightly `which xargo` build -v --target=$(TARGET_TRIPLE) --manifest-path Cargo.toml
	RUSTFLAGS='-C relocation-model=dynamic-no-pic -Z external-macro-backtrace' RUST_TARGET_PATH=$(PWD) rustup run nightly-2018-06-29 `which xargo` build -v --target=$(TARGET_TRIPLE) --manifest-path Cargo.toml

$(BUILDDIR)/bootpack.bin: target/$(TARGET_TRIPLE)/debug/libdekoos.a osfunc.o
	i686-unknown-linux-gnu-ld -v -nostdlib -Tdata=0x00310000 target/$(TARGET_TRIPLE)/debug/libdekoos.a $(BUILDDIR)/osfunc.o -T $(SRCDIR)/kernel.ld -o $@

$(BUILDDIR)/%.bin: %.asm
	nasm $< -o $@ -l $(BUILDDIR)/$*.lst

run: install
	VBoxManage unregistervm $(OSNAME) --delete
	VBoxManage createvm --name $(OSNAME) --ostype Other --register
	VBoxManage modifyvm  $(OSNAME) --ostype Other --memory 32
	VBoxManage storagectl $(OSNAME) --name Floppy --add floppy
	VBoxManage storageattach $(OSNAME) --storagectl Floppy --device 0 --medium $(TARGET)
	VBoxManage startvm $(OSNAME)

clean:
	$(RM) $(TARGET)
	$(RM) -r $(BUILDDIR)


