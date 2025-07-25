# Pencil API 使用文档

这是一个基于 Rust Rocket 框架的 Hitokoto（一言）API 服务，支持用户注册、Hitokoto 提交、文集管理等功能。

## API 端点

### 1. 获取随机 Hitokoto
**GET** `/hitokoto/get`

返回一个随机的 Hitokoto。

**响应示例:**
```json
{
  "uuid": "f4a5f102-bc88-478a-a9af-4c53ab78264a",
  "hitokoto": "测试用户UUID引用功能",
  "type": "a",
  "from": "测试",
  "from_who": "测试者",
  "user": "新测试用户",
  "user_id": 3261390917,
  "created_at": 1753354989,
  "length": 12
}
```

### 2. 用户注册
**POST** `/user/register`

注册新用户。

**请求体:**
```json
{
  "username": "用户名"
}
```

**响应示例:**
```json
{
  "user_id": 3261390917,
  "username": "新测试用户",
  "items": [],
  "collections": []
}
```

### 3. 提交 Hitokoto
**POST** `/hitokoto/submit`

提交新的 Hitokoto（需要已注册的用户）。

**请求体:**
```json
{
  "hitokoto": "一言内容",
  "type": "a",
  "from": "来源",
  "from_who": "作者",
  "user_id": 3261390917
}
```

**响应示例:**
```json
{
  "uuid": "generated-uuid",
  "hitokoto": "一言内容",
  "type": "a",
  "from": "来源",
  "from_who": "作者",
  "user": "用户名",
  "user_id": 3261390917,
  "created_at": 1753354989,
  "length": 4
}
```

### 4. 获取用户详情
**GET** `/user/<user_id>`

获取用户的完整信息，包括所有提交的 Hitokoto 和创建的文集。采用递归结构，返回完整的层级数据。

**响应示例:**
```json
{
  "user_id": 3261390917,
  "username": "新测试用户",
  "items": [
    {
      "uuid": "f4a5f102-bc88-478a-a9af-4c53ab78264a",
      "hitokoto": "测试用户UUID引用功能",
      "type": "a",
      "from": "测试",
      "from_who": "测试者",
      "user": "新测试用户",
      "user_id": 3261390917,
      "created_at": 1753354989,
      "length": 12
    }
  ],
  "collections": [
    {
      "collection_uuid": "fb329110-3b42-410c-bd2d-e4256df53d01",
      "title": "我的第一个文集",
      "description": "这是一个测试文集",
      "user_id": 3261390917,
      "hitokoto_items": [
        {
          "uuid": "f4a5f102-bc88-478a-a9af-4c53ab78264a",
          "hitokoto": "测试用户UUID引用功能",
          "type": "a",
          "from": "测试",
          "from_who": "测试者",
          "user": "新测试用户",
          "user_id": 3261390917,
          "created_at": 1753354989,
          "length": 12
        }
      ],
      "created_at": 1753356305
    }
  ]
}
```

### 5. 创建文集
**POST** `/collection/create`

为已注册用户创建新的文集。

**请求体:**
```json
{
  "user_id": 3261390917,
  "title": "文集标题",
  "description": "文集描述（可选）"
}
```

**响应示例:**
```json
{
  "collection_uuid": "fb329110-3b42-410c-bd2d-e4256df53d01",
  "title": "我的第一个文集",
  "description": "这是一个测试文集",
  "user_id": 3261390917,
  "hitokoto_ids": [],
  "created_at": 1753356305
}
```

### 6. 向文集添加 Hitokoto
**POST** `/collection/<collection_uuid>/add`

向指定文集添加 Hitokoto 条目。

**请求体:**
```json
{
  "hitokoto_uuid": "f4a5f102-bc88-478a-a9af-4c53ab78264a"
}
```

**响应示例:**
```json
{
  "success": true,
  "message": "添加成功"
}
```

## 数据结构说明

### 三层架构
本API采用三层数据架构：

1. **用户层 (User)** - 顶层，包含用户基本信息
2. **文集层 (Collection)** - 中间层，用户可以创建多个文集来组织 Hitokoto
3. **Hitokoto层 (Item)** - 底层，实际的一言内容

### 引用关系
- 用户通过 `items` 字段引用其提交的所有 Hitokoto UUID
- 用户通过 `collections` 字段引用其创建的所有文集 ID
- 文集通过 `hitokoto_ids` 字段引用包含的 Hitokoto UUID
- 所有引用关系通过 UUID/ID 维护，保证数据一致性

### 递归数据检索
当调用 `/user/<user_id>` API 时，系统会递归检索：
1. 用户基本信息
2. 用户提交的所有 Hitokoto 完整内容
3. 用户创建的所有文集信息
4. 每个文集中包含的所有 Hitokoto 完整内容

## 技术特性

- **异步处理**: 所有文件 I/O 操作均采用 Tokio 异步运行时
- **并发安全**: 使用 `tokio::sync::Mutex` 保证多线程安全
- **UUID 系统**: 所有实体使用 UUID 进行唯一标识
- **数据持久化**: JSON 文件存储，支持服务重启后数据恢复
- **输入验证**: 严格的用户ID和UUID验证机制
- **错误处理**: 完善的错误处理和日志记录
- **性能优化**: 减少不必要的数据克隆，优化锁持有时间
- **代码重用**: 模块化设计，减少重复代码

## 最新优化

### 代码重构优化 (2025-07-24)

1. **代码重用优化**:
   - 提取了 `get_hitokoto_items_by_uuids` 辅助函数
   - 添加了 `get_collection_with_details` 辅助函数
   - 实现了 `get_username_by_id` 和 `hitokoto_exists` 验证函数
   - 创建了 `with_user_mut` 通用用户操作函数

2. **性能优化**:
   - 减少了不必要的 `.clone()` 调用
   - 优化了锁的持有时间
   - 改善了内存使用效率
   - 使用移动语义替代部分克隆操作

3. **架构改进**:
   - 模块化的辅助函数设计
   - 更清晰的错误处理机制
   - 统一的数据验证流程
   - 优化的异步函数调用模式

## 启动服务

```bash
cargo run
```

服务默认运行在 `http://0.0.0.0:8000`

## 数据文件

- `hitokoto.json` - 存储所有 Hitokoto 数据
- `user.json` - 存储用户信息和引用关系
- `collection.json` - 存储文集信息和引用关系
