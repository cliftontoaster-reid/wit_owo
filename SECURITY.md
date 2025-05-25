# Security

## Reporting a Vulnerability

If you discover a security vulnerability in this package, please send us an encrypted email at <clifton.toaster.reid@proton.me>. We recommend encryption but unencrypted is also fine—it’s mainly here if you want or feel it’s needed. If you really cannot send an email, please open a public issue using our [Bug issue template](https://github.com/cliftontoaster-reid/wit_owo/issues/new/choose). We know issues are public, but it’s better than nothing and we appreciate any report.

We will do our best to respond within 48 hours, but you can expect a reply as soon as we are able. As long as we get the information, we are more than thankful for your time.

### PGP Public Key

**Key Fingerprint:** `3cd5 d4db 6905 4692 372e 05e1 cff0 74de 8c8b 62b7`

You can verify and import our public key using:

```bash
# Import from keyserver
gpg --keyserver keyserver.ubuntu.com --recv-keys CFF074DE8C8B62B7

# Or import from the embedded key below
gpg --import <<EOF
-----BEGIN PGP PUBLIC KEY BLOCK-----

xjMEZKMSjhYJKwYBBAHaRw8BAQdAa3/dJoF5a4JKUaZKmnfIkYld6nr/ReT+
NEvuJ/DIy/bNP2NsaWZ0b24udG9hc3Rlci5yZWlkQHByb3Rvbi5tZSA8Y2xp
ZnRvbi50b2FzdGVyLnJlaWRAcHJvdG9uLm1lPsKMBBAWCgA+BYJkoxKOBAsJ
BwgJkM/wdN6Mi2K3AxUICgQWAAIBAhkBApsDAh4BFiEEPNXU22kFRpI3LgXh
z/B03oyLYrcAANtaAP9//gws9TK0tQB0whzY3tp1pSXKlgoxfxEScJaXD63S
bwEAnFeFFcHNDiaZgx6xReIkwPGjmnsn4Tedh8VbByku/wzOOARkoxKOEgor
BgEEAZdVAQUBAQdAX2TZN5ZeWK19amyfX0zDXsVQQAW6fJDC2oBaLzm/b3sD
AQgHwngEGBYIACoFgmSjEo4JkM/wdN6Mi2K3ApsMFiEEPNXU22kFRpI3LgXh
z/B03oyLYrcAALAAAP4uSuRJWaz6Ij79/PUpoWuUcghdT9F88A0qwoNx6Aek
4wD+KjOHjEXFtdySZH9ta/jHwp/d0Ow8CZ7rBeui/k06WAw=
=+fEM
-----END PGP PUBLIC KEY BLOCK-----
EOF
```

**Key Details:**

- Algorithm: ECC (Curve25519)
- Created: 3 Jul 2023
- Expires: Never

> **Note:** Always verify the key fingerprint matches before using it to encrypt sensitive information.

We will acknowledge your report within 48 hours and work to deliver a fix as quickly as possible.

## Security Policy

We follow a responsible disclosure model. We will not publicly disclose any vulnerability until a fix has been merged and published, nor until affected users have been notified.

## Supported Versions

We maintain the following release range:

- **Current stable release** (latest version)
- **Previous minor versions** within the current major version
- **Previous major version** (critical security fixes only, best effort)

### Backport Strategy

- **Patch releases**: All fixes within the current major version
- **Critical security fixes**: May be backported to the previous major version when feasible
- **Non-critical fixes**: Current major version only

### Upgrade Recommendation

To ensure you receive the latest security fixes and features, please upgrade to the current stable release as soon as possible. We strongly recommend staying within the current major version for full support.
