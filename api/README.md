
The project refactor involves splitting the hard to work with and messy serialization backend from the outside world.
Essentially, the new API is a symbiote that wraps around the serialization backend.

The backend/internal code is set to only be accessible by the API crate, so that dependents don't have to use the mess that it is.

tldr. the API crate is an abstraction layer that makes the project easier to work with.
