CARGO_TARGET_DIR ?= $(PWD)/target
OUT := $(CARGO_TARGET_DIR)/generated

$(OUT):
	mkdir $(OUT)

generate: $(OUT)
	rm -rf $(OUT) && cd generator && openapi-generator-cli generate -i api-spec-v2.yaml -o $(OUT) -g rust -c config.yaml  --skip-validate-spec

update:
	wget -O generator/api-spec-v2.yaml https://raw.githubusercontent.com/fireblocks/fireblocks-openapi-spec/refs/heads/main/api-spec-v2.yaml
