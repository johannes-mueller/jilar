# jilar' – Johannes' pugl-ui widget set for LV2 plugins

This is a widget set that I use for the UIs of my LV2 plugins. It uses
[pugl-ui](https://github.com/johannes-mueller/pugl-ui) for the general widget
management.

This repo is on GitHub, so that everyone can freely build, use, modify,
package, distribute my LV2 plugins. When you launch `cargo build` of one of my
plugins, this repo is pulled as dependency if needed. There should be no need
to clone or install this repository directly.

This widget set is far from a complete GUI toolkit. Widgets that I don't need
in one of my plugins simply are not and will not be implemented. If you still want
to use this widget set in your own plugins or other projects, you are free to
do so according to terms of [GPLv2](LICENSE). However, if you intent to do that
I strongly encourage you to fork the repo, and reimplement the `::exposed()`
methods of the widgets to make the look and feel suit your corporate identity
design.

There is no API documentation as of yet, because it is primarily intended for
my own use in my plugins. So documentation runs at low priority. Also be aware
that severe API changes my occur at any time, when I realize that there is some
design flaw in the current API. Don't expect any kind of backwards
compatibility.
