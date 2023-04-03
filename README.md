# hello_plot

### 本软件介绍(Software Introduction)

* 中文

这是一个配合官方**bladebit_cuda**绘图仪的分发软件.

我在使用**bladebit_cuda**进行绘图时,发现它的绘图速度是如此之**快**,以至于我急需一个分发软件,来让热乎的.plot文件**并发地转入多个机械硬盘**之中.

于是我开始查找已经存在的该类软件,我承认它们都很优秀,并衷心地感谢那些愿意开源的作者们,但我需要精度更细的操控和更加简单的使用教程.

我希望在分发给每个硬盘时,软件能**自动地优先写入剩余空间最充足的硬盘目录**,并且我能**控制每个硬盘分流的速度**(这在防止smba内存占用爆炸方面有奇效),而且我还想时刻看到已经转移多少byte,转移了多少图.我花了很多时间来寻找这样的开源软件.

但后来我突然意识到,我他妈可以自己写一个呀,这就是目前你们看到的版本,**它真正来源于我的实际生产过程,它应该属于全体劳动人民!**

* English
This is a distribution software that works in conjunction with the official **bladebit_cuda** plotter.

While using **bladebit_cuda** for plotting, I noticed that it was incredibly **fast**. However, I urgently needed a distribution software to **concurrently transfer .plot files to multiple mechanical hard drives**.

I started looking for existing software and while I admit that they were all excellent, I needed more precise control and simpler usage instructions.

I wanted the software to **automatically prioritize writing to the most available hard drive directories** when distributing to each hard drive. Additionally, I wanted to be able to **control the transfer speed of each hard drive** to prevent memory usage explosions in Samba. Finally, I wanted to constantly monitor the transfer progress, including the number of bytes and plots transferred.

After spending a lot of time searching for open-source software that met my needs, I suddenly realized that I could write one myself. This is the version you are currently seeing, which truly comes from my actual production process and **belongs to all the working people**!

### 使用教程(Usage Instructions)

* 中文

解压后,我们需要设置配置文件:userset.json

![关于如何设置参数](/how_to_use.jpg "参数设置")

**注意**:所有的目录路径都不要使用"\\",而应该使用"/"

若ubuntu系统,在程序所在目录,打开终端(Ctl + Alt + T),首先赋予分软件权限:

``
$ sudo chmod +x hello
``

然后,运行它

``
$ sudo ./hello
``

若windows系统,在程序所在目录下,打开cmd,然后输入:

``
$  ./hello_plot.exe
``

如果显示比较乱的话,请将终端窗口变大一点,使其中的表格能更适应窗口.

**注意**:因为ubuntu系统允许更大的堆栈,本软件在ubuntu下的执行效率比Windows高3倍,**如果在Windows传输速度过低,请更换ubuntu系统进行绘图和分发**

* English

After extracting, we need to configure the userset.json file.

Attention: Do not use "\\" in any directory paths, use "/" instead.

If using Ubuntu system, open the terminal (Ctrl + Alt + T) in the directory where the program is located, and first grant executable permissions to the software.

``
$ sudo chmod +x hello
``

Then,run it:

``
$ sudo ./hello
``

If using Windows,open the CMD and run it:

``
$  ./hello_plot.exe
``

### 若正常运行(If running normally.)

![若正常运行](/run_normaly.png "运行正常")