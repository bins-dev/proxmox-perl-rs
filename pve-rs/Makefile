include /usr/share/dpkg/pkg-info.mk

PACKAGE=libpve-rs-perl
export PERLMOD_PRODUCT=PVE

ARCH:=$(shell dpkg-architecture -qDEB_BUILD_ARCH)

PERL_INSTALLVENDORARCH != perl -MConfig -e 'print $$Config{installvendorarch};'
PERL_INSTALLVENDORLIB != perl -MConfig -e 'print $$Config{installvendorlib};'

MAIN_DEB=$(PACKAGE)_$(DEB_VERSION)_$(ARCH).deb
DBGSYM_DEB=$(PACKAGE)-dbgsym_$(DEB_VERSION)_$(ARCH).deb
DEBS=$(MAIN_DEB) $(DBGSYM_DEB)
DSC=$(PACKAGE)_$(DEB_VERSION_UPSTREAM_REVISION).dsc
BUILDDIR ?= $(PACKAGE)-$(DEB_VERSION_UPSTREAM)

DESTDIR=

PM_DIR := PVE

PERLMOD_GENPACKAGE := /usr/lib/perlmod/genpackage.pl \
	--lib=pve_rs \
	--lib-tag=proxmox \
	--lib-package=Proxmox::Lib::PVE \
	--lib-prefix=PVE \
	--include-file=Fixup.pm

PERLMOD_PACKAGES := \
	  PVE::RS::Firewall::SDN \
	  PVE::RS::OpenId \
	  PVE::RS::ResourceScheduling::Static \
	  PVE::RS::SDN::Fabrics \
	  PVE::RS::TFA

PERLMOD_PACKAGE_FILES := $(addsuffix .pm,$(subst ::,/,$(PERLMOD_PACKAGES)))

ifeq ($(BUILD_MODE), release)
CARGO_BUILD_ARGS += --release
TARGET_DIR=release
else
TARGET_DIR=debug
endif

# better way to automate this without a massively annoying cargo-metatdata + jq
# chain that spits out errors?
PROXMOX_DOC_CRATES := \
    proxmox-apt \
    proxmox-apt-api-types \
    proxmox-base64 \
    proxmox-config-digest \
    proxmox-http \
    proxmox-http-error \
    proxmox-log \
    proxmox-notify \
    proxmox-openid \
    proxmox-resource-scheduling \
    proxmox-shared-cache \
    proxmox-subscription \
    proxmox-sys \
    proxmox-tfa \
    proxmox-time \
    proxmox-ve-config \

define make_rustdoc_url =
--extern-html-root-url $(subst -,_,$(1))=https://rustdoc.intra.proxmox.com/$(DEB_DISTRIBUTION)/packages/proxmox
endef
PROXMOX_DOC_URL_FLAGS = $(foreach crate,$(PROXMOX_DOC_CRATES),$(call make_rustdoc_url,$(crate)))

all: PVE
	cargo build $(CARGO_BUILD_ARGS)
	mkdir -p test/Proxmox/Lib
	sed -r -e \
	  's@^sub libfile.*$$@sub libfile { "$(shell pwd)/target/$(TARGET_DIR)/libpve_rs.so"; }@' \
	  Proxmox/Lib/PVE.pm >test/Proxmox/Lib/PVE.pm

Proxmox: Proxmox/Lib/PVE.pm
PVE: $(PERLMOD_PACKAGE_FILES)
Proxmox/Lib/PVE.pm $(PERLMOD_PACKAGE_FILES) &: Fixup.pm
	$(PERLMOD_GENPACKAGE) $(PERLMOD_PACKAGES)

check: all
	$(MAKE) -C test test

.PHONY: install
install: target/release/libpve_rs.so Proxmox/Lib/PVE.pm $(PERLMOD_PACKAGE_FILES)
	install -d -m755 $(DESTDIR)$(PERL_INSTALLVENDORARCH)/auto
	install -m644 target/release/libpve_rs.so $(DESTDIR)$(PERL_INSTALLVENDORARCH)/auto/libpve_rs.so
	install -d -m755 $(DESTDIR)$(PERL_INSTALLVENDORLIB)
	install -d -m755 $(DESTDIR)$(PERL_INSTALLVENDORLIB)/Proxmox/Lib
	install -m644 Proxmox/Lib/PVE.pm $(DESTDIR)$(PERL_INSTALLVENDORLIB)/Proxmox/Lib/PVE.pm
	find $(PM_DIR) \! -type d -print -exec install -Dm644 '{}' $(DESTDIR)$(PERL_INSTALLVENDORLIB)'/{}' ';'

clean:
	rm -rf PVE Proxmox
	cargo clean
	rm -f *.deb *.dsc *.tar.* *.build *.buildinfo *.changes Cargo.lock
	rm -rf $(PACKAGE)-[0-9]*/

.PHONY: dinstall
dinstall: $(DEBS)
	dpkg -i $(DEBS)

.PHONY: upload
upload: UPLOAD_DIST ?= $(DEB_DISTRIBUTION)
upload: $(DEBS)
	# check if working directory is clean
	git diff --exit-code --stat && git diff --exit-code --stat --staged
	tar cf - $(DEBS) | ssh -X repoman@repo.proxmox.com upload --product pve --dist $(DEB_DISTRIBUTION)

$(BUILDDIR): src debian test common/src Cargo.toml Makefile .cargo/config.toml
	rm -rf $(BUILDDIR) $(BUILDDIR).tmp
	mkdir $(BUILDDIR).tmp
	mkdir $(BUILDDIR).tmp/common
	cp -a -t $(BUILDDIR).tmp src debian test Cargo.toml Makefile .cargo Fixup.pm
	cp -a -t $(BUILDDIR).tmp/common common/src
	mv $(BUILDDIR).tmp $(BUILDDIR)

.PHONY: deb
deb: $(DEBS)
$(DEBS) &: $(BUILDDIR)
	cd $(BUILDDIR); PATH="/usr/local/bin:/usr/bin" dpkg-buildpackage -b -us -uc
	lintian $(DEBS)

.PHONY: dsc
dsc: $(DSC)
$(DSC): $(BUILDDIR)
	cd $(BUILDDIR); PATH="/usr/local/bin:/usr/bin" dpkg-buildpackage -S -us -uc -d
	lintian $(DSC)

sbuild: $(DSC)
	sbuild $(DSC)

.PHONY: doc
doc:
	RUSTDOCFLAGS='-Z unstable-options $(PROXMOX_DOC_URL_FLAGS)' \
	  cargo +nightly doc --no-deps

.PHONY: doc-open
doc-open:
	RUSTDOCFLAGS='-Z unstable-options $(PROXMOX_DOC_URL_FLAGS)' \
	  cargo +nightly doc --no-deps --open
