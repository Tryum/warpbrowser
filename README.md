# WarpBrowser

Redirect hyperlinks to the appropriate browser given a set of rules.

## How to use

```
warpbrowser -h

Capture and redirects URLs based on rules

Usage: warpbrowser.exe [OPTIONS]

Options:
  -u, --url <url>       The URL to capture and redirect
      --register            Register in system browser list
      --select-browser      Select default browser
      --add-rule <website>  Add a new rule: specify a website
  -h, --help                Print help
  -V, --version             Print version
  ```


  ## What's next

  * Rules set, with time trigger (different set of rules given day and time)
  * Regexp rules
  * Add rules from history