# 这里是 Phira (吗？)

- 此项目通过替换 `libphira.so` 文件实现对 `Android` 端 `phira` 的编译和运行

## 使用

- 运行 `.github/workflows/build.yml` 构建 `libphira.so`
- 将 `libphira.so` 文件替换到 `release` 版本中

## 原理

- 通过 `Android NDK` 编译 `libphira.so`
- 在 `lib.rs` 中添加 `Java_quad_1native_QuadNative_preprocessInput` 函数, 避免壳无法找到函数崩溃 (Tip: 也可在 `dex` 之中吧调用操作 nop 掉)
- 替换正常 `release` 版本中的 `libphira.so` 文件以实现更改
