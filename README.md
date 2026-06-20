# filelock

XOR 加密/解密文件的 CLI 工具，支持基于偏移量的密钥生成与文件保护。

## 安装

从 [Releases](https://github.com/J-SLY/filelock/releases) 下载预编译二进制，或自行编译：

```bash
cargo build --release
./target/release/filelock
```

## 使用

### 加密

```bash
filelock lock -f <文件路径> -o <偏移量>
```

根据偏移量计算密钥，对整个文件进行 XOR 加密，并将密钥追加到文件末尾。

### 解密

```bash
filelock unlock -f <文件路径>
```

读取文件末尾的密钥字节，对剩余内容进行 XOR 解密。

## 算法

- 密钥 = `(offset % file_len) % 256`，保证密钥在 1..=255 范围内（零偏移量仍返回 0）
- 加密时每个字节与密钥异或，密钥和魔数标记 `FL` 追加到文件末尾
- 解密时校验末尾标记 `FL`，弹出密钥字节再次异或（对称加密）
- 文件格式：`[加密数据][1字节密钥][2字节标记 FL]`
