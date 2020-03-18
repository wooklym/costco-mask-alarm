env ?= .env
include $(env)

build:
	@docker build -t $(APP_NAME) .

build-nc:
	@docker build --no-cache -t $(APP_NAME) .

run: stop
	@docker run -t --env-file=.env -d --name="$(APP_NAME)" $(APP_NAME)

log:
	@docker logs -f $(APP_NAME)

stop:
	@docker stop $(APP_NAME)
	@docker rm $(APP_NAME)

up: build run
