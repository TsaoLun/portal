FROM rust:1.67 as builder

WORKDIR /usr/src/portal

COPY . .

RUN set -x && \
    NODEJS_VERSION=v16.17.0 && \
    curl -O -L https://registry.npmmirror.com/-/binary/node/latest-v16.x/node-$NODEJS_VERSION-linux-x64.tar.gz && \
    tar zxf node-$NODEJS_VERSION-linux-x64.tar.gz && \
    rm node-$NODEJS_VERSION-linux-x64.tar.gz && \
    mv node-$NODEJS_VERSION-linux-x64/ /nodejs && \
    PATH=$PATH:/nodejs/bin && \
    npm i tailwindcss -g

RUN tailwindcss info

ENV RUSTUP_DIST_SERVER="https://rsproxy.cn"

ENV RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"

RUN cd /usr/src/portal/server && cargo --config ../cargo.config.toml install --path .

RUN cd /usr/src/portal/client && rustup target add wasm32-unknown-unknown && cargo --config ../cargo.config.toml install trunk --version=0.16.0  && trunk build --release

FROM nginx

WORKDIR /usr/src/client

COPY --from=builder /usr/local/cargo/bin/portal-server /usr/local/bin/portal-server

COPY --from=builder /usr/src/portal/client/dist /usr/share/nginx/html/

RUN echo "Asia/Shanghai" > /etc/timezone

RUN ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime

COPY client/nginx.conf /etc/nginx/nginx.conf

EXPOSE 8080/tcp

ENTRYPOINT sh -c "service nginx start && portal-server"