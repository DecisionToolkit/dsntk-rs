---
outline: 'deep'
---

# Command line

## Command Cheat Scheat

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

Below is a categorized list of all <DsntkName/> commands.

### SERVE

#### srv

Runs <DsntkName/> as a service and serves DMN™ decision models.

To display all available options of the [**srv**](commands/command-srv.md) command, type:
 
```shell
$ dsntk help srv
```
 
For more details, refer to [Serving DMN™ models](commands/command-srv.md).

### EVALUATE

#### edm

Evaluates DMN™ decision model.

To display all available options of the [**edm**](commands/command-edm.md) command, type:

```shell
$ dsntk help edm
```

For more details, refer to [Evaluating DMN™ models](commands/command-edm.md).

#### edt

Evaluates decision table.

To display all available options of the **edt** command, type:

```shell
$ dsntk help edt
```

For more details, refer to [Evaluating decision tables](commands/command-edt.md).

#### efe

Evaluates FEEL expression.

To display all available options of the [**efe**](commands/command-efe.md) command, enter:

```shell
$ dsntk help efe
```

For more details, refer to [Evaluating FEEL expressions](commands/command-efe.md).

### PARSE

#### pdm

Parses DMN™ model.

To display all available options of the [**pdm**](commands/command-pdm.md) command, enter:

```shell
$ dsntk help pdm
```

For more details, refer to [Parsing DMN™ models](commands/command-pdm.md).

#### pdt

Parses decision table.

To display all available options of the [**pdt**](commands/command-pdt.md) command, enter:

```shell
$ dsntk help pdt
```

For more details, refer to [Parsing decision tables](commands/command-pdt.md).

#### pfe

Parses FEEL expression.

To display all available options of the [**pfe**](commands/command-pfe.md) command, enter:

```shell
$ dsntk help pfe
```

For more details, refer to [Parsing FEEL expressions](commands/command-pfe.md).

### TEST

#### tdm

Tests DMN™ model.

To display all available options of the [**tdm**](commands/command-tdm.md) command, enter:

```shell
$ dsntk help tdm
```

For more details, refer to [Testing DMN™ models](commands/command-tdm.md).

#### tdt

Tests decision table.

To display all available options of the [**tdt**](commands/command-tdt.md) command, enter:

```shell
$ dsntk help tdt
```

For more details, refer to [Testing decision tables](commands/command-tdt.md).

#### tfe

Tests FEEL expression.

To display all available options of the [**tfe**](commands/command-tfe.md) command, enter:

```shell
$ dsntk help tfe
```

For more details, refer to [Testing FEEL expressions](commands/command-tfe.md).

### EXPORT

#### xdm

Exports DMN™ model.

To display all available options of the [**xdm**](commands/command-xdm.md) command, enter:

```shell
$ dsntk help xdm
```

For more details, refer to [Exporting DMN™ models](commands/command-xdm.md).

#### xdt

Exports decision table.

To display all available options of the [**xdt**](commands/command-xdt.md) command, enter:

```shell
$ dsntk help xdt
```

For more details, refer to [Exporting decision tables](commands/command-xdt.md).

#### xfe

Exports FEEL expression.

To display all available options of the [**xfe**](commands/command-xfe.md) command, enter:

```shell
$ dsntk help xfe
```

For more details, refer to [Exporting FEEL expressions](commands/command-xfe.md).

### RECOGNIZE

#### rdt

Recognizes decision table.

To display all available options of the [**rdt**](commands/command-rdt.md) command, enter:

```shell
$ dsntk help rdt
```

For more details, refer to [Recognizing decision tables](commands/command-rdt.md).

### SAVE

#### exs

Saves the examples.

To display all available options of the [**exs**](commands/command-exs.md) command, enter:

```shell
$ dsntk help exs
```

For more details, refer to [Saving examples](commands/command-exs.md).

## Options

Below is a list of all <DsntkName/> options.

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
