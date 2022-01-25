# RinOS
本项目为《30天自制操作系统》（[日]川合秀实 著）的实现。
## 说明
书中C语言实现部分在本项目中采用rust实现。书中的汇编部分采用rust内联汇编以及nask（原书作者创造的一种类nasm汇编语言）实现。关于nask，请参考原书。

编译时，会先将rust部分编译为`.a`文件，再通过ld将这个文件与nask编写的部分链接生成完整的系统，最后使用原书作者提供的img制作工具将引导部分与系统整合成软盘镜像。
## 使用
`cd [需要测试的程序目录（如helloos）]`

`make run`

请确保您使用的环境是**Linux**（不限发行版），且已安装**qemu**、**make**和**ld**。更多依赖相关信息请参考z_tools_linux的文档。
## 参考
rust编译方案： \
https://github.com/yoshitsugu/hariboteos_in_rust \
ld自定义链接脚本： \
https://github.com/kotetuco/ructiss/blob/master/kernel/arch/i686-unknown-linux-gnu/kernel.ld \
原书所使用的编译相关工具的Linux版本： \
https://github.com/HariboteOS/z_tools_linux \
原书源代码（中文翻译注释）： \
https://github.com/yourtion/30dayMakeOS
