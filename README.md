#atelier-rest-api

This is a simple REST API implemented in Rust/nickel.rs that sits on top of
atelier. It's main usage is to save and restore filesets for kanvaz.

##Getting started

1. Getting to the precious bits

The server is written in Rust. Since Rust is a compiled language one has to have
the right binary for the specific platform to run it.

So how do you get to the binary? If you are a Rust user, things are quite simple.
Just clone this repository and run `cargo build` to compile the server.

If you didn't touch Rust yet but are on a Mac you can just grab the latest binary
for Mac from here.

1. Start the server

Starting the server is as simple as calling the binary. If you compiled it yourself
with Cargo you'll find it here: `./target/debug/atelier-rest-api`.

##API usage

The server currently supports three API calls to allow saving and retrieval of
kanvaz filesets.

The JSON structure for a kanvaz is as follows.

```rust
{
    "files": [{
        "name":"style.css",
        "content": "button: { color: red; }"
    }]
}
```

The `files` property is a simple array that holds object that map the contents of a file
to a name. The number of files is unlimited. It's currently not supported to use directories
so a `name` like `"/some/directory/style.css"` will probably not work.

###Storing a kanvaz

To store a kanvaz make a `POST` request to `/kanvaz`.

Example:

`curl 'http://localhost:6767/kanvaz' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "files": [{ "name":"style.css", "content": "button: { color: red; }"}] }'`

The call will return a repository object that looks like this:

```
{
    "id": "38c398e8bc6dfb94360eb3623fd5b34daf7f776c",
    "path": "TEMP_REP_38c398e8bc6dfb94360eb3623fd5b34daf7f776c"
}
```

###Updating a kanvaz

To update a kanvaz make a `PUT` request to `/kanvaz/:id`.

Example:

`curl 'http://localhost:6767/kanvaz/38c398e8bc6dfb94360eb3623fd5b34daf7f776c' -X PUT -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "files": [{ "name":"style.css", "content": "button: { color: red; }"}] }'`

The call will return a repository object that looks like this:

```
{
    "id": "38c398e8bc6dfb94360eb3623fd5b34daf7f776c",
    "path": "TEMP_REP_38c398e8bc6dfb94360eb3623fd5b34daf7f776c"
}
```

###Fetching a kanvaz

To fetch an existing kanvaz make a `GET` request to `/kanvaz/:id`.

Example:

`curl 'http://localhost:6767/kanvaz/38c398e8bc6dfb94360eb3623fd5b34daf7f776c'`

The call will return a repository object that looks like this:

```
{
    "id": "38c398e8bc6dfb94360eb3623fd5b34daf7f776c",
    "path": "TEMP_REP_38c398e8bc6dfb94360eb3623fd5b34daf7f776c"
}
```

##Plans

###Fetching a kanvaz at a specific revision

It's currently not supported to fetch a kanvaz at a specific revision. In order to
support that we'll probably change the `repository` model to include a `revision`
property so that it looks like this.

```
{
    "id": "38c398e8bc6dfb94360eb3623fd5b34daf7f776c",
    "path": "TEMP_REP_38c398e8bc6dfb94360eb3623fd5b34daf7f776c",
    "revision": "d020ae4a533a755945d828bedcc1341fcf73acd5"
}
```

We could then introduce a new `GET` API `/kanvaz/:id/:revision`
