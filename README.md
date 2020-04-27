# Websteer: Using multiple browsers with xdg's default browser

The core binary manages taking a toml config file to specify
browser launchers and regexes to match against. It possible to
have ambiguous names that will show a prompt.

## Usage

`websteer [-c config] [-d kdialog|zenity] [url]`
`websteer gen-config [path]`
`websteer gen-desktop [path]`

### Options

`-c config`
    Path to config file

`-d kdialog|zenity`
    Select dialog style. Overrides config file

`url`
    url to open. If empty, still launches the browser

`gen-desktop`
    generate desktop file entry. When `path` is given, write to file. Otherwise, $XDG_CONFIG_HOME/websteer/config

`gen-config`
    generates a default config. When `path` is given, write to file. Otherwise, $HOME/.local/share/applications

## Installation

### Cargo
```bash
cargo install websteer
websteer gen-config
websteer gen-desktop
update-desktop-database $HOME/.local/share/applications
xdg-settings set default-web-browser websteer.desktop
```

### Debian
```bash
dpkg -i websteer.deb
xdg-settings set default-web-browser websteer.desktop
```

## Dependencies
* `zenity`|`kdialog`: Presenting gui dialog for ambiguous urls
* `dex`: Launches desktop files

## Config File

.config/websteer/config

```
dialog=kdialog
default=personal
prompt=[personal, work]

[[browser]]
name=personal
desktop=person-browser.desktop
exec="/usr/bin/personal-browser"

[[browser]]
name=work
desktop=work-browser.desktop
exec="/usr/bin/personal-browser --various --args=foo"

[[browser]]
name=test
desktop=test-browser.desktop
exec="/usr/bin/other-browser"

[[rule]]
regex="localhost"
browser=test

[[rule]]
regex="workcorp"
browser=work

[[rule]]
regex="drive\.google\.com"
browser=work
ambiguous=true

[[rule]]
regex="facebook\.com"
browser=personal
```

* `dialog`: Dialog format. `kdialog` and `zenity` are allowed. Otherwise attempt to default based on gnome/kde
* `default`: Name of default browser to fall-back to
* `prompt`: List of browsers to include in dialog. Default to all available
* `browser`: Table of browsers
    * 'name'
    * 'desktop': xdg desktop-entry file. Launched with `dex`
    * 'exec`: Command to execute instead of desktop file.
* `rule`: Array of rule tables. Checked in sequence
    * `regex`: Regex to check against
    * `browser`: Name of browser to use to handle this url
    * `ambiguous`: Show prompt and highlight browser if defined


