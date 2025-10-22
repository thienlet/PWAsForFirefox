![Logo](https://rawcdn.githack.com/wiki/filips123/PWAsForFirefox/images/banner.svg)

Progressive Web Apps for Firefox
================================

[![Release](https://img.shields.io/github/v/release/filips123/PWAsForFirefox?sort=semver&style=flat-square)](https://github.com/filips123/PWAsForFirefox/releases/latest)
[![Users](https://img.shields.io/amo/users/pwas-for-firefox?style=flat-square)](https://addons.mozilla.org/firefox/addon/pwas-for-firefox/)
[![Rating](https://img.shields.io/amo/rating/pwas-for-firefox?style=flat-square)](https://addons.mozilla.org/firefox/addon/pwas-for-firefox/reviews/)
[![License](https://img.shields.io/github/license/filips123/PWAsForFirefox?style=flat-square)](https://github.com/filips123/PWAsForFirefox/blob/main/LICENSE)
[![Repositories](https://img.shields.io/repology/repositories/firefoxpwa?style=flat-square)](https://repology.org/project/firefoxpwa/versions)
[![Packagecloud.io DEB](https://img.shields.io/badge/deb-packagecloud.io-844fec.svg?style=flat-square)](https://packagecloud.io/filips/FirefoxPWA)
[![Packagecloud.io RPM](https://img.shields.io/badge/rpm-packagecloud.io-844fec.svg?style=flat-square)](https://packagecloud.io/filips/FirefoxPWA)

A tool to install, manage and use Progressive Web Apps (PWAs) in Mozilla Firefox.

## Description

[Progressive Web Apps (PWAs)](https://developer.mozilla.org/docs/Web/Progressive_web_apps) are web apps that use web APIs and features along with progressive enhancement strategy to bring a native app-like user experience to cross-platform web applications. Although Firefox supports many of Progressive Web App APIs, it does not support functionality to install them as a standalone system app with an app-like experience.

This project creates a custom modified Firefox runtime to allow websites to be installed as standalone apps and provides a console tool and browser extension to install, manage and use them.

## Usage

**TLDR**: Install [the browser extension](https://addons.mozilla.org/firefox/addon/pwas-for-firefox/) and follow in-browser installation instructions. You can read [the documentation website](https://pwasforfirefox.filips.si/) for usage instructions and other useful resources.

For more details and technical documentation about setting up, using, and developing the project, see the READMEs of the native and extension part:

* [Native](native/README.md)
* [Extension](extension/README.md)

## Features

### Available Features

* Command-line tool to install, manage and run Progressive Web Apps in Firefox.
* Extension to set up native programs, and install, manage and run PWAs and their profiles directly from the main Firefox browser.
* Isolated Firefox installation and profile(s) that store the PWAs.
* Installed PWAs have their own start/app menu entry and taskbar icon, and live in their own window.
* Installed PWAs have no tabs and address bar for a better app-like feel.
* Support for installing all websites as Progressive Web Apps.
* Support for all Firefox addons/extensions and built-in Firefox features.
* Support for automatic (user-triggered) installation and patching of installation and profile(s).

### Planned Features

* Support for more system-related web app manifest features (once they are standardized).

### Not Planned Features

* **Integration into official Firefox code.** This project currently modifies the browser chrome (UI) at runtime using JS and CSS. Although this works, it is officially unsupported by Mozilla and can break with Firefox updates. To contribute features back into the official Firefox code, they would need to be implemented properly with the new chrome page and browser services. Unfortunately, this requires an almost complete rewrite of the project, and I currently don't have enough knowledge and time to do that.

* **Using the same installation profile for PWAs and normal browsing.** This could make the main browser installation/profile unstable if things break. It would also prevent customizing the PWA profile to work better as a PWA profile, and installing custom addons. If you want to sync data between your main and PWA profile, I recommend using Firefox Account or a third-party sync solution.

* **Running PWAs installed as Windows APPX/MSIX packages or from Microsoft Store.** They will always use Chromium-based Edge that is installed on Windows 10/11. I'm not sure if it is possible to override this. If it is not too hard and doesn't cause any problems, I may try this in the future.

* **Support for Chromium-specific APIs (Filesystem, Bluetooth, NFC, USB...).** This would require forking and directly modifying the Firefox source. Also, I'm not sure if giving websites the same privileges as native apps is the best idea...

### Current Limitations

You can [our documentation website](https://pwasforfirefox.filips.si/about/current-limitations/) for a list of current limitations.

## Supporters

### Sponsors

Thanks to [packagecloud.io](https://packagecloud.io/) for sponsoring this project and providing us a free hosting for our DEB and RPM packages!

  [<img src="https://assets-production.packagecloud.io/assets/packagecloud-logo-light-3c521566d5567fe0ce8435ef1f9485b0c3ad28a958af6f520d82ad3b232d2ff3.png" alt="Private NPM repository and Maven, RPM, DEB, PyPi and RubyGems repository · packagecloud" width="500">](https://packagecloud.io/)

&nbsp;

Thanks to [SignPath Foundation](https://signpath.org/) for providing us a free code signing certificate for Windows packages and [SignPath](https://about.signpath.io/) for providing the code signing infrastructure!

  [<img src="https://signpath.org/assets/logo.svg" alt="Free Code Signing for Open Source software · SignPath" width="500">](https://signpath.org/)

&nbsp;

Thanks to all donors for providing financial support for the project!</br>
<sub>Please check [supported donation services](https://github.com/filips123/PWAsForFirefox?sponsor=1) if you want to help the project by donating.</sub>

### Contributors

Thanks to [all contributors](https://github.com/filips123/PWAsForFirefox/graphs/contributors) to this project for providing help and developing features!

  [![Contributors](https://contrib.rocks/image?repo=filips123/PWAsForFirefox)](https://github.com/filips123/PWAsForFirefox/graphs/contributors)

### Other Mentions

Thanks to [all package maintainers](https://repology.org/project/firefoxpwa/information) making sure the project is up-to-date! Thanks to [all translators](https://crowdin.com/project/firefoxpwa) making the project available in multiple languages! Thanks to [all stargazers](https://github.com/filips123/PWAsForFirefox/stargazers) who starred our repository on GitHub. Finally, thanks to Mozilla and its developers for creating Firefox and making it possible to modify its UI using JavaScript!

## Versioning

The project uses [SemVer](https://semver.org/) for versioning. For the available versions and the changelog, see [the releases](https://github.com/filips123/PWAsForFirefox/releases) on this repository.

The native and extension part of the project at released tags are compatible with each other according to SemVer. The native and extension part at non-tagged commits may not be compatible with each other, because they are development versions that may not be both updated at the same time.

The project aims for compatibility with the latest stable Firefox version. It may not be compatible with the others.

## License

The project is licensed under the Mozilla Public License 2.0. By using, redistributing, or modifying it, you must agree to the license, and the additional clauses provided below. See [the LICENSE file](LICENSE) for the full license text.

The project uses additional third-party assets and code:

* The project logo is based on [the "Fox SVG Vector" icon](https://www.svgrepo.com/svg/40267/fox) and [the community-introduced PWA logo](https://github.com/webmaxru/progressive-web-apps-logo), both dedicated to the public domain using [CC0](https://creativecommons.org/publicdomain/zero/1.0/).

* Browser chrome modifications were inspired by and partially derived from the [`xiaoxiaoflood/firefox-scripts`](https://github.com/xiaoxiaoflood/firefox-scripts) repository on GitHub, licensed under the Mozilla Public License 2.0. Detailed information can be found in the respective files.

* Browser chrome modifications partially use code derived from the [`black7375/Firefox-UI-Fix`](https://github.com/black7375/Firefox-UI-Fix) repository on GitHub, licensed under the Mozilla Public License 2.0. Detailed information can be found in the respective files.

* Browser chrome modifications partially use code and icons derived from [the original Firefox source](https://github.com/mozilla/gecko-dev), licensed under the Mozilla Public License 2.0. Detailed information can be found in the respective files.

* Native programs contain [the Metropolis Semi Bold typeface](https://fontsarena.com/metropolis-by-chris-simpson/) by Chris Simpson, released into the public domain using [Unlicense](https://unlicense.org/).

* Windows installer contains [Bootstrap Icons](https://icons.getbootstrap.com/), licensed under [the MIT License](https://opensource.org/licenses/MIT). Detailed license information can be found in [the WiX configuration file](native/packages/wix/main.wxs).

Additional open source software will be downloaded and installed at runtime when initiated by the user:

* Installing the runtime on Windows will install [7-Zip](https://7-zip.org/) if it is not already installed. The 7-Zip project is made by Igor Pavlov and [licensed under the GNU LGPL license and others](https://7-zip.org/license.txt). This project is not affiliated with the 7-Zip project or its developers in any way.

* Installing the runtime on any system will download the unmodified [Mozilla Firefox](https://www.mozilla.org/firefox/) browser and locally modify it. By using the runtime you agree to the Firefox [Terms of Use](https://www.mozilla.org/about/legal/terms/firefox/) and [Privacy Notice](https://www.mozilla.org/privacy/firefox/). Firefox is licensed under the Mozilla Public License 2.0. Firefox and the Firefox logo are trademarks of the Mozilla Foundation in the U.S. and other countries. This project is not affiliated with the Mozilla Foundation in any way.
