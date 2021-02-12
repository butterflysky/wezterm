#!/bin/bash
set -e
set -x

which dnf && dnf -y install git curl || yum -y install git curl

SPEC=$1
OUTDIR=$2

REPO_ROOT=$(git rev-parse --show-toplevel)

TMPDIR=$(mktemp -d)
TOPDIR=$TMPDIR/_topdir
BUILDROOT=$TOPDIR/BUILDROOT

# wezterm version for rpm
TAG_NAME=${TAG_NAME:-$(git describe --tags || true)}
TAG_NAME=${TAG_NAME:-$(date +'%Y%m%d-%H%M%S')-$(git log --format=%h -1)}
WEZTERM_RPM_VERSION=$(echo $TAG_NAME | sed -e 's/-/_/g')

echo setting up working directories

mkdir -p $TOPDIR/{BUILDROOT,SOURCES}

# clone repo so we can iterate on this script
cp -r $REPO_ROOT $TMPDIR/scratch
cd $TMPDIR/scratch

echo updating submodules
git submodule update --init --recursive
echo installing build deps
./get-deps

echo installing rustup
curl https://sh.rustup.rs/ -sSf | bash -s -- -y --default-toolchain stable --profile minimal -c rustfmt

echo building wezterm
~/.cargo/bin/cargo build --release

echo creating source tarball
install -Dsm755 target/release/wezterm -t             $BUILDROOT/usr/bin
install -Dsm755 target/release/wezterm-mux-server -t  $BUILDROOT/usr/bin
install -Dsm755 target/release/wezterm-gui -t         $BUILDROOT/usr/bin
install -Dsm755 target/release/strip-ansi-escapes -t  $BUILDROOT/usr/bin
install -Dm644 assets/shell-integration/wezterm.sh -t $BUILDROOT/etc/profile.d
install -Dm644 assets/icon/terminal.png               $BUILDROOT/usr/share/icons/hicolor/128x128/apps/org.wezfurlong.wezterm.png
install -Dm644 assets/wezterm.desktop                 $BUILDROOT/usr/share/applications/org.wezfurlong.wezterm.desktop
install -Dm644 assets/wezterm.appdata.xml             $BUILDROOT/usr/share/metainfo/org.wezfurlong.wezterm.appdata.xml

cd $(BUILDROOT) ; tar cvf $TOPDIR/SOURCES/wezterm.tar .

echo building source rpm
rpmbuild \
	-D "_topdir $TOPDIR" \
	-D "_srcrpmdir $OUTDIR" \
	-D "_version $WEZTERM_RPM_VERSION" \
	-bs $SPEC

echo cleaning up
cd / ; rm -rf $TMPDIR
