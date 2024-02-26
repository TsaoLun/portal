# Dioxus + GraphQL + Rust-Actix/Go-Fiber 的粘贴板网站 Portal

本项目实现了一个可以快速启动并部署的 **WASM** 粘贴板网站，通过浏览器在多个平台实现文本内容的复制粘贴功能（主要解决我个人使用 Ubuntu 遇到的各种不方便）。

项目使用 [Dioxus](https://dioxuslabs.com/) 作为 UI 框架、[Trunk](https://trunkrs.dev/) 作为前端构建工具，并有多个语言及框架的后端版本（比如 **Rust** 的 [Actix](https://actix.rs/) 及 **Go** 的 [Fiber](https://gofiber.io/)）。本项目选择使用 GraphQL 作为接口协议，并推荐使用 Docker 运行本项目，其中包含一个 Nginx 服务与后端服务（总共所需资源很小，在 Actix 后端环境中的运行内存约 5-8MB）

![stats](docs/stats.png)

**Docker 快速搭建方式**

1. 设置 JWT_KEY 和用户密码：创建 .env 文件，内容为： `PORTAL_JWT_KEY={任意JWT_KEY} PORTAL_USERNAME={你的用户名} PORTAL_PASSWORD={你的密码}`。

2. 项目目录下执行 `docker build -t portal -f {对应版本 Dockerfile，如 'Dockerfile.actix'} .`。

3. .env 目录执行 `docker run -d --rm --env-file .env -p 8080:8080 --name portal portal`

4. 多设备访问服务器 8080 端口，进行数据快速复制。

**推荐在项目目录使用 Makefile 命令**

1. 设置 JWT_KEY 和用户密码：创建 .env 文件，内容为： `PORTAL_JWT_KEY={任意JWT_KEY} PORTAL_USERNAME={你的用户名} PORTAL_PASSWORD={你的密码}`。

2. make actix / make fiber 构建对应版本的 Docker 镜像。

3. make run 运行 Docker 镜像 / make stop 停止 Docker 镜像 / make logs 查看 Docker 镜像日志。

**开发环境**

1. 设置 JWT_KEY 和用户密码：项目目录下创建 .env 文件，内容为： `PORTAL_JWT_KEY={任意JWT_KEY} PORTAL_USERNAME={你的用户名} PORTAL_PASSWORD={你的密码}`。

2. 安装 Cargo 环境和 WASM 工具 Trunk (`cargo install trunk --version 0.16.0`)，执行 `rustup target add wasm32-unknown-unknown`。

3. 安装 npm 环境，执行 `npm i tailwindcss -g`。

4. 在对应的 server 目录下执行运行指令，actix-server 为 `cargo run .`。

5. 在 client 目录下执行 `trunk serve` 运行前端部分。

> 若 trunk serve 运行失败，且不需要修改前端代码，可以直接运行 make dev 单独运行前端部分。

![input](docs/input.png)

![copy](docs/copy.png)

**库与工具**

1. [**Rover**](https://github.com/apollographql/rover) - Rust 编写的 GraphQL CLI 工具，用于获取、整合 GraphQL Schema。

2. [**Async-GraphQL**](https://github.com/async-graphql/async-graphql) - Rust 编写的 GraphQL 库 (Code-First)，用于实现后端 GraphQL 服务。

3. [**GQLGEN**](https://github.com/99designs/gqlgen) - Go 编写的 GraphQL 代码生成工具 (Schema-First)，用于生成 Go 语言的 GraphQL 服务。

4. [**Trunk**](https://trunkrs.dev/) - Rust 编写的 WASM 构建工具，用于构建前端项目。

5. [**Dioxus**](https://dioxuslabs.com/) - Rust 编写的 UI 框架，用于构建前端项目。

6. [**Fiber**](https://gofiber.io/) - Go 编写的 Web 框架，用于构建后端项目。

7. [**Actix**](https://actix.rs/) - Rust 编写的 Web 框架，用于构建后端项目。
