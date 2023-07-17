# Gitpod CLI

Gitpod workspaces include a command-line-utility (gp) that comes installed in all workspaces and prebuilds.

```sh
Command line interface for Gitpod

Usage:
  gp [command]

Available Commands:
  docs                Open Gitpod Documentation in default browser
  env                 Controls workspace environment variables.
  help                Help about any command
  info                Display workspace info, such as its ID, class, etc.
  init                Create a Gitpod configuration for this project.
  open                Opens a file in Gitpod
  ports               Interact with workspace ports.
  preview             Opens a URL in the IDE's preview
  snapshot            Take a snapshot of the current workspace
  ssh                 Show the SSH connection command for the current workspace
  stop                Stop current workspace
  sync-await          Awaits an event triggered using gp sync-done
  sync-done           Notifies the corresponding gp sync-await calls that this event has happened
  tasks               Interact with workspace tasks
  timeout             Interact with workspace timeout configuration
  top                 Display usage of workspace resources (CPU and memory)
  url                 Prints the URL of this workspace
  validate            [experimental] Validates the workspace (useful to debug a workspace configuration)
  version             Prints the version of the CLI

Flags:
  -h, --help   help for gp

Use "gp [command] --help" for more information about a command.
```

## init
```sh
gp init -i
```

## validate
```sh
gp validate
```

## open a file (e.g. VS Code)

```sh
gp open README.md
```

## opens a URL

```sh
gp preview $(gp url 3000) --external
```

> You can combine the preview and the url command to open a certain path instead of the default URL.

```sh
gp preview $(gp url 3000)/my/path/index.html
```

## ENV

1. Setting ENV
```sh
gp env API_ENDPOINT=https://api.example.com
```

2. Deleting ENV
```sh
gp env -u API_ENDPOINT
```
3. Show all ENV

```sh
gp env
```

## info
Displays information about the current workspace (such as the workspace ID and URL) and also the workspace class.

```sh
gp info
```

## ports

```sh
gp ports list
```

## expose

```sh
gp ports expose

// Here’s an example that will open a certain path once a service is a available:
gp ports await 3000 && gp preview $(gp url 3000)/my/path/index.html

// A port’s default visibility is always private
gp ports visibility 3000:public
gp ports visibility 3000:private

```

