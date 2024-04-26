# README for backend API for xivif

## SETUP your env

```bash
$ cargo install ssclient
echo 'export PATH="$PATH:$HOME/.cargo/bin"' >> ~/.bashrc
source ~/.bashrc
```

## APPEND SECRET KEY

```bash
$ ssclient -k secrets.key set xivapi_secret
```

