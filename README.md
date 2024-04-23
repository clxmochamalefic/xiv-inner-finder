# README

## HOW TO DEV / 開発環境構築

### SETUP

```
$ apt update
$ apt install libpq-dev
$ cargo install diesel_cli --no-default-features --features postgres
```

### MIGRATION / マイグレーション

```bash
$ diesel migration run
```

#### 初期セットアップ

```bash
$ export DATABASE_URL=postgres://<username>:<password>@localhost/<dbname>
$ diesel setup
$ diesel migration generate create_projects
```

## REFERENCE / 参考

- [Rust で MVP な Web API サーバを開発する方法](https://zenn.dev/tetter/books/webapi-mvp-book)
