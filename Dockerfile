FROM rust:1.94.1-slim-bookworm

# 将 apt 源切换为国内镜像（清华 TUNA），加速 apt update / install。
# 同时兼容 Debian 12 的 /etc/apt/sources.list 与 Debian 13+ 的 deb822 格式。
RUN set -eux; \
    for f in /etc/apt/sources.list /etc/apt/sources.list.d/debian.sources; do \
        if [ -f "$f" ]; then \
            sed -i \
                -e 's|deb.debian.org|mirrors.tuna.tsinghua.edu.cn|g' \
                -e 's|security.debian.org|mirrors.tuna.tsinghua.edu.cn|g' \
                "$f"; \
        fi; \
    done

# 安装编译与打包依赖：合并为一层，--no-install-recommends 减少下载量，装完清理 apt 缓存。
RUN apt-get update \
 && apt-get install -y --no-install-recommends \
        build-essential libgtk-4-dev libglib2.0-dev \
        ruby ruby-dev rubygems rpm zstd libarchive-tools \
 && rm -rf /var/lib/apt/lists/*

# 通过国内 RubyGems 镜像（清华 TUNA）安装 fpm；--no-document 跳过 ri/rdoc 生成。
RUN gem sources --add https://mirrors.tuna.tsinghua.edu.cn/rubygems/ \
 && { gem sources --remove https://rubygems.org/ || true; } \
 && gem install fpm --no-document

# 通过国内 rustup 镜像（中科大 USTC）安装组件，两条 component 合并为一条命令。
ENV RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
ENV RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
RUN rustup component add clippy rustfmt

# 让 cargo 走国内 crates.io 镜像（中科大 USTC：索引与 crate 下载均在境内）。
RUN printf '%s\n' \
        '[source.crates-io]' \
        'replace-with = "ustc"' \
        '[source.ustc]' \
        'registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"' \
        > "$CARGO_HOME/config.toml"

##### Commands #####

WORKDIR /workspace

ENV DEBIAN_DEPS="--depends libgtk-4-1 --depends dbus-x11 --depends iproute2 --depends crunch --deb-recommends aircrack-ng"
ENV REDHAT_DEPS="--depends gtk4-devel --depends dbus-x11 --depends iproute --rpm-tag Recommends:aircrack-ng"
ENV ARCHLINUX_DEPS="--depends gtk4 --depends dbus --depends iproute2 --pacman-optional-depends aircrack-ng"

# Build and package the project
CMD cargo build && \
    fpm -f -t deb -p airgorah_`arch`.deb -a native $DEBIAN_DEPS && \
    fpm -f -t rpm -p airgorah_`arch`.rpm -a native $REDHAT_DEPS && \
    fpm -f -t pacman -p airgorah_`arch`.pkg.tar.zst -a native $ARCHLINUX_DEPS
