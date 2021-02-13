outdir ?= rpmbuild/SRPMS
here := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

.PHONY: srpm

srpm:
	.copr/build-srpm.sh $(here)/.copr/wezterm.spec $(outdir)
