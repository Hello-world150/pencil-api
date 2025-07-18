# Hitokoto API 使用说明

## 端点

### 1. 获取随机item
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
  "creator": "跳舞的果果",
  "creator_uid": 0,
  "created_at": "1468605909",
  "length": 22
}
```

### 2. 提交新item
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
  "creator": "提交者名称"
}
```

**成功响应**:
```json
{
  "message": "提交成功",
  "item": {
    "uuid": "UUID",
    "hitokoto": "你的一言内容",
    "type": "a",
    "from": "来源作品",
    "from_who": "作者",
    "creator": "提交者名称",
    "creator_uid": 0,
    "created_at": "1721234567",
    "length": 7
  }
}
```

**错误响应**:
```json
{
  "error": "错误信息"
}
```

## 使用示例

### 启动服务器
```bash
cargo run
```

### 获取随机item
```bash
curl http://localhost:8000/get
```

### 提交新item
```bash
curl -X POST http://localhost:8000/submit \
  -H "Content-Type: application/json" \
  -d '{
    "hitokoto": "生活就像一盒巧克力，你永远不知道下一颗是什么味道。",
    "type": "a",
    "from": "阿甘正传",
    "from_who": "阿甘",
    "creator": "用户名"
  }'
```
