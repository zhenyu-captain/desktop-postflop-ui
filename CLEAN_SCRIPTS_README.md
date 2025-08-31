# 清理脚本使用说明

本项目提供了三个不同平台的清理脚本，用于清理构建产物和临时文件。

## 🐧 Linux/macOS 用户

使用 bash 脚本：
```bash
# 给脚本执行权限（首次使用）
chmod +x clean.sh

# 运行清理脚本
./clean.sh
```

## 🪟 Windows 用户

### 方法1：PowerShell 脚本（推荐）
```powershell
# 在 PowerShell 中运行
.\clean.ps1

# 如果遇到执行策略限制，先运行：
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### 方法2：批处理文件
```cmd
# 在命令提示符中运行
clean.bat

# 或者直接双击 clean.bat 文件
```

## 🧹 清理内容

所有脚本都会清理以下内容：

### Rust 构建产物
- `src-tauri/target/` 目录（通过 `cargo clean`）

### 前端构建产物
- `dist/` 目录（构建输出）
- `node_modules/` 目录（依赖包）

### 临时文件
- `*.tmp`, `*.temp`, `*.log` 文件
- `Thumbs.db`, `desktop.ini`（Windows 系统文件）
- `.cache/` 目录（缓存文件）

## 🚀 清理后操作

清理完成后，重新安装依赖并构建：

```bash
# 安装依赖
npm install

# 构建项目
npm run build

# 或者开发模式
npm run dev
```

## ⚠️ 注意事项

1. **备份重要文件**：清理前确保没有重要的临时文件需要保留
2. **关闭应用**：清理前关闭正在运行的应用程序
3. **权限问题**：Windows 用户可能需要以管理员身份运行
4. **执行策略**：PowerShell 可能需要调整执行策略

## 🔧 故障排除

### PowerShell 执行策略错误
```powershell
# 查看当前策略
Get-ExecutionPolicy

# 临时允许脚本执行
Set-ExecutionPolicy -ExecutionPolicy Bypass -Scope Process

# 永久允许脚本执行（需要管理员权限）
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope LocalMachine
```

### 文件被占用
如果某些文件无法删除，请：
1. 关闭相关的应用程序
2. 重启终端/命令提示符
3. 以管理员身份运行脚本

## 📝 自定义清理

如果需要添加其他清理规则，可以编辑相应的脚本文件：

- `clean.sh` - Linux/macOS 版本
- `clean.ps1` - Windows PowerShell 版本  
- `clean.bat` - Windows 批处理版本
