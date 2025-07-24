# Hitokoto API 使用说明

## 📚 概述

本 API 提供了 Hitokoto（一言）的提交、获取和用户管理功能。

## 📋 API 端点

### 1. 用户注册
- **方法**: POST
- **路径**: `/register`
- **Content-Type**: application/json

**请求体**:
```json
{
  "username": "用户名"
}
```

**成功响应**:
```json
{
  "message": "用户注册成功",
  "item": {
    "user_id": 3939516485,
    "username": "用户名",
    "items": []
  }
}
```

**错误响应**:
```json
{
  "error": "用户名已存在"
}
```

### 2. 提交新 Hitokoto
- **方法**: POST
- **路径**: `/submit`
- **Content-Type**: application/json

**请求体**:
```json
{
  "hitokoto": "你的一言内容",
  "type": "a",
  "from": "来源作品",
  "from_who": "作者（可选）",
  "user_id": 3939516485
}
```

**成功响应**:
```json
{
  "message": "提交成功",
  "item": {
    "uuid": "d0c55af5-0075-414b-bd9c-364767d53c96",
    "hitokoto": "你的一言内容",
    "type": "a",
    "from": "来源作品",
    "from_who": "作者",
    "user": "用户名",
    "user_id": 3939516485,
    "created_at": 1753354122,
    "length": 7
  }
}
```

**错误响应**:
```json
{
  "error": "提交失败: 用户ID 99999 不存在，请先注册用户"
}
```

### 3. 获取随机 Hitokoto
- **方法**: GET
- **路径**: `/get`
- **返回**: 随机的一条 hitokoto 数据

**示例响应**:
```json
{
  "uuid": "9818ecda-9cbf-4f2a-9af8-8136ef39cfcd",
  "hitokoto": "与众不同的生活方式很累人呢，因为找不到借口。",
  "type": "a",
  "from": "幸运星",
  "from_who": null,
  "user": "跳舞的果果",
  "user_id": 3939516485,
  "created_at": 1468605909,
  "length": 22
}
```

## 💡 使用示例

### 启动服务器
```bash
cargo run
```

### 完整使用流程

#### 1. 注册用户
```bash
curl -X POST http://localhost:8000/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "测试用户"
  }'
```

**响应**：
```json
{
  "message": "用户注册成功",
  "item": {
    "user_id": 3939516485,
    "username": "测试用户",
    "items": []
  }
}
```

#### 2. 使用 user_id 提交 Hitokoto
```bash
curl -X POST http://localhost:8000/submit \
  -H "Content-Type: application/json" \
  -d '{
    "hitokoto": "人生就像一盒巧克力，你永远不知道下一颗是什么味道。",
    "type": "a",
    "from": "阿甘正传",
    "from_who": "阿甘",
    "user_id": 3939516485
  }'
```

#### 3. 获取随机 Hitokoto
```bash
curl http://localhost:8000/get
```

#### 4. 测试错误处理（使用不存在的 user_id）
```bash
curl -X POST http://localhost:8000/submit \
  -H "Content-Type: application/json" \
  -d '{
    "hitokoto": "这应该失败",
    "type": "a",
    "from": "测试",
    "from_who": null,
    "user_id": 99999
  }'
```

**错误响应**：
```json
{
  "error": "提交失败: 用户ID 99999 不存在，请先注册用户"
}
```
