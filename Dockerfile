FROM rust:1.67 as builder

WORKDIR /usr/src/portal

COPY . .

RUN sed -i 's/deb.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list

RUN sed -i 's/security.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list

RUN curl -sL https://deb.nodesource.com/setup_18.x | -E bash -

RUN apt-get update && apt-get install -y nodejs npm && npm i tailwindcss -g

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