# Sinkdir

## Test

This isn't easy to write unit tests for as it would require mocking system io calls. A short integration test script is included in `test.sh`.

## Install instructions

Download the latest binary from releases, and copy it to `/usr/local/bin`.

### Usage

Copy from one directory to another, also works on individual files.

`sinkdir copy <SOURCE> <TARGET>`

Delete a file or directory. This is mainly a wrapper around remove file system calls.

`simkdir delete <TARGET>`

Sync one directory with another.

`sinkdir sync <SOURCE> <TARGET>`

To run sync on a timer every minute, create systemd service files in `/etc/systemd/system`:

**sinkdir.service**

```ini
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

```ini
[Unit]
Description="Run directory sync every minute"

[Timer]
Unit=sinkdir.service
OnCalendar=minutely

[Install]
WantedBy=timers.target
```

Then run `sudo systemctl enable --now sinkdir.timer`
