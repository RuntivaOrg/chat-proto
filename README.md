# chat-proto

This project contains all proto definitions for chat that are used by (1) client apps as well as (2) internal service-to-service communications (via NATS messaging). This project also includes trait-based extensions of the NATS messages that are used to create Generic functions to minimize the code needed in the API layer of the services that consume these proto messages.

## proto structure

The proto files are structured in the following folder structure:
