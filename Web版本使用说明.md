# Gooey Web 版本使用说明

## 概述

Gooey 现在支持 Web 版本！您可以通过浏览器访问和使用 Gooey 的所有核心功能，无需安装桌面应用程序。

## 功能特性

### ✅ 完全支持的功能
- **项目管理**: 浏览、创建和管理 Claude Code 项目
- **代理管理**: 创建、编辑、执行和管理 AI 代理
- **使用统计**: 查看 API 使用情况和成本统计
- **MCP 服务器**: 管理 Model Context Protocol 服务器
- **实时通信**: WebSocket 支持实时数据更新

### ⚠️ 部分支持的功能
- **文件系统操作**: 通过 API 访问，需要服务器端配置
- **系统命令执行**: 通过服务器端执行
- **会话管理**: 基础功能支持，高级功能待完善

### ❌ 暂不支持的功能
- **直接文件编辑**: 需要通过 API 操作
- **本地数据库**: 使用服务器端数据库
- **系统级集成**: 如系统托盘、全局快捷键等

## 启动方式

### 方法一：开发模式（推荐）

1. **启动后端服务器**
   ```bash
   # 在项目根目录执行
   bun run serve:web
   ```
   
   这将启动 Rust 后端服务器，监听端口 3000。

2. **启动前端开发服务器**
   ```bash
   # 在新终端窗口执行
   bun run dev:web
   ```
   
   这将启动 Vite 开发服务器，监听端口 1420。

3. **访问 Web 应用**
   ```
   打开浏览器访问: http://localhost:1420
   ```

### 方法二：生产模式

1. **构建前端**
   ```bash
   bun run build:web
   ```

2. **启动服务器**
   ```bash
   bun run serve:web
   ```

3. **访问应用**
   ```
   打开浏览器访问: http://localhost:3000
   ```

## 使用指南

### 1. 项目管理

#### 浏览项目
- 在主页点击 "Projects" 卡片
- 查看所有 Claude Code 项目列表
- 点击项目名称查看详细信息

#### 创建项目
- 点击 "Open Project" 按钮
- 选择项目目录（需要服务器端配置）
- 系统将自动创建项目记录

#### 查看会话
- 在项目列表中点击项目
- 查看该项目的所有会话历史
- 点击会话查看详细信息

### 2. 代理管理

#### 创建代理
- 在主页点击 "CC Agents" 卡片
- 点击 "Create Agent" 按钮
- 填写代理信息：
  - **名称**: 代理的显示名称
  - **图标**: 选择代理图标
  - **系统提示**: 定义代理行为
  - **默认任务**: 可选的默认任务描述
  - **模型**: 选择使用的 Claude 模型

#### 执行代理
- 在代理列表中找到目标代理
- 点击 "Execute" 按钮
- 选择项目路径和任务描述
- 点击 "Run" 开始执行

#### 管理代理
- **编辑**: 点击代理名称进入编辑模式
- **删除**: 点击删除按钮移除代理
- **导出**: 导出代理配置为 JSON 文件
- **导入**: 从 JSON 文件导入代理配置

### 3. 使用统计

#### 查看统计
- 在顶部导航栏点击 "Usage" 按钮
- 查看总体使用统计：
  - 总成本
  - 总 Token 数
  - 会话数量
  - 按模型分组统计

#### 时间范围筛选
- 选择开始和结束日期
- 查看指定时间范围内的使用情况
- 导出统计数据

#### 项目分析
- 查看各项目的使用情况
- 分析成本分布
- 优化使用策略

### 4. MCP 服务器管理

#### 添加服务器
- 在顶部导航栏点击 "MCP" 按钮
- 点击 "Add Server" 按钮
- 配置服务器信息：
  - **名称**: 服务器标识
  - **传输方式**: stdio 或 sse
  - **命令**: 启动命令
  - **参数**: 命令参数
  - **环境变量**: 环境配置

#### 测试连接
- 在服务器列表中点击 "Test" 按钮
- 验证服务器连接状态
- 查看连接结果

#### 管理服务器
- **编辑**: 修改服务器配置
- **删除**: 移除服务器
- **导入**: 从 Claude Desktop 导入配置

## 配置说明

### 环境变量

创建 `env.web` 文件（已包含在项目中）：
```env
VITE_API_BASE_URL=http://localhost:3000
VITE_WS_URL=ws://localhost:3000
VITE_APP_MODE=web
VITE_APP_NAME=Gooey Web
VITE_APP_VERSION=0.1.0
```

### 服务器配置

Web 版本需要访问 `~/.claude` 目录，确保：
1. 服务器有读取权限
2. Claude Code CLI 已正确安装
3. 项目目录结构正确

## 故障排除

### 常见问题

#### 1. 无法连接到服务器
**症状**: 页面显示连接错误
**解决方案**:
- 检查后端服务器是否启动：`bun run serve:web`
- 确认端口 3000 未被占用
- 检查防火墙设置

#### 2. 项目列表为空
**症状**: 项目管理页面显示无项目
**解决方案**:
- 确认 `~/.claude/projects` 目录存在
- 检查目录权限
- 确认有 Claude Code 项目

#### 3. 代理执行失败
**症状**: 代理执行时出现错误
**解决方案**:
- 检查 Claude Code CLI 是否正确安装
- 确认项目路径有效
- 查看服务器日志

#### 4. WebSocket 连接失败
**症状**: 实时更新不工作
**解决方案**:
- 检查 WebSocket 连接：`ws://localhost:3000/ws`
- 确认代理设置正确
- 重启服务器

### 日志查看

#### 后端日志
```bash
# 启动时查看详细日志
RUST_LOG=debug bun run serve:web
```

#### 前端日志
- 打开浏览器开发者工具
- 查看 Console 标签页
- 检查 Network 标签页的 API 请求

## 开发说明

### 架构概述

```
浏览器 → Vite 开发服务器 → Rust 后端服务器 → Claude Code CLI
```

### 技术栈
- **前端**: React + TypeScript + Vite
- **后端**: Rust + Axum + Tauri
- **通信**: HTTP API + WebSocket
- **状态管理**: Zustand

### 开发命令

```bash
# 开发模式
bun run dev:web          # 启动前端开发服务器
bun run serve:web        # 启动后端服务器

# 构建
bun run build:web        # 构建前端
bun run serve:web        # 启动生产服务器

# 检查
bun run check            # TypeScript 类型检查
```

### 文件结构

```
src/
├── lib/
│   └── api/
│       ├── web-adapter.ts    # Web API 适配器
│       └── api.ts            # 主 API 文件（已修改）
├── components/               # React 组件（复用现有）
└── ...

src-tauri/src/
├── web_server.rs             # Web 服务器主文件
├── web_routes.rs             # API 路由定义
├── web_websocket.rs          # WebSocket 处理
└── main.rs                   # 主文件（已修改）
```

## 部署说明

### 本地部署

1. **构建应用**
   ```bash
   bun run build:web
   ```

2. **启动服务器**
   ```bash
   bun run serve:web
   ```

3. **访问应用**
   ```
   http://localhost:3000
   ```

### 服务器部署

1. **准备服务器**
   - 安装 Rust 和 Bun
   - 配置 Claude Code CLI
   - 设置文件权限

2. **部署代码**
   ```bash
   git clone <repository>
   cd gooey
   bun install
   bun run build:web
   ```

3. **启动服务**
   ```bash
   bun run serve:web
   ```

4. **配置反向代理**（可选）
   - 使用 Nginx 或 Apache
   - 配置 SSL 证书
   - 设置域名

## 更新日志

### v0.1.0 (当前版本)
- ✅ 基础 Web 版本支持
- ✅ 项目管理功能
- ✅ 代理管理功能
- ✅ 使用统计功能
- ✅ MCP 服务器管理
- ✅ WebSocket 实时通信
- ✅ 响应式设计

### 计划功能
- 🔄 文件编辑器集成
- 🔄 会话时间线可视化
- 🔄 检查点管理
- 🔄 批量操作支持
- 🔄 用户认证系统

## 支持与反馈

如果您在使用 Web 版本时遇到问题或有建议，请：

1. 查看本文档的故障排除部分
2. 检查项目的 GitHub Issues
3. 提交新的 Issue 或 Pull Request

---

**注意**: Web 版本是桌面版本的补充，某些高级功能可能需要在桌面版本中使用。建议根据具体需求选择合适的版本。
