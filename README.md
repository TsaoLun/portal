# 简单的 Dioxus 粘贴板网站 Portal

* Docker 快速搭建方式

0. 设置JWT_KEY，用户密码：项目目录下创建 .env 文件，PORTAL_JWT_KEY={任意JWT_KEY} PORTAL_USERNAME={你的用户名} PORTAL_PASSWORD={你的密码}。

1. 项目目录下执行 `docker build -t portal .`。

2. 执行 `docker run -d --restart=always --env-file .env -p 8008:8008 -p 8080:8080 --name portal portal`

3. 访问 8008 端口