FROM rust:1.67 as builder

WORKDIR /usr/src/portal

COPY . .

RUN sed -i 's/deb.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list

RUN sed -i 's/security.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list

RUN apt-get update && apt-get install -y nodejs npm && npm i -g n && n stable && npm i tailwindcss -g

RUN echo '[source.crates-io]' > ~/.cargo/config \
 && echo 'registry = "https://github.com/rust-lang/crates.io-index"'  >> ~/.cargo/config \
 && echo '# 替换成你偏好的镜像源'  >> ~/.cargo/config \
 && echo "replace-with = 'sjtu'"  >> ~/.cargo/config \
 && echo '# 上海交通大学'   >> ~/.cargo/config \
 && echo '[source.sjtu]'   >> ~/.cargo/config \
 && echo 'registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"'  >> ~/.cargo/config \
 && echo '' >> ~/.cargo/config
 
RUN cd /usr/src/portal/server && cargo install --path .

RUN cd /usr/src/portal/client && rustup target add wasm32-unknown-unknown && cargo install trunk  && trunk build --release

FROM nginx

WORKDIR /usr/src/client

RUN apt-get update && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/portal-server /usr/local/bin/portal-server

COPY --from=builder /usr/src/portal/client/dist /usr/share/nginx/html/

RUN echo "Asia/Shanghai" > /etc/timezone

RUN ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime

COPY client/nginx.conf /etc/nginx/nginx.conf

EXPOSE 8080/tcp

ENTRYPOINT sh -c "service nginx start && portal-server"