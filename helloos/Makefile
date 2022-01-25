default :
	../tolenv/z_tools/make img

ipl.bin : ipl.nas Makefile
	../tolenv/z_tools/nask ipl.nas ipl.bin ipl.lst

helloos.img : ipl.bin Makefile
	../tolenv/z_tools/edimg imgin:../tolenv/z_tools/fdimg0at.tek wbinimg src:ipl.bin len:512 from:0 to:0 imgout:helloos.img

img :
	../tolenv/z_tools/make -r helloos.img

asm :
	../tolenv/z_tools/make -r ipl.bin

run :
	../tolenv/z_tools/make img
	cp helloos.img ../tolenv/z_tools/qemu/fdimage0.bin
	../tolenv/z_tools/make -C ../tolenv/z_tools/qemu

clean :
	-del ipl.bin
	-del ipl.lst

src_only :
	../tolenv/z_tools/make clean
	-del helloos.img
