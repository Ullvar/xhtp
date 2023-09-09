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
```json
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
  }
]
```

The body_type must be one of `json | form | text`.
The `extract_variables` will extract variables from a json response and save it as a global vaiable.
Global variables can be used in any part of the request and should look like the `{{access_token}}` above.

<h3>
    Features
</h3>

Show help:
```
xhtp h
```

Simple GET request:
```
xhtp <url>
```

Import an openapi spec and save the requests in the config file:
```
xhtp i <path to openapi spec>
```

List all the urls in the config file:
```
xthp l
```

List all the details of a specific request:
```
xhtp l <request number>
```

Open the requests config file in your editor:
```
xhtp e
```
Will use the EDITOR variable if set, else `vi`

List all the global variables:
```
xhtp gl
```

Add or override a global variable:
```
xhtp ga <variable name> <variable value>
```

Delete a global variable:
```
xhtp gd <variable number>
```

Delete a url from the config file:
```
xhtp d
```


Works well together with [jq](https://jqlang.github.io/jq) for processing responses. 
