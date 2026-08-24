# sinkdir

## Install instructions

Download the latest binary from releases, and copy it to `/usr/local/bin`.

Create systemd service files in `/etc/systemd/system`:

**sinkdir.service**

```toml
[Unit]
Description="Sychronise target directry with source"

[Service]
Type=oneshot
User=pierre
ExecStart=/usr/local/bin/sinkdir sync test_dir test_dir2
WorkingDirectory=/home/pierre/Projects/sinkdir/

[Install]
WantedBy=multi-user.target
```

**sinkdir.timer**

```toml
[Unit]
Description="Run directory sync every minute"

[Timer]
Unit=sinkdir.service
OnCalendar=minutely

[Install]
WantedBy=timers.target
```

Then run `sudo systemctl enable --now sinkdir.timer`
