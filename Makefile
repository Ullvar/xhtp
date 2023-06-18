.DEFAULT_GOAL := none

release:
	cargo build --release && aws s3 rm s3://xhtp-release/xhtp && aws s3 cp target/release/xhtp s3://xhtp-release/xhtp --acl public-read && aws s3 rm s3://xhtp-release/install-xhtp.sh && aws s3 cp install-xhtp.sh s3://xhtp-release/install-xhtp.sh --acl public-read && aws s3 cp upgrade-xhtp.sh s3://xhtp-release/upgrade-xhtp.sh --acl public-read


none:
	@echo "Please specify a target to make."


