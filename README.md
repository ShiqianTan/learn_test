# 学习rust测试

## 如何写测试

1. 通过assert!宏写测试
2. 通过assert_eq!和assert_ne!宏写测试
3. 通过panic!进行测试
4. 通过Result写测试
5. 通过[should_panic],[ignore]控制测试
6. 通过expected方法提供额外测测试信息

## 如何运行测试

```bash
# 查看测试辅助信息
cargo test --help

# 参看测试二进制的帮助信息
cargo test -- --help

# 运行所有测试，不包含[ignored]
cargo test

# 运行指定测试
cargo test test_name

# 通过测试名的一部分或者模块名
# 例如有add_pass, addd_fail两个测试
cargo test add

# 通过模块名测试
# 运行mod test2中所有的测试
cargo test test2

# 测试被忽略的测试
cargo test -- --ignored
```

## 测试的分类

- Rust对测试的分类：
  - 单元测试
  - 集成测试

- 单元测试：
  - 小，专注
  - 一次对一个模块进行隔离的测试
  - 可测试private接口

- 集成测试：
  - 在库外部。和其它外部代码一样使用你的代码
  - 只能使用pub接口
  - 可能在每个测试中使用到多个模块

- 标注
  - 单元测试：#[cfg(test)]标注测试模块
  - 基础测试在不同的目录， 不需要#[cfg(test)]标注

- 测试私有函数
  - Rust允许测试私有函数

## 集成测试

- 目的：测试被测试库的多个部分是否能正确的一起工作
- 集成测试的覆盖率很重要

### tests目录

* 创建集成测试：tests目录
* tests目录下的每个测试文件都是单独的一个crate

### 运行指定的集成测试

1. cargo test 函数名
2. 运行某个测试文件内的所有测试:

```bash
cargo test --test integration_test
```

### 针对binary crate的集成测试

- 如果项目是binary crate, 只含有src/main.rs没有src/lib.rs;
  - 不能在tests目录下创建集成测试
  - 无法把main.rs的函数导入作用域
- 只有library crate才能暴露函数给其他crate用
- binary crate意味着独立运行

参考<https://www.bilibili.com/video/BV1hp4y1k7SV?spm_id_from=333.999.0.0>
