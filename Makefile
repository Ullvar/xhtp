.DEFAULT_GOAL := none

upgrade:
	aws s3 rm s3://xhtp-release/xhtp && aws s3 cp target/release/xhtp s3://xhtp-release/xhtp --acl public-read && aws s3 rm s3://xhtp-release/install-xhtp.sh && aws s3 cp install-xhtp.sh s3://xhtp-release/install-xhtp.sh --acl public-read

release:
	cargo build --release

none:
	@echo "Please specify a target to make."


