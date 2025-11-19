# How to Enable Service Binary on Linux

## Create Service

path
```sh
/etc/systemd/system/xsia.service
```

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
## Verify Execute Permissions & Ownership


### Check permissions (must have -rwxr-xr-x)

```sh
ls -l /usr/share/nginx/html/server/xsia_loco-cli
```

### Add execute permissions if missing

```sh
sudo chmod +x /usr/share/nginx/html/server/xsia_loco-cli
```

### Set proper ownership (user bendo01)

```sh
sudo chown bendo01:bendo01 /usr/share/nginx/html/server/xsia_loco-cli
```

## Verify Directory Permissions

### Ensure user bendo01 has access to the entire path

```sh
sudo chown -R bendo01:bendo01 /usr/share/nginx/html/server
```

```sh
sudo chmod 755 /usr/share/nginx/html/server
```

### Check SELinux Context

```sh
sudo chcon -R -t httpd_sys_rw_content_t folder
```

### View current context

```sh
ls -Z /usr/share/nginx/html/server/xsia_loco-cli
```

### Temporarily disable SELinux for testing

```sh
sudo setenforce 0
```

```sh
sudo systemctl start xsia.service
```

### If works, restore context permanently

```sh
sudo restorecon -v /usr/share/nginx/html/server/xsia_loco-cli
```

### Manual Execution Test

### Run as bendo01 user directly

```sh
sudo -u bendo01 /usr/share/nginx/html/server/xsia_loco-cli start -s
```

## If using non-standard directory (/usr/share/nginx/html/server):

### Create SELinux policy exception

```sh
sudo semanage fcontext -a -t bin_t "/usr/share/nginx/html/server(/.\*)?"
```

```sh
sudo restorecon -Rv /usr/share/nginx/html/server
```

### Final Service File Fixes

#### Add these critical directives
```sh
WorkingDirectory=/usr/share/nginx/html/server
Environment="APP_ENVIRONMENT=production"
Environment="RUST_BACKTRACE=1" # For debugging
```

#### After making changes:

```sh
sudo systemctl daemon-reload
```

```sh
sudo systemctl start xsia.service
```

```sh
sudo systemctl status xsia.service
```

If still failing, check:

Binary architecture matches your OS (x86_64?)

Required glibc version matches your Alma Linux

No missing shared libraries (check with ldd xsia_loco-cli)

Filesystem isn't mounted with noexec option

To verify successful fix:



### Check service status

```sh
systemctl status xsia.service
```

### Expected successful output:

### Active: active (running) since [timestamp]

### Main PID: [number] (xsia_loco-cli)

### Check listening port

```sh
ss -tulpn | grep 5150
```

```sh
sudo chcon -t bin_t /var/www/html/gis_project/gisapp/gisapp-cli
```

```sh
sudo chcon -R -t usr_t /var/www/html/gis_project/gisapp
```

```sh
sudo systemctl daemon-reload
sudo systemctl restart gis_project
```

```sh
sudo semanage fcontext -a -t bin_t "/var/www/html/gis_project/gisapp/gisapp-cli"
sudo semanage fcontext -a -t usr_t "/var/www/html/gis_project/gisapp(/.*)?"
```

```sh
sudo restorecon -Rv /var/www/html/gis_project/
```

```sh
sudo setenforce 0
sudo systemctl restart gis_project
```

```sh
sudo setenforce 1
```
