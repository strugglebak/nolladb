# nolladb

nolladb 是一个基于 Rust 开发的 simple 关系型数据库

## 技术栈

主要是

- `sqlparser 0.13`
- `rstest 0.12`
- `rustyline 9.1.2`
- `prettytable-rs 0.8`
- `bincode 1.3.3`
- `thiserror 1.0.30`

## Features

- [x] 支持命令行接口
- [x] 支持部分 `meta` 命令和 SQL 语句的解析
- [x] 支持 `执行` 简单的命令
- [x] 使用 `thiserror` 支持标准错误处理
- [x] 支持创建 `Table`
- [x] 支持创建 `Table` 时解析重复列
- [x] 支持创建 `Table` 时解析多个 `PRIMARY KEY`
- [x] 支持简单 `INSERT` 查询命令的解析
- [x] 拥有专门为 `PRIMARY KEY` 初始化的内存型 `BTreeMap` 索引
- [x] 支持唯一 `KEY` 约束

## 安装以及调试

```bash
git clone git@github.com:strugglebak/nolladb.git
cd nolladb
cargo run test.db
```

## 测试

```bash
cd nolladb
cargo test
```

## Roadmaps

- [ ] 实现简单 `SELECT` 查询
- [ ] 实现 JOINS
  - [ ] INNER JOIN
  - [ ] LEFT OUTER JOIN
  - [ ] CROSS JOIN
- [ ] 实现预写日志
- [ ] 实现页模块
  - [ ] 实现事务 ACID
  - [ ] 并发
  - [ ] 锁管理
- [ ] 实现复合索引
- [ ] 实现连接管理
- [ ] 实现不同场景下的存储引擎
  - [ ] 实现 `LSM Tree && SSTable` 应对大量写的场景
  - [ ] 实现更快的 `B-Tree` 应对大量读的场景

## 协议

[MIT](./LICENSE)
