outdir ?= rpmbuild/SRPMS
here := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

.PHONY: srpm

srpm:
	$(here)/build-srpm.sh $(here)/wezterm.spec $(outdir)
