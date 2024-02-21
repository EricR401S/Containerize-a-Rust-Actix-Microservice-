install:
	cargo clean &&\
		cargo build -j 1

build:
	docker build -t number_generator .

rundocker:
	# docker run -it --rm -p 3030:3030 number_generator
	#docker run -it --rm -p 8080:8080 number_generator
	docker run -dp 3000:3000 number_generator


format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet


deploy-aws:
	cargo build
	aws ecr get-login-password --region us-east-2 | docker login --username AWS --password-stdin 667719398048.dkr.ecr.us-east-2.amazonaws.com
	docker build -t lotto_axum .
	docker tag lotto_axum:latest 667719398048.dkr.ecr.us-east-2.amazonaws.com/lotto_axum:latest
	docker push 667719398048.dkr.ecr.us-east-2.amazonaws.com/lotto_axum:latest

run:
	cargo run 

all: format lint test run