# repo-sync
auto commit and pull daemon


utilisation

repo-sync [args] or via a config.toml

- take path
- detect git repo
- pull remote repetively
- watch modification in it
- push commit automatically


args:

-r, --repo <REPO PATH>
-tpush <SECOND> example: -tpush 0.5
-tpull <SECOND> example: -tpush 60