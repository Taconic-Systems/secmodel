# Taconic Security Model

This project provides a data model for describing the security
relevant features of Information Technology systems, and tools for
generating diagrams and reports from those models.  It is intended to
be used for security architecture, auditing, and compliance tasks.

The project is Rust workspace containing the following crates:

* secmodel -- a CLI frontend
* secmodel_core -- a library providing the domain model
* secmodel_md -- library for generating markdown reports
* sedmodel_mermaid -- library for generating Mermaid diagrams

The model is defined in a [TOML File](test/test_model.toml) and an be processed with the `secmodel` CLI tool.

For example, a small model of a web based medical billing application:

```toml
[network.prod]
title = "Production"
description = """
Production Network
"""
ipv4_ranges = ["192.168.1.0/24"]

[server.prod]
interfaces = [{network="network.prod"}]
applications = ["application.mainapp"]
stores = ["store.main-mysql", "store.prod-fs"]

[application.mainapp]
title = "MainApp"
description = """
A multi-tenant PHP Based Medical Billing System
"""

[flow.mainapp-client]
sources = ["agent.client"]
destinations = ["application.mainapp"]
data = ["data.login-credentials", "data.health-info"]
channel = "channel.https"

[data.health-info]
classification = "high-risk"
regulations = ["regulation.phi", "regulation.pii"]

[data.health-info]
regulations = ["regulation.phi", "regulation.pii"]
classification = "high-risk"

[data.login-credentials]
description = "Passwords used by clients and employees to login to main app"
classification = "confidential"

[store.main-mysql]
data = ["data.health-info", "data.login-credentials", "data.health-metadata"]
backing = "store.prod-fs"

[store.prod-fs]
data = ["data.health-info", "data.ssh-credentials"]

[channel.https]
ports = [443]
protocols = ["https"]
encryption = "tls"

[channel.https]
ports = [443]
protocols = ["protocol.https"]
encryption = "encryption.tls"

```

# Usage

To produce a security architecture report:
```sh
cargo run -- -m mymodel.toml report > report.md
```

Subsequently, you can use the mermaid CLI,`mmdc`, and `pandoc` to
generate a PDF:

```sh
mmdc -i report.md -e pdf -o out.md && pandoc --toc out.md -o report.pdf
```


To create just the overview diagram:

```sh
cargo run -- -m mymodel.toml diagram > diagram.m
```

That can be processes with the mermaid CLI into an svg,  this will produce a `diagram.m.svg` in the current directory:

```sh
./node_modules/.bin/mmdc -i diagram.m -b black
```


