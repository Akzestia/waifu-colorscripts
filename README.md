# Waifu Colorscripts

[🇺🇸 English](README.md) | [🇯🇵 日本語](README_jap.md)

Small rust program for printing anime `Waifus` in your terminal.

<img src="assets/Preview.jpg"/>

# Description

Nothing special, just a program that uses [viuer](https://docs.rs/viuer/latest/viuer/) to print a random waifu.

> [!TIP]  
> You can add your own images to image folder. <br/>

> [!IMPORTANT]  
> Recommended image size is 250x250px.

# Installation (From Source Code)

### Clone git repo
```sh
git clone https://github.com/Akzestia/waifu-colorscripts.git
cd waifu-colorscripts
```

### Run install script
```sh
sudo chmod +x install.sh
./install.sh
```

> [!NOTE]  
> You can delete the cloned repo's folder after a successful installation.

# Installation (From Latest Release)

Download the Latest Release
```
wget -q -nv -O - https://api.github.com/repos/Akzestia/waifu-colorscripts/releases/latest \
  | awk -F'"' '/browser_download_url/ && /\.tar\.gz"/ {print $4}' \
  | xargs -I {} wget -q {}
```

Unpack the Archive
```sh
tar -xvzf waifu-colorscripts-x.x.x.tar.gz
```

Navigate to the Extracted Directory
```sh
cd waifu-colorscripts-x.x.x
```

Run setup script
```sh
sudo chmod +x setup.sh
./setup.sh
```

> [!NOTE]  
> You can delete both, archive and extracted folder after a successful setup.
