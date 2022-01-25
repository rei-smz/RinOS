default :
	../z_tools/make img

ipl.bin : ipl.nas Makefile
	../z_tools/nask ipl.nas ipl.bin ipl.lst

helloos.img : ipl.bin Makefile
	../z_tools/edimg imgin:../z_tools/fdimg0at.tek wbinimg src:ipl.bin len:512 from:0 to:0 imgout:helloos.img

img :
	../z_tools/make -r helloos.img

asm :
	../z_tools/make -r ipl.bin

run :
	../z_tools/make img
	cp helloos.img ../z_tools/qemu/fdimage0.bin
	../z_tools/make -C ../z_tools/qemu

clean :
	-del ipl.bin
	-del ipl.lst

src_only :
	../z_tools/make clean
	-del helloos.img
