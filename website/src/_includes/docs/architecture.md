## Architecture (in progress)

Rome uses a server-client architecture to run its tasks.

### Daemon

A [daemon](https://en.wikipedia.org/wiki/Daemon_(computing)) is a long-running server 
that Rome spawns in the background and uses to process requests from the editor and CLI. 