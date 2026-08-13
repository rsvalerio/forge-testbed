# Minimal .deb producer, matching the layout forge's publish-deb workflow defaults to:
# `make build` in this directory, and `make deb-path` printing the artifact's path
# relative to the repository root.
#
# dpkg-deb is preinstalled on GitHub's ubuntu runners. This deliberately does not use
# cargo-deb — the testbed is proving the *workflow*, and adding a tool install would make
# a forge regression and a cargo-deb regression look identical in the logs.

PKG     := testbed-cli
VERSION := $(shell sed -n 's/^version = "\(.*\)"/\1/p' ../Cargo.toml | head -n1)
ARCH    := amd64
STAGE   := $(CURDIR)/stage
DEB     := $(CURDIR)/$(PKG)_$(VERSION)_$(ARCH).deb

.PHONY: build clean deb-path

build:
	cargo build --release --manifest-path ../Cargo.toml --bin $(PKG)
	rm -rf "$(STAGE)"
	mkdir -p "$(STAGE)/DEBIAN" "$(STAGE)/usr/bin"
	install -m 0755 ../target/release/$(PKG) "$(STAGE)/usr/bin/$(PKG)"
	printf 'Package: %s\nVersion: %s\nSection: utils\nPriority: optional\nArchitecture: %s\nMaintainer: Rodrigo Valerio <rsvalerio@gmail.com>\nDescription: forge testbed binary\n' \
	  "$(PKG)" "$(VERSION)" "$(ARCH)" >"$(STAGE)/DEBIAN/control"
	dpkg-deb --build --root-owner-group "$(STAGE)" "$(DEB)"

# Relative to the repository root, which is what the workflow's default command expects.
deb-path:
	@echo "packaging/$(PKG)_$(VERSION)_$(ARCH).deb"

clean:
	rm -rf "$(STAGE)" "$(CURDIR)"/*.deb
