
.SUFFIXES = .asm .bin .lst .rs .ld .img .sys

SRCDIR = src
BUILDDIR = build
OSNAME = dekoos

vpath %.asm $(SRCDIR)
vpath %.rs $(SRCDIR)
vpath %.o $(BUILDDIR)

TARGET=haribote.img

.PHONY: run clean install build init

install:
	docker-compose up --abort-on-container-exit

build:
	@mkdir -p $(BUILDDIR)
	@make $(TARGET)

$(TARGET): $(BUILDDIR)/ipl10.bin $(BUILDDIR)/haribote.sys
	mformat -f 1440 -C -B $< -i $(TARGET)
	mcopy $(filter-out $<,$^) -i $(TARGET) ::

$(BUILDDIR)/haribote.sys: $(BUILDDIR)/asmhead.bin $(BUILDDIR)/bootpack.bin
	cat $^ > $@

$(BUILDDIR)/osfunc.o: osfunc.asm
	nasm -f elf32 $(SRCDIR)/osfunc.asm -o $(BUILDDIR)/osfunc.o -l $(BUILDDIR)/osfunc.lst

$(BUILDDIR)/bootpack.o: bootpack.rs
	rustc --target=i686-unknown-linux-gnu --crate-type=staticlib --emit=obj -C lto -C no-prepopulate-passes -C relocation-model=static -Z verbose -Z no-landing-pads -o $(BUILDDIR)/bootpack.o $<

$(BUILDDIR)/bootpack.bin: bootpack.o osfunc.o
	i686-unknown-linux-gnu-ld -v -nostdlib -Tdata=0x00310000 $(BUILDDIR)/bootpack.o $(BUILDDIR)/osfunc.o -T $(SRCDIR)/kernel.ld -o $@

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


