<h2 align="center">
        <img height="100" alt="xhtp" src="https://github.com/Ullvar/xhtp/blob/master/docs/xhtp-logo.png" />
    <br>
    xhtp CLI: Simple and fast HTTP client for the terminal
</h2>

To install (for arm OSX only):
```
curl -LO https://xhtp-release.s3.eu-north-1.amazonaws.com/install-xhtp.sh && chmod +x install-xhtp.sh && sudo ./install-xhtp.sh
```

To upgrade (for arm OSX only):
```
curl -LO https://xhtp-release.s3.eu-north-1.amazonaws.com/upgrade-xhtp.sh && chmod +x upgrade-xhtp.sh && sudo ./upgrade-xhtp.sh
```

<h3 align="center">
    Features
</h3>

Show help:
```
xhtp help
```

Simple GET request:
```
xhtp <url>
```

Import an openapi spec and save the requests in the config file:
```
xhtp import <path to openapi spec>
```

List all the urls in the config file:
```
xthp list
```

List all the details of a specific request:
```
xhtp list <request number>
```

Open the requests config file in your editor:
```
xhtp edit
```

Manage all the global variables:
```
xhtp global
```

Delete a url from the config file:
```
xhtp delete
```
