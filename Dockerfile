FROM rust:1.67 as builder

WORKDIR /usr/src/portal

COPY . .

#for chinese network

#RUN set -x; \
#     CARGO_CONF='/root/.cargo/config'; \
#     BASHRC='/root/.bashrc' \
#     && mkdir /root/.cargo \
#     && echo 'export RUSTUP_DIST_SERVER=https://rsproxy.cn' >> $BASHRC \
#     && echo 'export RUSTUP_UPDATE_ROOT=https://rsproxy.cn/rustup' >> $BASHRC \
#     && touch $CARGO_CONF \
#     && echo '[source.crates-io]' > $CARGO_CONF \
#     && echo "replace-with = 'rsproxy'" >> $CARGO_CONF \
#     && echo '[source.rsproxy]' >> $CARGO_CONF \
#     && echo 'registry = "https://rsproxy.cn/crates.io-index"' >> $CARGO_CONF \
#     && echo '[net]' >> $CARGO_CONF \
#     && echo 'git-fetch-with-cli = true' >> $CARGO_CONF

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


EXPOSE 8008/tcp

EXPOSE 8080/tcp

ENTRYPOINT sh -c "service nginx start && portal-server"