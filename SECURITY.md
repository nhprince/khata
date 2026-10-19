# Security policy

## Reporting a vulnerability

Please do not open a public issue for a security problem. Report it privately:

- GitHub: the repository's **Security** tab › **Report a vulnerability** (private vulnerability
  reporting), or
- email: contact@nhprince.me

Include the version, your system, and the steps to reproduce. You will get a reply as soon as
possible; fixes ship in the next release.

## Supported versions

Only the latest release receives security fixes.

## Verifying a download

Every release has a `SHA256SUMS.txt` with the SHA-256 of each file:

```sh
sha256sum -c SHA256SUMS.txt --ignore-missing      # Linux
shasum -a 256 -c SHA256SUMS.txt --ignore-missing  # macOS
Get-FileHash .\khata-<version>-windows-x64.msi    # Windows (compare with the file)
```
