---
outline: 'deep'
---

# Command line

## Command cheat sheet

|       Action  |         DECISION<br/>MODEL         |         DECISION<br/>TABLE         |        FEEL<br/>EXPRESSION         |              EXAMPLES              |
|--------------:|:----------------------------------:|:----------------------------------:|:----------------------------------:|:----------------------------------:|
|  **EVALUATE** | [**edm**](commands/command-edm.md) | [**edt**](commands/command-edt.md) | [**efe**](commands/command-efe.md) |                                    | 
|     **PARSE** | [**pdm**](commands/command-pdm.md) | [**pdt**](commands/command-pdt.md) | [**pfe**](commands/command-pfe.md) |                                    |
|      **TEST** | [**tdm**](commands/command-tdm.md) | [**tdt**](commands/command-tdt.md) | [**tfe**](commands/command-tfe.md) |                                    |
|    **EXPORT** | [**xdm**](commands/command-xdm.md) | [**xdt**](commands/command-xdt.md) | [**xfe**](commands/command-xfe.md) |                                    |
| **RECOGNIZE** |                                    | [**rdt**](commands/command-rdt.md) |                                    |                                    |
|     **SERVE** | [**srv**](commands/command-srv.md) |                                    |                                    |                                    |
|      **SAVE** |                                    |                                    |                                    | [**exs**](commands/command-exs.md) |

## Commands

Below is a categorized list of all #DSNTK commands.

### SERVE

#### srv

Runs #DSNTK as a service and serves DMN™ decision models.

To display all available options of the [**srv**](commands/command-srv) command, type:

```shell
$ dsntk help srv
```

For more details, refer to [Serving DMN™ models](commands/command-srv).

### EVALUATE

#### edm

Evaluates DMN™ decision model.

To display all available options of the [**edm**](commands/command-edm) command, type:

```shell
$ dsntk help edm
```

For more details, refer to [Evaluating DMN™ models](commands/command-edm).

#### edt

Evaluates decision table.

To display all available options of the [**edt**](commands/command-edt) command, type:

```shell
$ dsntk help edt
```

For more details, refer to [Evaluating decision tables](commands/command-edt).

#### efe

Evaluates FEEL expression.

To display all available options of the [**efe**](commands/command-efe) command, enter:

```shell
$ dsntk help efe
```

For more details, refer to [Evaluating FEEL expressions](commands/command-efe).

### PARSE

#### pdm

Parses DMN™ model.

To display all available options of the [**pdm**](commands/command-pdm) command, enter:

```shell
$ dsntk help pdm
```

For more details, refer to [Parsing DMN™ models](commands/command-pdm).

#### pdt

Parses decision table.

To display all available options of the [**pdt**](commands/command-pdt) command, enter:

```shell
$ dsntk help pdt
```

For more details, refer to [Parsing decision tables](commands/command-pdt).

#### pfe

Parses FEEL expression.

To display all available options of the [**pfe**](commands/command-pfe) command, enter:

```shell
$ dsntk help pfe
```

For more details, refer to [Parsing FEEL expressions](commands/command-pfe).

### TEST

#### tdm

Tests DMN™ model.

To display all available options of the [**tdm**](commands/command-tdm) command, enter:

```shell
$ dsntk help tdm
```

For more details, refer to [Testing DMN™ models](commands/command-tdm).

#### tdt

Tests decision table.

To display all available options of the [**tdt**](commands/command-tdt) command, enter:

```shell
$ dsntk help tdt
```

For more details, refer to [Testing decision tables](commands/command-tdt).

#### tfe

Tests FEEL expression.

To display all available options of the [**tfe**](commands/command-tfe) command, enter:

```shell
$ dsntk help tfe
```

For more details, refer to [Testing FEEL expressions](commands/command-tfe).

### EXPORT

#### xdm

Exports DMN™ model.

To display all available options of the [**xdm**](commands/command-xdm) command, enter:

```shell
$ dsntk help xdm
```

For more details, refer to [Exporting DMN™ models](commands/command-xdm).

#### xdt

Exports decision table.

To display all available options of the [**xdt**](commands/command-xdt) command, enter:

```shell
$ dsntk help xdt
```

For more details, refer to [Exporting decision tables](commands/command-xdt).

#### xfe

Exports FEEL expression.

To display all available options of the [**xfe**](commands/command-xfe) command, enter:

```shell
$ dsntk help xfe
```

For more details, refer to [Exporting FEEL expressions](commands/command-xfe).

### RECOGNIZE

#### rdt

Recognizes decision table.

To display all available options of the [**rdt**](commands/command-rdt) command, enter:

```shell
$ dsntk help rdt
```

For more details, refer to [Recognizing decision tables](commands/command-rdt).

### SAVE

#### exs

Saves the examples.

To display all available options of the [**exs**](commands/command-exs) command, enter:

```shell
$ dsntk help exs
```

For more details, refer to [Saving examples](commands/command-exs).

## Options

Below is a list of all #DSNTK options.

### --help

Print help.

```shell
$ dsntk --help
```

or (short)

```shell
$ dsntk -h
```

### --version

Print version.

```shell
$ dsntk --version
```

or (short)

```shell
$ dsntk -V
```
