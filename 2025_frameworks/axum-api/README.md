# Rust HTTP API

基于 Axum 框架构建的 RESTful API 服务，提供完整的用户管理功能。

## 功能特性

- ✅ 完整的 CRUD 操作（创建、读取、更新、删除用户）
- ✅ UUID 作为用户唯一标识符
- ✅ 内存数据存储（线程安全）
- ✅ JSON 请求/响应格式
- ✅ 健康检查端点
- ✅ 错误状态码处理

## 启动方式

```bash
cargo run
```

## API 端点

- `GET /` - 根路径
- `GET /health` - 健康检查
- `GET /users` - 获取所有用户
- `GET /users/{id}` - 获取指定用户
- `POST /users` - 创建新用户
- `PUT /users/{id}` - 更新用户信息
- `DELETE /users/{id}` - 删除用户

## 请求示例

创建用户：
```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"张三","email":"zhangsan@example.com","age":25}'
```
