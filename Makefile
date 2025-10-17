CARGO_TARGET_DIR ?= $(PWD)/target
OUT := $(CARGO_TARGET_DIR)/generated

$(OUT):
	mkdir $(OUT)

generate: $(OUT)
	rm -rf $(OUT) && cd generator && openapi-generator-cli generate -i api-spec-v2.yaml -o $(OUT) -g rust -c config.yaml  --skip-validate-spec
	rm -v -f $(OUT)/src/models/status.rs $(OUT)/src/apis/cosigners_beta_api.rs
	rsync -a --delete $(OUT)/src/models/ src/models/
	rsync -a --delete $(OUT)/src/apis/ src/apis/
	rsync -a --delete $(OUT)/docs/ docs/
  sed -i 's/models::serde_json::/serde_json::/g' src/apis/*.rs
	cargo +nightly fmt --all

update:
	wget -O generator/api-spec-v2.yaml https://raw.githubusercontent.com/fireblocks/fireblocks-openapi-spec/refs/heads/main/api-spec-v2.yaml
