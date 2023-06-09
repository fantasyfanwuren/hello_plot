﻿# hello_plot(该工具已经停止维护,请移步到它的重构版本hello_plot_max:https://github.com/fantasyfanwuren/hello_plot_max)
 

## Introduction in English below the introduction in Chinese

### 本软件介绍

这是一个配合官方**bladebit_cuda**绘图仪的分发软件.

我在使用**bladebit_cuda**进行绘图时,发现它的绘图速度是如此之**快**,以至于我急需一个分发软件,来让热乎的.plot文件**并发地转入多个机械硬盘**之中.

于是我开始查找已经存在的该类开源软件,我承认它们都很优秀,并衷心地感谢那些愿意开源的作者们,但我需要精度更细的操控和更加简单的使用教程.

我希望分发软件能做到以下几点:

* 足够简明的教程和便捷的使用
* 自动地优先写入剩余空间最充足的硬盘目录
* 精准地控制每个硬盘分流的传输速度(防止smba内存占用爆炸)
* 时刻都能知道每个硬盘当前正在传输什么,传输了多少,传输速度是多大.
* 时刻都能知道每个硬盘传输了多少plot文件了,方便与计划进行对比.

我花了很多时间来寻找这样的开源软件.这些时间足够我亲手把它做出来了!

对呀,我他妈可以自己写一个呀,这就是目前你们看到的版本,**它真正来源于我的实际生产过程,它应该属于全体劳动人民!**

### 使用教程

#### 已不再对windows系统提供支持,因为它的效率要低一些,而且我自己也不用windows绘图,所以无法保证测试.

点开本页面右侧的"release"标签,下载最新的"release_ubuntu"字样的压缩包

解压到一个独立文件夹后,我们需要设置配置文件:userset.json,具体设置参照下图

![关于如何设置参数](/how_to_use.jpg "参数设置")

**注意**:所有的目录路径都不要使用"\\",而应该使用"/"

在程序所在目录,打开终端(Ctl + Alt + T),首先赋予分软件权限:

``
$ sudo chmod +x hello
``

然后,运行它

``
$ sudo ./hello
``

如果显示比较乱的话,请将终端窗口变大一点,使其中的表格能更适应窗口.


### 若正常运行

![若正常运行](/run_normaly.png "运行正常")

### 新增了日志查看功能

在软件运行后,可以查看日志:

在log目录下打开终端,执行:

``
$ tail -f requests.log
``

若希望调整日志级别,可以打开"log4rs.yaml"进行调整,默认是debug级别,这意味着当软件出错后,可以及时将日志发送给作者诊断错误的原因.

若不希望产生太多日志,可以将:

``
level: debug
``

改为:

``
level: info
``


### Software Introduction

This is a distribution software that works in conjunction with the official **bladebit_cuda** plotter.

While using **bladebit_cuda** for plotting, I noticed that it was incredibly **fast**. However, I urgently needed a distribution software to **concurrently transfer .plot files to multiple mechanical hard drives**.

I started looking for existing software and while I admit that they were all excellent, I needed more precise control and simpler usage instructions.

I hope the software distribution can achieve the following points:

* Sufficiently concise tutorials and convenient use
* Automatically prioritize writing to the hard drive directory with the most remaining space
* Precisely control the transmission speed of each hard drive branch to prevent smba memory usage explosion
* Always know what is currently being transmitted on each hard drive, how much has been transmitted, and what the transmission speed is.
* Always know how many plot files have been transferred to each hard drive, which is convenient for comparison with the plan.

After spending a lot of time searching for open-source software that met my needs, I suddenly realized that I could write one myself. This is the version you are currently seeing, which truly comes from my actual production process and **belongs to all the working people**!

### Usage Instructions

No longer providing support for Windows systems

After extracting, we need to configure the userset.json file.

* sourse_dir:The location where the BladeBit CUDA plotter stores the final plot file is also the starting point for distributing the software.

* hdisks:This represents the settings for each mechanical hard drive, where the settings for a single hard drive are enclosed in {} and the settings for all hard drives are enclosed in [], separated by commas. Please note the spacing and indentation are not critical.

* target_dir:Here you represent where you want to redirect the plot file.

* limit_num:The maximum allowed number of files to be transferred, which is related to the software's priority judgment. The more remaining files to be transferred, the higher the priority to transfer them.

* limit_rate:Represents the maximum transfer speed for incoming transfers. If the SMB protocol is deployed in the LAN and a memory explosion occurs, this number can be lowered.

Attention: Do not use "\\" in any directory paths, use "/" instead.

If using Ubuntu system, open the terminal (Ctrl + Alt + T) in the directory where the program is located, and first grant executable permissions to the software.

``
$ sudo chmod +x hello
``

Then,run it:

``
$ sudo ./hello
``

If the display is messy, please make the terminal window larger so that the tables inside can better fit the window

### If running normally

![If running normally](/run_normaly.png "running normally")

### Added a log viewing function

After the software is running, you can view the log:

Open the terminal in the log directory and execute:

``
$ tail -f requests.log
``

If you want to adjust the log level, you can open "log4rs.yaml" to adjust it. The default is the debug level, which means that when the software encounters an error, the log can be sent to the author in time to diagnose the cause of the error.

If you don't want to generate too many logs, you can change:

``
level: debug
``

to:

``
level: info
``
