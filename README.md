# 简单的 Dioxus 粘贴板网站 Portal

**Docker 快速搭建方式**

1. 设置 JWT_KEY 和用户密码：项目目录下创建 .env 文件，内容为： `PORTAL_JWT_KEY={任意JWT_KEY} PORTAL_USERNAME={你的用户名} PORTAL_PASSWORD={你的密码}`。

2. 项目目录下执行 `docker build -t portal .`。

3. 执行 `docker run -d --restart=always --env-file .env -p 8008:8008 -p 8080:8080 --name portal portal`

4. 多设备访问服务器 8080 端口，进行数据快速复制。

**开发环境**

1. 设置 JWT_KEY 和用户密码：项目目录下创建 .env 文件，内容为： `PORTAL_JWT_KEY={任意JWT_KEY} PORTAL_USERNAME={你的用户名} PORTAL_PASSWORD={你的密码}`。

2. 安装 Rust 环境和 WASM 工具 Trunk (cargo install trunk)，任意路径下执行 `rustup target add wasm32-unknown-unknown`。

3. 在 client 目录下执行 `trunk serve` 运行前端部分。

4. 在 server 目录下执行 `cargo run .` 运行后端服务。