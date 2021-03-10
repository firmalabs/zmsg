# cardano-systemd

A tool for starting, stopping, and debugging systemd services for Cardano node.

## Executables

You need the executables in the `PATH` (in SELinux like Amazon Linux 2 they should be placed in `/usr/local/sbin`).

Examples in Bash script:

 ```bash

# relay-init.sh

#!/usr/bin/bash

node_path="/home/ec2-user/testnet.relay"
priv_ip=$(hostname -I)
port=3001

cardano-node run \
 --topology $node_path/testnet-topology.json \
 --database-path $node_path/db \
 --socket-path $node_path/db/node.socket \
 --host-addr $priv_ip \
 --port $port \
 --config $node_path/testnet-config.json

 ```

```bash

# prometheus-init.sh

#!/usr/bin/bash

node_path="/home/ec2-user/testnet.relay"

prometheus --config.file=$node_path/prometheus.yml

```

```bash

# node-exporter-init.sh

#!/usr/bin/bash

node_exporter

```

# Systemd Unit Files

Service files should be placed in `/etc/systemd/system` with strict names:

```systemd

# node-exporter.service

[Unit]
Description=Node Exporter
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
Environment=PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/home/ec$
ExecStart=/usr/local/sbin/node-exporter-init.sh
Restart=always
RestartSec=0
KillMode=process

[Install]
WantedBy=multi-user.target

```

```systemd

# prometheus.service

[Unit]
Description=Prometheus
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
Environment=PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/home/ec\
2-user/.local/bin
ExecStart=/usr/local/sbin/prometheus-init.sh
Restart=always
RestartSec=0
KillMode=process

[Install]
WantedBy=multi-user.target

```

```systemd

# cardano-node.service

[Unit]
Description=Cardano Node
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
Environment=PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/home/ec\
2-user/.local/bin
Environment=LD_LIBRARY_PATH=/usr/local/lib
Environment=PKG_CONFIG_PATH=/usr/local/lib/pkgconfig

ExecStart=/usr/local/sbin/relay-init.sh
Restart=always
RestartSec=0
KillMode=process

[Install]
WantedBy=multi-user.target

```

