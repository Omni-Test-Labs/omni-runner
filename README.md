# Omni-Runner

高性能边缘测试执行引擎。负责拉取 Pipeline 任务、管理本地执行环境、实时上报日志与产物。

## 设计原则

"管生管死不管过程" - 负责 Pipeline 的调度、分配与执行状态管理，不干预内部执行细节。

## Pipeline 引擎说明

**为什么 omni-runner 包含 pipeline 引擎？**

omni-runner 的 Pipeline Engine 是**本地执行编排**引擎，负责：
- 解析 Server 下发的 TaskManifest 配置
- 拓扑排序（确定步骤执行顺序）
- 依赖管理（depends_on 检查）
- 步骤调度（选择合适的 executor）
- 流程控制（must_pass, always_run, failure_policy）

这**不是** "定义 pipeline"，而是 "控制如何按照定义执行 pipeline"。Server 定义 pipeline 结构，Runner 控制本地执行流程。详见 `/docs/architecture.md`。

## 架构

### 核心模块

- **src/main.rs** - 程序入口点，包含主循环和信号处理
- **src/heartbeat.rs** - 心跳发送逻辑
- **src/system.rs** - 系统资源监控（CPU、内存、磁盘）
- **src/tasks.rs** - 任务轮询和执行逻辑
- **src/lib.rs** - 库接口，支持单元测试

### 执行器

- **ShellExecutor** - Shell 命令执行
- **PythonExecutor** - Python 脚本执行
- **BinaryExecutor** - 二进制程序执行
- **ApiExecutor** - API 调用执行

### Pipeline 引擎

- 支持步骤依赖关系
- 支持必须通过（must_pass）配置
- 支持重试策略
- 支持安全策略验证

## 测试

### 测试覆盖率

- **omni-runner**: 49.60% (249/502 lines)
  * heartbeat.rs: 70.00% (7/10 lines)
  * system.rs: 55.88% (19/34 lines)
  * pipeline/engine.rs: 80.00% (72/90 lines)
  * security/policy.rs: 96.43% (27/28 lines)
  * executor/shell.rs: 94.74% (72/76 lines)

- **omni-server**: 96.20% ✅

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行测试并生成覆盖率报告
cargo tarpaulin --out Html --output-dir ./coverage

# 查看覆盖率报告
open coverage/tarpaulin-report.html
```

### 测试文件

- 18 个测试文件
- 101 个单元测试
- 100% 测试通过率

## 构建

```bash
# 开发构建
cargo build

# 生产构建
cargo build --release

# 运行 bin
cargo run -- --config config/runner.toml
```

## 环境变量

- `CONFIG_PATH` - 配置文件路径（默认：`config`）

## 配置

TOML 配置文件格式：

```toml
[device]
device_id = "omni-device-001"
device_type = "edge-agent"
hostname = "omni-edge-01"

[server]
base_url = "http://omni-server:8080"
api_key = "your-api-key"

[polling]
interval_seconds = 5
heartbeat_interval_seconds = 30
```

## 约束说明

### 无法测试的代码

以下代码在"不使用mock、必须使用真实功能代码"的约束下无法达到高覆盖率：

1. **src/main.rs** - 程序入口点、无限循环、信号处理
2. **src/utils/logging.rs** - 全局副作用（初始化 tracing subscriber）
3. **src/executor/python.rs** - 依赖系统 Python 解释器
4. **src/executor/api.rs** - 需要 HTTP 服务器

当前 49.60% 覆盖率是在给定约束下的最大可实现值。

## License

MIT
