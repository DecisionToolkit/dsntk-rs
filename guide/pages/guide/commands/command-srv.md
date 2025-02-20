# Serving DMN™ models

The core functionality of the #DSNTK is serving <Dmn/> models.
The <Dmn/> specification precisely defines XML interchange format for decision models.
XML files containing decision models are loaded and processed by #DSNTK server and exposed
as a set of [JSON API](https://jsonapi.org) endpoints.
Each endpoint represents a single invocable defined in the decision model.
Calling an endpoint is equivalent to executing a decision, business knowledge model
or decision service.

To explain in details, how to run and use the #DSNTK server, we assume
that the built-in examples are already saved in the **~/examples** directory
(see [Saving examples](command-exs.md) for details).

Change to the directory containing the examples:

```shell
$ cd ~/examples
```

## Running a server

To run #DSNTK as a server, type the following command:

```shell
$ dsntk srv
```

Expected output should look like this:

```ansi
[32mFound 1 model.[0m
[32mLoaded 1 model.[0m
[32mDeployed 1 invocable.[0m
[34mdsntk[0m [33m0.0.0.0:22022[0m
```

#DSNTK server is started. This server accepts connections from all available network
interfaces **0.0.0.0** and listens on port **22022**. During startup, the #DSNTK server
scans the current directory with all its subdirectories, and searches for decision models
stored as XML files with **.dmn** extension.

In our example, during directory scanning, the #DSNTK server has found the **dm** directory
containing one decision model file named **dm.dmn**. This file was loaded, and one invocable was deployed,
a decision named `Greeting Message`.

To stop the #DSNTK server, press **Ctrl+C**.

The list of all deployed invocables with endpoint names can be displayed during server startup
by specifying the option `-v` or `--verbose`, like shown below:

```shell
$ dsntk srv -v
```

```ansi
[32mFound 1 model.[0m
[32mLoaded 1 model.[0m
[32mDeployed 1 invocable.[0m

[33mDeployed invocable:[0m
  [35mdm[0m/[36morg/decision-toolkit[0m/[35mgreetings[0m/[36mGreeting%20Message[0m

[34mdsntk[0m [33m0.0.0.0:22022[0m
```

## Evaluating invocables

After starting the #DSNTK server, the deployed invocable can be evaluated by calling
its endpoint with required input data, using, e.g. [**curl**](https://curl.se).
In another terminal window, type:

```shell
$ curl -s -w '\n' -d '{"Full Name":"Solomon L. Pollack"}' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/evaluate/dm/org/decision-toolkit/greetings/Greeting%20Message
```

The expected output should be:

```ansi
{"data":"Hello Solomon L. Pollack"}
```

The #DSNTK's version of a [hello world](https://en.wikipedia.org/wiki/%22Hello,_World!%22_program)
program could look like this:

```shell
$ curl -s -w '\n' -d '{"Full Name":"world"}' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/evaluate/dm/org/decision-toolkit/greetings/Greeting%20Message
```

Expected output is:

```text
{"data":"Hello world"}
```

## Endpoint names

The common endpoint for evaluating invocables exposed by the #DSNTK server
is named **`evaluate/`**.

The full URL of the endpoint is composed of the following parts:

- the protocol:

  **`http://`** or **`https://`**

- host address:

  **`0.0.0.0`** or **`127.0.0.1`** or **`my.domain.com/`** alike

- common endpoint name:

  **`evaluate/`**

- path built from directory names where the file containing the DMN™ model was found during startup scanning:

  **`dm/`**

- model namespace converted to RDNN-like path:

  **`org/decision-toolkit/`**

- model name:

  **`greetings/`**

- the name of the invocable:

  **`Greeting%20Message`**

All parts put together give the following URL of the endpoint:

```ansi
[34mhttp://127.0.0.1:22022/evaluate/dm/org/decision-toolkit/greetings/Greeting%20Message[0m
```

While not all characters are legal in URLs, there is `%20` between `Greeting` and `Message`,
which represents a space in [percent-encoding](https://en.wikipedia.org/wiki/Percent-encoding).
See [RFC3986](https://datatracker.ietf.org/doc/html/rfc3986#section-2.4) for more details.
