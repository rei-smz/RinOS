TOOLPATH = ../z_tools
MAKE = $(TOOLPATH)/make -r
NASK = $(TOOLPATH)/nask
EDIMG = $(TOOLPATH)/edimg
QEMU = $(TOOLPATH)/qemu
BOCHS = bochs -f bochsrc
IPL = ipl.bin
IMG = haribote.img


default :
	$(MAKE) img

%.bin : %.nas Makefile
	$(NASK) $*.nas $*.bin $*.lst

$(IMG) : ipl.bin haribote.sys Makefile
	$(EDIMG) imgin:$(TOOLPATH)/fdimg0at.tek \
	wbinimg src:ipl.bin len:512 from:0 to:0 \
	copy from:haribote.sys to:@: \
	imgout:$(IMG)

libharibote_os.a : ./src/
	cargo xbuild
	cp ./target/i686-haribote/debug/libharibote_os.a .

kernel.bin : ./libharibote_os.a
	ld -v -nostdlib -m elf_i386 -Tdata=0x00310000 -T kernel.ld $<  -o $@

haribote.sys : ./asmhead.bin ./kernel.bin
	cat $^ > $@

img :
	$(MAKE) $(IMG)

asm :
	$(MAKE) $(IPL)

run :
	$(MAKE) img
	cp $(IMG) a.img
	$(BOCHS)

qemu:
	$(MAKE) img
	cp $(IMG) $(QEMU)/fdimage0.bin
	$(MAKE) -C $(QEMU)

clean :
	rm *.a *.img *.bin
	rm -rf target

src_only :
	$(TOOLPATH)/make clean
	-del $(IMG)
