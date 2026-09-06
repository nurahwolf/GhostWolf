# Design Goal

This is a high-level objective of what exactly this project is intended to achieve.

## Multi Instance

Unlike most other bots, to learn IPC and other neat things, I am intending to be able to run 'multiple' bots in a swarm and have them work together, almost like agents. This is intended to allow for some neat things:

- Multiple bots are 'aware' of each other, and will avoid conflicting with each other where possible (i.e. They act on message deletion, such as a logger, and only one posts in a logging channel)
- It is possible to run a 'preview' version and a 'release' version independently.
- Commands can be sent between each other. For example, to restart the other bot, or to pull down a new service.
- A split between SFW and NSFW functionality.

## Multi Backend

The most featureful / performant version has a full blown DB available. For testing, a local file based DB could be used. For even more testing, just in cache / memory.

It should be able to handle and fall back gracefully when certain engines are not available.

## Modular

It should be 'easy' to enable or disable parts of the bot, without it impacting or breaking other components.
