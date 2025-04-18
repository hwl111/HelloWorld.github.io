### Build & Run
预先条件：
需要PC上安装poetry（最好是最新版本）以及python3.8及以上版本

poetry 安装方式（推荐pipx）：
```shell
    pipx install poetry
```

pipx的安装请见官网 https://pipx.pypa.io/latest/installation/

在pyproject.toml同级目录下执行以下命令即可
```shell
    poetry run main
```

注：
如果pyproject.toml同级目录下不存在poetry.lock文件，请先运行下面指令
```shell
    poetry install
```