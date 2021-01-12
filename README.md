**This software requested and paid for by www.Nerdit.uk**

This code is an example project for querying playback state from a Mopidy server.  
(code for dealing with API in src/mopidy.rs)

This program queries the Mopidy server at 'MOPIDY_ADDR' and displays the result.

Developed using rust 1.49.0

# Build
`cargo build`

# Run
`cargo run`

Should see output similar to:
```
Making Mopidy RPC request to [http://localhost:6680/mopidy/rpc]...
Done.
Playback State: stopped
```