# JobService における OutcomeToken の扱い

## 1 OutcomeToken = 1 .tar.gz = 1 directory or file

JobService では、OutcomeToken と実際のファイルが 1 対 1 対応になるように管理されています。具体的には、1 つの OutcomeToken が 1 つの `.tar.gz` ファイルを指し、その `.tar.gz` を展開すると必ず 1 つのファイルまたはディレクトリが出現します。さらに、OutcomeToken の UUID、`.tar.gz` ファイル名、展開されるファイル/ディレクトリ名がすべて同じになるように設計されています。

## 全体の流れ

1. **ファイル準備**: 実行に必要なファイルを`place_file`で準備します
2. **結果格納先準備**: 実行結果を格納するための空ディレクトリを`place_file`で作成します
3. **リモート実行**: 1 で準備したファイルと 2 で作成した格納先をリモートサーバに送信して実行します
4. **結果受信**: 2 で指定した格納先から実行結果を受信します

## 詳細フロー

### 1. ファイル準備段階（place_file）

実行に必要なファイルを準備します。place_file は 3 種類の入力を受け付けます。

- `FileConf::EmptyDirectory`: 新しい UUID が生成され、その UUID をフォルダ名とする空ディレクトリが`.tar.gz`形式で作成されます
- `FileConf::Text(resource_id)`: S3 から指定されたリソースを取得し、新しい UUID をファイル名とするテキストファイルが`.tar.gz`形式で作成されます
- `FileConf::RuntimeText(content)`: 渡されたテキスト内容から、新しい UUID をファイル名とするテキストファイルが`.tar.gz`形式で作成されます

いずれの場合も、作成された`.tar.gz`ファイルは`outcomes/{UUID}.tar.gz`として保存され、OutcomeToken が返されます。

### 2. 結果格納先準備

`place_file`で空ディレクトリを作成し、実行結果の格納先 UUID を事前に決定しておきます。

### 3. リモート実行段階（execute）

1 で準備した依存ファイル（OutcomeToken 群）と、2 で作成した OutcomeToken をリモートサーバに送信します。

リモートサーバでは、受信した各`.tar.gz`ファイルを`/outcomes/{UUID}/`または`/outcomes/{UUID}`として展開し、環境変数で各ファイルのパスを指定して Docker コンテナ内でスクリプトを実行します。

重要な点は、実行結果は 2 で指定したディレクトリ（`/outcomes/{2で作成したOutcomeTokenのUUID}/`）に保存されることです。

### 4. 結果受信段階

実行完了後、リモートサーバは 2 で指定したディレクトリを`.tar.gz`として圧縮して返します。job_service は、この結果を 2 で決定していた同じ UUID を使って OutcomeToken を作成し、`outcomes/{同じUUID}.tar.gz`として保存します。

このように事前に結果の格納先 UUID を決定し、リモート側に指定の場所に結果を保存させることで、戻ってきた `.tar.gz` は必ず指定された UUID 構造を持つことが保証されます。これにより、実行結果として得られた OutcomeToken を次の execute の依存ファイルとして使いまわすことができます。
