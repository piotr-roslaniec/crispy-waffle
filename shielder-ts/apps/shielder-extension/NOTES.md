# Notes

## Manifest

Consider limiting the `host_permissions` to selected domains:

```json
{
  "manifest": {
    "host_permissions": ["https://*/*"]
  }
}
```

Update content security policy

```json
{
  "manifest": {
    "content_security_policy": {
      "extension_pages": "script-src 'self' 'wasm-unsafe-eval'; default-src 'self';"
    }
  }
}
```
