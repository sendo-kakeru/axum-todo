- rest api
- 生SQLに近いDB操作
- ddd
- テスト駆動開発
  1. 失敗するテストを書き、走らせて想定通りの理由で失敗することを確かめる。
  2. 十分な量のコードを書くか変更して新しいテストを通過するようにする。
  3. 追加または変更したばかりのコードをリファクタリングし、テストが通り続けることを確認する。
  4. 手順1から繰り返す！

## コマンド
### dbリセット
```bash
sh scripts/reset-db.sh
```


### コンテナpsqlログイン
```bash
docker-compose exec dev-postgres bash
```

```bash
psql -U dev -d dev
```