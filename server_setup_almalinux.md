## Install using alma linux operating system

```sh
sudo dnf update && sudo dnf upgrade
sudo dnf install epel-release
sudo dnf config-manager --set-enabled crb
sudo dnf update && sudo dnf upgrade
sudo dnf install git bind-utils make gcc patch zlib-devel bzip2 bzip2-devel readline-devel sqlite sqlite-devel openssl-devel tk-devel libffi-devel xz-devel libuuid-devel gdbm-libs libnsl2 fzf bat wget readline-devel net-tools firewalld nano dnf-utils glibc-langpack-en fontawesome-fonts btop pkgconf perl-FindBin perl-IPC-Cmd openssl-devel opendkim opendkim-tools perl-core zlib-devel valkey -y

sudo dnf install git bind-utils make gcc patch zlib-devel bzip2 bzip2-devel readline-devel sqlite sqlite-devel openssl-devel tk-devel libffi-devel xz-devel libuuid-devel gdbm-libs libnsl2 fzf bat helix fd-find wget readline-devel net-tools firewalld nano dnf-utils glibc-langpack-en fontawesome-fonts btop pkgconf perl-FindBin perl-IPC-Cmd openssl-devel perl-core zlib-devel valkey -y
sudo dnf group install "Development Tools"
sudo timedatectl set-timezone Asia/Makassar
sudo hostnamectl set-hostname yourdomain.com
```

## Install zsh

```
sudo dnf install zsh
sudo usermod --shell /bin/zsh bendo01
```

