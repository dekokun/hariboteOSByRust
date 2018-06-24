
.SUFFIXES = .asm .bin .lst .rs .ld .img .sys

SRCDIR = src
BUILDDIR = build
OSNAME = dekoos

vpath %.asm $(SRCDIR)
vpath %.rs $(SRCDIR)

TARGET=haribote.img

.PHONY: run clean install build init

install:
	docker-compose up --exit-code-from haribote-rust

build:
	@mkdir -p $(BUILDDIR)
	@make $(TARGET)

$(TARGET): $(BUILDDIR)/ipl10.bin $(BUILDDIR)/haribote.sys
	mformat -f 1440 -C -B $< -i $(TARGET)
	mcopy $(filter-out $<,$^) -i $(TARGET) ::

$(BUILDDIR)/haribote.sys: $(BUILDDIR)/asmhead.bin $(BUILDDIR)/bootpack.bin
	cat $^ > $@

$(BUILDDIR)/%.bin: %.rs
	rustc --target=i686-unknown-linux-gnu --crate-type=staticlib --emit=obj -C lto -C no-prepopulate-passes -Z verbose -Z no-landing-pads -o $(BUILDDIR)/$*.o $<
	i686-unknown-linux-gnu-ld -v -nostdlib -Tdata=0x00310000 $(BUILDDIR)/$*.o -T $(SRCDIR)/kernel.ld -o $@

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


