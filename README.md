<p align="center"><a href="https://github.com/Dentrax/kubesql" target="_blank"><img height="128" src="https://raw.githubusercontent.com/Dentrax/kubesql/main/.res/logo.png"></a></p>

<h1 align="center">kubesql</h1>

<div align="center">
 <strong>
   An experimental tool for querying your Kubernetes API Server using SQL
 </strong>
</div>

<br />

<p align="center">
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square" alt="MIT"></a>
  <a href="https://github.com/Dentrax/kubesql/releases/latest"><img src="https://img.shields.io/github/release/Dentrax/kubesql.svg?style=flat-square" alt="GitHub release"></a>
</p>

<br />

*kubesql*, an experimental tool for querying your Kubernetes API Server using simple and smallest SQL syntax.

```bash
$ kubesql -q "SELECT namespace FROM context WHERE pod.status.phase = 'Running'"
```
![Screenshot](.res/screenshot.png)

# Installation

## Docker
```bash
$ docker pull furkanturkal/kubesql:0.1.0
```

## From Source
```bash
$ cargo install --path . # local
# - or -
$ cargo install --git https://github.com/Dentrax/kubesql # remote
```

# Usage

[![asciicast](https://asciinema.org/a/407398.svg)](https://asciinema.org/a/407398)

```bash
USAGE:
    kubesql [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <FILE>      
    -q, --query <query>
```

## Evaluate
```bash
$ kubesql --file ./kube.sql
$ kubesql --query "SELECT namespace FROM context WHERE pod.status.phase = 'Running'"
```

### With Docker
```bash
$ docker container run -v ~/.kube/config/:/home/nonroot/.kube/config kubesql:0.1.0 --query "SELECT namespace FROM context WHERE pod.status.phase = 'Running'"
```

# Quick Start

## SQL Syntax

### Single Context
```sql
SELECT namespace
FROM context
WHERE pod.status.phase = 'Running'
```

### Multiple Context-Namespace
```sql
SELECT namespace1, namespace2
FROM context1, context2
WHERE pod.status.phase = 'Running' AND deployment.metadata.name = 'my-awesome-deployment'
```

### Supported Statements
| STATEMENT | REQUIRED |
|-----------|----------|
| SELECT    | ✓        |
| FROM      | ✓        |
| WHERE     | ✓        |

### Supported Operators
| OPERATOR | WHERE | ACTION                   |
|----------|-------| ------------------------ |
| AND      | ✓     | Panic if no result found |
| OR       | ✓     | Continue                 |

# Special Thanks

| Package                                                       | Author                                                  | License                                                                                      |
| :------------------------------------------------------------ | :------------------------------------------------------ | :------------------------------------------------------------------------------------------- |
| [sqlparser](https://github.com/ballista-compute/sqlparser-rs) | [ballista-compute](https://github.com/ballista-compute) | [Apache License 2.0](https://github.com/ballista-compute/sqlparser-rs/blob/main/LICENSE.TXT) |
| [kube](https://github.com/clux/kube-rs)                       | [clux](https://github.com/clux)                         | [Apache License 2.0](https://github.com/clux/kube-rs/blob/master/LICENSE)                    |
| [prettytable](https://github.com/phsym/prettytable-rs)        | [phsym](https://github.com/phsym)                       | [MIT](https://github.com/phsym/prettytable-rs/blob/master/LICENSE.txt)                       |

- Thanks to everyone who contributed these libraries and [others](https://github.com/Dentrax/kubesql/blob/master/Cargo.toml) that made this project possible.

# License

*kubesql* was created by Furkan 'Dentrax' Türkal

The base project code is licensed under [MIT](https://opensource.org/licenses/MIT) unless otherwise specified. Please see the **[LICENSE](https://github.com/Dentrax/kubesql/blob/master/LICENSE)** file for more information.

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FDentrax%2Fkubesql.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2FDentrax%2Fkubesql?ref=badge_large)

<kbd>Best Regards</kbd>
