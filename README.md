# :olive: Olive Editor

A text editor trying its hardest to not edit text

## :shrug: What is Olive?

Olive is a new form of text editor.
While it can be configured to function like Vim or Emacs, it is designed to run external tools for as many features as possible.
This means that scripting can be kept to a minimum, while functionality is limited only by the tools available anywhere else.
As not all of the tools necessary for a text editor exist as CLI tools currently, this project also contains small tools to do those jobs.

## :stop_sign: Current State

Olive is currently pre-pre-alpha.
Nearly nothing works at the moment and it has no real function other than being a significantly worse (and limited) cat clone.
It is currently being developed alone for fun and education.

## :comapss: Roadmap

While Olive has no concrete plans, there are some ideas for the path ahead:

- Ability to issue commands that are executed asynchronously by the editor (new window, read/write file, etc)
- Running external programs to filter, format, lint, etc. text/code
- LSP implementation (automatically launch and attach when file opened)
- Abstract away the editor components, focus on orchestration (some sort of front-end agnostic protocol akin to LSP)

## :eyes: Usage

Currently... don't.

## :question: Why Olive?

I came up with many of the ideas while taking my new dog on walks (her name is Olive).
