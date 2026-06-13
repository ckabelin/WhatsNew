# Inter font

WhatsNew uses [Inter](https://github.com/rsms/inter) (OFL-1.1, open source) as its UI
font, self-hosted so the app works fully offline.

To add it, download `InterVariable.woff2` from the
[Inter releases](https://github.com/rsms/inter/releases) and place it in this
directory alongside the upstream `LICENSE.txt` (OFL-1.1). Until the file is
present, `app.css` falls back to the system UI font stack, so the app remains
fully usable without it.
