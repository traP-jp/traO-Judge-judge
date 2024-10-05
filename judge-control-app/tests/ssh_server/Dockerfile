FROM ubuntu:latest

# 必要なパッケージをインストール
RUN apt-get update && apt-get install -y \
    openssh-server

# SSH用のディレクトリを作成
RUN mkdir /var/run/sshd

# rootユーザーのパスワードを設定
RUN echo 'root:password' | chpasswd

# rootでのSSHアクセスを許可、パスワード認証を有効に
RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config
RUN sed -i 's/#PasswordAuthentication yes/PasswordAuthentication yes/' /etc/ssh/sshd_config

# SSHデーモンのリッスンポートを2022に変更
RUN sed -i 's/#Port 22/Port 2022/' /etc/ssh/sshd_config

# テスト用のフラッグを設定
RUN echo 'TEST_FLAG' > /flag

# 2022番ポートを開放
EXPOSE 2022

# SSHサービスを起動
CMD ["/usr/sbin/sshd", "-D"]
