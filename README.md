# fluvio-jq

SmartModule for processing jq.  This uses [jq](https://github.com/01mf02/jaq/tree/main/) to process JSON data.

## Playground

You can use the [jaq playground](https://gedenkt.at/jaq/) to test Jq expression.

## Usage

To get a simple fruit:

```bash
smdk test --file test-data/fruit-input.json --raw -e filter=.fruit
{"name":"apple","color":"green","price":1.2}
```

With pipeline:

```bash
smdk test --file test-data/creatures-input.json --raw -e filter=".[] | .name"
["Sammy","Bubbles","Splish","Splash"]
```