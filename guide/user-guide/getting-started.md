# Getting started

This is a quick overview of the basic functionalities of the <DsntkName/>, using copy and paste.
The examples are brief and designed to help you quickly become familiar with the <DsntkName/>.
For detailed explanations, refer to the chapters cited below each example.

## Install examples

&#8203;<DsntkName/> provides a set of built-in examples ready to play with.
To install the examples in, e.g., the **examples** subdirectory in your home directory, type:

```shell 
$ dsntk exs ~/examples
```

For more details, refer to the chapter [Saving examples](commands/command-exs.md).

## Serve DMN™ model

Change to the directory containing the decision model example: 

```shell
$ cd ~/examples/dm
```

Start the server  (**srv** command):

```shell
$ dsntk srv -v -H 127.0.0.1 -D .
```

Expected output:

```ansi
[32mFound 1 model.[0m
[32mLoaded 1 model.[0m
[32mDeployed 1 invocable.[0m
[33m
Deployed invocables:[0m
  [36morg/decision-toolkit[0m/[35mgreetings[0m/[36mGreeting%20Message[0m

[34mdsntk[0m [33m127.0.0.1:22022[0m
```

Switch to another terminal and type:

```shell
$ curl -s -d '{"Full Name": "Solomon L. Pollack"}' -H 'Content-Type: application/json' -X POST http://127.0.0.1:22022/evaluate/org/decision-toolkit/greetings/Greeting%20Message
```

Expected output:

```ansi
{"data":"Hello Solomon L. Pollack"}
```

For more details, refer to the chapter [Serving DMN™ models](commands/command-srv.md).

## Evaluate DMN™ model

Again, change to the directory containing the decision model example:

```shell
$ cd ~/examples/dm
```

Evaluate the decision model (**edm** command):

```shell
$ dsntk edm dm.ctx dm.dmn -i "Greeting Message"
```

Expected result:

```ansi
"Hello Solomon L. Pollack"
```

For more details, refer to the chapter [Evaluating DMN™ models](commands/command-edm.md).

## Evaluate decision table

Change to the directory containing the decision table example:

```shell
$ cd ~/examples/dt
```

Evaluate the decision table (**edt** command):

```shell
$ dsntk edt dt.ctx dt.dtb
```

Expected result:

```ansi
0.15
```

For more details, refer to the chapter [Evaluating decision tables](commands/command-edt.md).

## Evaluate FEEL expression

Change to the directory containing the FEEL expression example:

```shell
$ cd ~/examples/fe
```

Evaluate FEEL expression (**efe** command):

```shell
$ dsntk efe fe.ctx fe.feel
```

Expected result:

```ansi
0.3
```

For more details, refer to the chapter [Evaluating FEEL expressions](commands/command-efe.md).
