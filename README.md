# caesa-rs 👑

A simple cli for "encrypting" ascii text using a shift cipher, also known as Caesar cipher.

## Usage

🔀️ The shift can be adjusted with the `--key/-k` parameter.

```bash
$ echo "my super secret text" | caesar -k 5  
rd xzujw xjhwjy yjcy
```
Only substitues ASCII alphabetic characters. Casing is preserved, whitespace and non-alphabetic characters are left unchanged.

🔴 Supplying a non-ASCII character will result in an error.
🔴 Don't use for acutal encryption. Please.
