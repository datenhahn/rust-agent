# rust-agent

:bangbang: **This is a learning project and not intended for production use.** :bangbang:

The goal of this project is to play around with rust and learn something with a semi
reallife project.

## Project Description

The agent spawns a commandline program and reads data from the program's STDOUT. It
sends the data to a REST endpoint.

* there should be two modes which can be switched via a config parameter:
  * buffered: reads STDOUT until it encounters a newline and then sends the request
  * unbuffered: immediately starts a request on incoming data and does passthrough
                streaming of the data to the rest endpoint
* in buffered mode the maximum filesize (and therefore memory use) should be specifyable
* in unbuffered mode the request should abort if data stalls for a certain time (configurable)
* also in unbuffered mode a maximum payload size should be possible to be enabled
* if the endpoint is not reachable data should be stored in a queue (up to a certain size)
* reconnects to the endpoint should happen with exponential backoff



