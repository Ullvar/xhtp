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

The requests config should look like this:
`
[
  {
    "method": "GET",
    "url": "http://localhost:3000/api/data",
    "headers": [
          "Authorization: Bearer {{access_token}}"
    ],
    "body_type": null,
    "body": null,
    "extract_variables": [
      {
        "key_path": "name",
        "variable_name": "name"
      }
    ]
  },
]
`

The body_type must be one of `json | form | text`.
The `extract_variables` will extract variables from a json response and save it as a global vaiable.
Global variables can be used in any part of the request and should look like the {{access_token}} above.

<h3 align="center">
    Features
</h3>

Show help (shorthand `h`):
```
xhtp help
```

Simple GET request:
```
xhtp <url>
```

Import an openapi spec and save the requests in the config file (shorthand `i`):
```
xhtp import <path to openapi spec>
```

List all the urls in the config file (shorthand `l`):
```
xthp list
```

List all the details of a specific request (shorthand `l`):
```
xhtp list <request number>
```

Open the requests config file in your editor (shorthand `e`):
```
xhtp edit
```
Will use the EDITOR variable if set, else `vi`

Manage all the global variables (shorthand `g`):
```
xhtp global
```

Delete a url from the config file (shorthand `d`):
```
xhtp delete
```


Works well together with [jq](https://jqlang.github.io/jq) for processing responses. 
