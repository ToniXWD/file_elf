
---

# file_elf 常用文件查询工具

`file_elf` 是一款轻量级的文件查询工具，旨在帮助用户快速查找本地计算机上的文件。它支持精确匹配、模糊匹配、正则查找的搜索方式，并提供了复制路径、打开文件/目录的功能。`file_elf`只会对监控到修改的文件进行索引的建立（这是判断常用文件的依据， 也是设计思路），因此不会进行全盘扫描而占用过多的系统资源。

![example](doc/example.png)

配置文件描述:
```toml
[database]
dbtype = "sqlite" # 使用的数据库类型(目前仅支持sqlite3)
path = "sqlite3.db" # 数据库文件存储位置
targets = ["D:/", "E:/", "F:/", "G:/", "H:/"] # 监听目标, 可以是具体的文件夹, 而不是单一盘符
```
# 功能

> 最新的功能可能还没有发布到[Release](https://github.com/ToniXWD/file_elf/releases), 需要本地编译

- [x] 快速搜索本地文件, 使用`Trie`树索引文件
- [x] 支持配置文件添加路径白名单
- [x] 支持配置文件添加路径黑名单(正则表达式)
- [x] 支持模糊搜索和正则匹配
- [x] 支持热点文件夹查询(`Smart Search`)
- [x] 支持添加or删除搜索到的记录项
- [x] 配置文件黑名单和过滤规则
- [ ] 配置文件支持自定义数据库(目前使用`sqlite3`)
- [ ] 前端后端使用更高效的`IPC`通信, 而不占用本地3000端口
- [ ] 支持剥离关系型数据库`sqlite`3存储, 创建自定义的文件格式存储
- [x] 重启时检查数据库记录是否有效

# 安装和开发

## 1 直接下载编译文件
从[Release](https://github.com/ToniXWD/file_elf/releases)中下载, 但**需要本地安装sqlite3数据库**才能运行

## 2 本地编译 

### 环境依赖
- 后端使用[`Rust`](https://www.rust-lang.org/learn/get-started)开发, 要求版本1.81及以上
- 客户端使用[`tauri`](https://tauri.app/)和[`react`](https://react.dev/)开发
- 数据库使用`sqlite3`存储

### Windows
```powershell
.\build.ps1 -Build # 编译客户端和服务端
.\build.ps1 -Publish # 打包编译产物和配置文件
```
在`publish`中可以看到`search-files-app.exe`文件, 双击运行即可

### Linux
```bash
make all # 编译客户端和服务端
make publish # 打包编译产物和配置文件
```
在`publish`中可以看到`search-files-app`文件, 双击运行即可
