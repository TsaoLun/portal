FROM rust:1.67

WORKDIR /usr/src/portal

COPY . .

RUN cd /usr/src/portal/server && cargo install --path .

EXPOSE 8000/tcp

CMD ["portal-server"]