- [zsh](https://www.tecmint.com/change-a-users-default-shell-in-linux/)
- [oh-my-zsh](https://ohmyz.sh/#install)
- [PowerLevell10K](https://github.com/romkatv/powerlevel10k?tab=readme-ov-file#oh-my-zsh)
- [zsh autosuggestion](https://github.com/zsh-users/zsh-autosuggestions/blob/master/INSTALL.md)

```
sh -c "$(wget https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh -O -)"
git clone --depth=1 https://github.com/romkatv/powerlevel10k.git "${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k"
git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions
nano ~/.zshrc

plugins=(
    # other plugins...
    zsh-autosuggestions
)
```

## install certbot

```sh
sudo dnf install snapd
sudo systemctl enable --now snapd.socket
sudo ln -s /var/lib/snapd/snap /snap

sudo snap install --classic certbot
sudo ln -s /snap/bin/certbot /usr/bin/certbot
sudo certbot --nginx
```

## Install Rust

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

cargo install eza
sudo dnf install fzf
sudo dnf install bat
sudo dnf install helix
sudo dnf install fd-find
sudo dnf install wget
sudo dnf install readline-devel
sudo dnf install net-tools
sudo dnf install firewalld (https://www.liquidweb.com/blog/fail2ban-install-tutorial-for-linux-almalinux/)
```

## Install zed

```
curl -f https://zed.dev/install.sh | sh
```

## Install Nginx

- [nginx](https://www.jaspreet.net/2024/09/08/2661/how-to-install-stable-and-mainline-nginx-on-red-hat-rocky-linux-almalinux-and-oracle-linux/)

```
sudo mkdir sites-available
sudo mkdir sites-enabled
```

nginx default path: /usr/share/nginx/html/

Certificate is saved at: /etc/letsencrypt/live/mail.xsia.app/fullchain.pem
Key is saved at: /etc/letsencrypt/live/mail.xsia.app/privkey.pem

## Install PostgreSQL with pgvector extension

- [PostgreSQL 17](https://cloudspinx.com/install-postgresql-17-on-rocky-almalinux-centos/)
- [PostgreSQL 17 UUID](https://stackoverflow.com/questions/63189760/postgresql-uuid-ossp-control-file-missing-in-extention-folder-i-have-installed)

```sh
sudo dnf install -y https://download.postgresql.org/pub/repos/yum/reporpms/EL-9-x86_64/pgdg-redhat-repo-latest.noarch.rpm
sudo dnf -qy module disable postgresql
sudo dnf install -y postgresql17-server postgresql17-contrib
sudo /usr/pgsql-17/bin/postgresql-17-setup initdb
sudo systemctl enable postgresql-17
sudo systemctl start postgresql-17
sudo -u postgres psql
CREATE ROLE bendo01 SUPERUSER LOGIN PASSWORD 'talaso';
CREATE DATABASE bendo01;
```

- [Pg Vector](https://github.com/pgvector/pgvector)
- [uuid v7](https://github.com/fboulnois/pg_uuidv7)
- [Super User](https://neon.tech/postgresql/postgresql-administration/create-superuser-postgresql)
- [Change User Password](https://stackoverflow.com/questions/12720967/how-can-i-change-a-postgresql-user-password)
- [Change User can login](https://stackoverflow.com/questions/35254786/postgresql-role-is-not-permitted-to-log-in)
- [Install pgvector](https://github.com/pgvector/pgvector?tab=readme-ov-file#yum)
- [Install valkey](https://valkey.io/topics/installation/)
- [openport 80 and 443](https://www.basezap.com/open-close-ports-almalinux-guide/)

path bin postgresql alma linux 9

/usr/pgsql-17/bin

```sh
sudo dnf install pgvector_17
```

- Install Chromonium

```sh
sudo dnf install chromium -y
```

- install ncurses

```sh
sudo yum install ncurses-devel
```

- [Install SQLite3](https://www.yougethost.com/installing-sqlite3-on-almalinux-9-a-step-by-step-guide/)

```sh
sudo dnf install sqlite-devel -y
```

- python dependies

- [install python latest](https://cloudspinx.com/how-to-install-python-3-13-on-rocky-linux-almalinux/)
- [install python switcher](https://github.com/pyenv/pyenv?tab=readme-ov-file#installation)

# Compile

```sh
export RUST_ENV=production
```

```sh
cargo build --release
sudo setsebool -P httpd_can_network_connect 1


# Set ownership to nginx user (usually "nginx" or "www-data"):
sudo chown -R nginx:nginx /usr/share/nginx/html/server

# Set permissions (directories: 755, files: 644):
sudo find /usr/share/nginx/html/server -type d -exec chmod 755 {} \;
sudo find /usr/share/nginx/html/server -type f -exec chmod 644 {} \;
ls -Z /usr/share/nginx/html/server
sudo chcon -R -t httpd_sys_content_t /usr/share/nginx/html/server
sudo tail -n 50 /var/log/nginx/siaka_tritunas_ac_id_error.log
sudo namei -l /usr/share/nginx/html/server

sudo semanage fcontext -a -t httpd_sys_content_t "/usr/share/nginx/html/server(/.*)?"
sudo restorecon -Rv /usr/share/nginx/html/server
sudo chcon -R -t httpd_sys_content_t /usr/share/nginx/html/server
sudo semanage port -a -t http_port_t -p udp 443
sudo restorecon -v /usr/share/nginx/html/server/xsia_loco-cli
```

# Setting Mail Server

```
sudo dnf install nmap
```

[setting email](https://www.linuxbabe.com/redhat/run-email-server-on-rocky-linux-9-alma-linux-9-postfix-smtp-server)

/etc/systemd/system/xsia.service

```service
[Unit]
Description=xSIA API Server
After=network.target

[Service]
User=bendo01
WorkingDirectory=/usr/share/nginx/html/server
Environment="APP_ENVIRONMENT=production"
Environment="RUST_BACKTRACE=1" # For debugging
ExecStart=/usr/share/nginx/html/server/xsia_loco-cli start -s
Restart=on-failure
RestartSec=3

[Install]
WantedBy=multi-user.target
```

The "Permission denied" error indicates either incorrect file permissions or SELinux blocking execution. Here's how to resolve this:

1. Verify Execute Permissions & Ownership

bash
Copy

# Check permissions (must have -rwxr-xr-x)

ls -l /usr/share/nginx/html/server/xsia_loco-cli

# Add execute permissions if missing

sudo chmod +x /usr/share/nginx/html/server/xsia_loco-cli

# Set proper ownership (user bendo01)

sudo chown bendo01:bendo01 /usr/share/nginx/html/server/xsia_loco-cli

2. Verify Directory Permissions

bash
Copy

# Ensure user bendo01 has access to the entire path

sudo chown -R bendo01:bendo01 /usr/share/nginx/html/server
sudo chmod 755 /usr/share/nginx/html/server 3. Check SELinux Context
sudo chcon -R -t httpd_sys_rw_content_t folder
bash
Copy

# View current context

ls -Z /usr/share/nginx/html/server/xsia_loco-cli

# Temporarily disable SELinux for testing

sudo setenforce 0
sudo systemctl start xsia.service

# If works, restore context permanently

sudo restorecon -v /usr/share/nginx/html/server/xsia_loco-cli 4. Manual Execution Test

bash
Copy

# Run as bendo01 user directly

sudo -u bendo01 /usr/share/nginx/html/server/xsia_loco-cli start -s 5. If using non-standard directory (/usr/share/nginx/html/server):

bash
Copy

# Create SELinux policy exception

sudo semanage fcontext -a -t bin_t "/usr/share/nginx/html/server(/.\*)?"
sudo restorecon -Rv /usr/share/nginx/html/server 6. Final Service File Fixes

ini
Copy
[Service]

# Add these critical directives

WorkingDirectory=/usr/share/nginx/html/server
Environment="APP_ENVIRONMENT=production"
Environment="RUST_BACKTRACE=1" # For debugging
After making changes:

bash
Copy
sudo systemctl daemon-reload
sudo systemctl start xsia.service
sudo systemctl status xsia.service
If still failing, check:

Binary architecture matches your OS (x86_64?)

Required glibc version matches your Alma Linux

No missing shared libraries (check with ldd xsia_loco-cli)

Filesystem isn't mounted with noexec option

To verify successful fix:

bash
Copy

# Check service status

systemctl status xsia.service

# Expected successful output:

# Active: active (running) since [timestamp]

# Main PID: [number] (xsia_loco-cli)

# Check listening port

ss -tulpn | grep 5150

## empty server nginx

```nginx
server {
      listen 80;
      listen [::]:80;
      server_name mail.your-domain.com;

      root /usr/share/nginx/html/;

      location ~ /.well-known/acme-challenge {
         allow all;
      }
}
```

postmaster@xsia.app

- URLs of installed web applications:
-
- - Roundcube webmail: https://mail.xsia.app/mail/
- - netdata (monitor): https://mail.xsia.app/netdata/
-
- - Web admin panel (iRedAdmin): https://mail.xsia.app/iredadmin/
-
- You can login to above links with below credential:
-
- - Username: postmaster@xsia.app
- - Password: B47u7453
-

/etc/letsencrypt/live/mail.xsia.app/fullchain.pem


Account Midtrans
email: bendo01@gmail.com
pass: B47u7453#

Contoh User
email: zilnuzukia@gmail.com
pass: SayaZilNuzukia


SELECT
  "id",
  "order_id",
  "account_id",
  "transaction_detailable_id",
  "transaction_detailable_type",
  "is_paid",
  "created_at",
  "updated_at",
  "deleted_at",
  "sync_at",
  "created_by",
  "updated_by",
  "gross_amount"
FROM
  "payment_midtrans"."transaction_details"
WHERE order_id LIKE '%092010 13261 ARS2402043 20251%';



UPDATE payment_midtrans.transaction_details
SET is_paid = FALSE
WHERE id = '01981c91-fe46-7d65-8c12-bd4043f0ddc8'

UPDATE payment_midtrans.transaction_details
SET is_paid = FALSE
WHERE id = '01978d87-f90f-7ef8-8f8b-f575bc58003c'

UPDATE payment_midtrans.transaction_details
SET id = '0198758c-ac5c-7673-8ca3-22d0bbcd5665'
WHERE id = '01981c91-fe46-7d65-8c12-bd4043f0ddc8'

UPDATE payment_midtrans.transaction_details
SET id = '0198758c-ac5c-75de-8eb0-d1c18d0a2604'
WHERE id = '01978d87-f90f-7ef8-8f8b-f575bc58003c'

YAAK = YKD9F9-3KT3PT-YMTH2Q-FYJRCC-H3CRR1-7Z42Y2-9RS1H7-906163-8N2K6G

BARRACUDA MAIL
Your confirmation number is BBR21755611820-33406-3405.

Username: postmaster@xsia.app
Password: B47u7453

appleID
email: bendo01@gmail.com
pass: #B47u7453