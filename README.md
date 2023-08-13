# yktotp-jsonapi

[![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/nilsalex/yktotp-jsonapi/badge)](https://securityscorecards.dev/viewer/?uri=github.com/nilsalex/yktotp-jsonapi)

**❗This is still work in progress and only tested on Windows 10 and a Linux operating system in conjunction with
Google Chrome Beta.❗**

Get TOTP codes from a YubiKey. Intended to be called from the
[yktotp browser extension](https://github.com/nilsalex/yktotp).

## Installation

### Windows
- Clone the repository and ensure you have `cargo` (Rust) available on your path
- In a powershell with administrator rights, run `.\install.ps1`.

### Linux
- Run `cargo build --release`. This builds the executable `target/release/yktotp-jsonapi`.
- Edit the [Native Messaging host manifest](manifest/de.nilsalex.yktotp.json) and update the `path` field
  to the path of the `yktotp-jsonapi` executable.
- Install the manifest by copying it to the directory `~/.config/google-chrome/NativeMessagingHosts/`
   (or `~/.config/google-chrome-beta/NativeMessagingHosts/` for Google Chrome Beta).

## Usage

Install the [yktotp browser extension](https://github.com/nilsalex/yktotp) and follow the usage instructions
there to retrieve an OTP from the YubiKey from within your browser.

The executable accepts input from `stdin` and writes output to `stdout`. The first four bytes of the input
specify the length of the payload (the *message*). The message is a UTF-8 encoded JSON object and is expected
to contain a field named `account` with a string value. `yktotp-jsonapi` requests an OTP for this account from
the YubiKey and, if successful, returns the OTP in the `code` field of the response message.
Otherwise, the response message will contain an error string in the `error` field. The response is again a
UTF-8 encoded JSON prefixed with four bytes representing the length of the message.
