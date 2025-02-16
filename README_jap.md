# Waifu Colorscripts

[🇺🇸 English](README.md) | [🇯🇵 日本語](README_jap.md)

ターミナルでアニメの「Waifus」を印刷するための小さな Rust プログラムです。

<img src="assets/Preview.jpg"/>

# 説明

特別なことは何もありません。[viuer](https://docs.rs/viuer/latest/viuer/) を使用してランダムな waifu を印刷するプログラムです。

> [!TIP]
> 画像フォルダーに独自の画像を追加できます。 <br/>

> [!IMPORTANT]
> 推奨される画像サイズは 250x250px です。

# インストール (ソース コードから)

### git リポジトリをクローン
```sh
git clone https://github.com/Akzestia/waifu-colorscripts.git
cd waifu-colorscripts
```

### インストール スクリプトを実行
```sh
sudo chmod +x install.sh
./install.sh
```

> [!NOTE]
> インストールが成功したら、クローンしたリポジトリのフォルダーを削除できます。

# インストール (最新リリースから)

最新リリースをダウンロード
```
wget -q -nv -O - https://api.github.com/repos/Akzestia/waifu-colorscripts/releases/latest \
| awk -F'"' '/browser_download_url/ && /\.tar\.gz"/ {print $4}' \
| xargs -I {} wget -q {}
```

アーカイブを解凍
```sh
tar -xvzf waifu-colorscripts-x.x.x.tar.gz
```

抽出したディレクトリに移動します
```sh
cd waifu-colorscripts-x.x.x
```

セットアップ スクリプトを実行します
```sh
sudo chmod +x setup.sh
./setup.sh
```

> [!NOTE]
> セットアップが成功したら、アーカイブと抽出したフォルダーの両方を削除できます。
