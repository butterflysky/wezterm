Name: wezterm
Version: %{_version}
Release: 1%{?dist}
Packager: Wez Furlong <wez@wezfurlong.org>
License: MIT
URL: https://wezfurlong.org/wezterm/
Summary: Wez's Terminal Emulator.
Requires: dbus, fontconfig, openssl, libxcb, libxkbcommon, libxkbcommon-x11, libwayland-client, libwayland-egl, libwayland-cursor, mesa-libEGL, xcb-util-keysyms, xcb-util-wm

%description
wezterm is a terminal emulator with support for modern features
such as fonts with ligatures, hyperlinks, tabs and multiple
windows.

%prep
# maybe generate %files manifest here?

%build
# nothing to do

%install
set -x
rm -rf %{buildroot}
mkdir %{buildroot}
cd %{buildroot}
tar xvf %{_sourcesdir}/wezterm.tar

%files
%defattr(-, root, root, -)
%attr(0755, root, root) /usr/bin/wezterm
%attr(0755, root, root) /usr/bin/wezterm-mux-server
%attr(0755, root, root) /usr/bin/wezterm-gui
%attr(0755, root, root) /usr/bin/strip-ansi-escapes
%attr(0644, root, root) /usr/share/icons/hicolor/128x128/apps/org.wezfurlong.wezterm.png
%attr(0644, root, root) /usr/share/applications/org.wezfurlong.wezterm.desktop
%attr(0644, root, root) /usr/share/metainfo/org.wezfurlong.wezterm.appdata.xml
%attr(0644, root, root) /etc/profile.d/wezterm.sh
