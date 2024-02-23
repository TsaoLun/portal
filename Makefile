IMAGE_NAME := portal
POD_NAME := portal
t ?= 100

actix:
	docker build -t portal -f Dockerfile.actix .

fiber:
	docker build -t portal -f Dockerfile.fiber .

dev:
	docker build -t portal -f Dockerfile.dev .

run:
	@if [ ! -f .env ]; then echo ".env file does not exist: 请在项目目录下创建 .env 文件"; exit 1; fi
	@if [ ! -s .env ]; then echo ".env file is empty: 请设置 PORTAL_JWT_KEY, PORTAL_USERNAME, PORTAL_PASSWORD"; exit 1; fi
	docker run -d -p 8080:8080 --rm --env-file .env --name $(POD_NAME) $(IMAGE_NAME)

logs:
	docker logs -f $(POD_NAME) --tail $(t)

stop:
	docker stop $(POD_NAME)