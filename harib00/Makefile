TOOLPATH = ../tolenv/z_tools
MAKE = $(TOOLPATH)/make -r
NASK = $(TOOLPATH)/nask
EDIMG = $(TOOLPATH)/edimg
QEMU = $(TOOLPATH)/qemu
IPL = ipl.bin
IMG = haribote.img


default :
	$(MAKE) img

$(IPL) : ipl.nas Makefile
	$(NASK) ipl.nas ipl.bin ipl.lst

asmhead.bin : asmhead.nas Makefile
	$(NASK) asmhead.nas asmhead.bin asmhead.lst

$(IMG) : ipl.bin haribote.sys Makefile
	$(EDIMG) imgin:$(TOOLPATH)/fdimg0at.tek \
	wbinimg src:ipl.bin len:512 from:0 to:0 \
	copy from:haribote.sys to:@: \
	imgout:$(IMG)

libharibote_os.a : ./src/lib.rs
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
	cp $(IMG) $(QEMU)/fdimage0.bin
	$(MAKE) -C $(QEMU)

clean :
	-del ipl.bin
	-del ipl.lst
	-del asmhead.bin
	-del libharibote_os.a

src_only :
	$(TOOLPATH)/make clean
	-del $(IMG)
