# Simple Rust Counter App
A very simple web app built in Rust that utilises web assembly to send messages to a reactive component.

## Usage
The initial value of the counter can be set in the `main.rs` file.  
To use the default value, simply remove the property.
```html
<Counter value={5} />
<!-- or -->
<Counter value={-5} />

<!-- or use the default -->
<Counter />
```

To customise the default value, change the `CounterProps` decorator to whatever default you like.  
Alternatively, this can be changed to `prop_or_default` as that too would default to `0`.
```rust
#[derive(Properties, PartialEq)]
pub struct CounterProps {
    #[prop_or(0)]
    //        ^--- Just change this :D
    pub value: isize,
}
```

For convenience when testing, there is a `serve.sh` script that contains a command to generate a TailwindCSS file, as well as compiling and servering the Rust application to localhost.
```powershell
$ sh serve.sh
```
