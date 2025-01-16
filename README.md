# JQuery SmartModule (jaq)

SmartModule for processing json record using jq syntax. The smartmodule is based on the rust version of jq named [jaq](https://github.com/01mf02/jaq/tree/main/).

## Playground

Use the [jaq playground](https://gedenkt.at/jaq/) to test Jq expressions before using them in a SmartModule.


## Download and Test the SmartModule

Download a pre-compiled version of this SmartModule from the InfinyOn Hub, create a topic, and to run the tests.

```bash
fluvio hub smartmodule download infinyon/jaq@0.1.0
```

Create a topic called `jaq`:

```bash
fluvio topic create jaq
```

Run the tests:

### Test - Array 1

* Produce: 

  ```bash
  echo '["zero", "one", "two"]' | fluvio produce jaq
  ```

* Consume:

  ```bash
  fluvio consume jaq -dT=1 --smartmodule infinyon/jaq@0.1.0 -e filter=".[]"  
  ```

* Result:

  ```bash
  ["zero","one","two"]
  ```

### Test - Array 2

* Consume:

  ```bash
  fluvio consume jaq -dT=1 --smartmodule infinyon/jaq@0.1.0 -e filter=".[1]"  
  ```

* Result:

  ```bash
  "one"
  ```

### Test - Array 3

* Consume:

  ```bash
  fluvio consume jaq -dT=1 --smartmodule infinyon/jaq@0.1.0 -e filter=".[7]"  
  ```

* Result:

  ```bash
  null
  ```

### Test - Math 1

Numbers are computed:

* Produce: 

  ```bash
  echo '{"x": 5, "y": 10}' | fluvio produce jaq
  ```

* Consume:

  ```bash
  fluvio consume jaq -dT=1 --smartmodule infinyon/jaq@0.1.0 -e filter=".x + .y"  
  ```

* Result:

  ```bash
  15
  ```
  
### Test - Math 2

* Consume:

  ```bash
  fluvio consume jaq -dT=1 --smartmodule infinyon/jaq@0.1.0 -e filter=".x * .y"  
  ```

* Result:

  ```bash
  50
  ```
  
### Test - Concatenation

Arrays are concatenated.

* Produce: 

  ```bash
  echo '{"x": [1, 2], "y": [3, 4]}' | fluvio produce jaq
  ```

* Consume:

  ```bash
  fluvio consume jaq -dT=1 --smartmodule infinyon/jaq@0.1.0 -e filter=".x + .y"  
  ```

* Result:

  ```bash
  [1,2,3,4]
  ```

### Test - Join

Strings are joined together.

* Produce: 

  ```bash
  echo '{"x": "foo", "y": "bar"}' | fluvio produce jaq
  ```

* Consume:

  ```bash
  fluvio consume jaq -dT=1 --smartmodule infinyon/jaq@0.1.0 -e filter=".x + .y"  
  ```

* Result:

  ```bash
  "foobar"
  ```
  
### Test - Merge

Objects are merged together.

* Produce: 

  ```bash
  echo '{"x": {"foo": "bar"}, "y": {"hello": "world"}}' | fluvio produce jaq
  ```

* Consume:

  ```bash
  fluvio consume jaq -dT=1 --smartmodule infinyon/jaq@0.1.0 -e filter=".x + .y"  
  ```

* Result:

  ```bash
  {"foo":"bar","hello":"world"}
  ```
 
  
### Test - Filtering

We'll use [cars.json](./test-data/cars.json) data set to test filtering.

* Produce: 

  ```bash
  fluvio produce jaq -f test-data/cars.json --raw
  ```

* Consume:

  ```bash
  fluvio consume jaq -dT=1 --smartmodule infinyon/jaq@0.1.0 -e filter='.[] | select(.Year == "1972-01-01")' -O json
  ```

* Result:

  ```bash
  [
    {
      "Acceleration": 16.5,
      "Cylinders": 4,
      "Displacement": 97,
      "Horsepower": 88,
      "Miles_per_Gallon": 27,
      "Name": "toyota corolla 1600 (sw)",
      "Origin": "Japan",
      "Weight_in_lbs": 2100,
      "Year": "1972-01-01"
    }
    ...
  ]
  ```

### Test - Mapping

Map from one object to another.

* Consume:

  ```bash
  fluvio consume jaq -dT=1 --smartmodule infinyon/jaq@0.1.0 -e filter='[ .[] | select(.Year == "1972-01-01") | {name: .Name, performance: {horsepower: .Horsepower, acceleration: .Acceleration, cylinders: .Cylinders, displacement: .Displacement}, efficiency: {milesPerGallon: .Miles_per_Gallon, weightInLbs: .Weight_in_lbs}, build: {year: .Year, origin: .Origin}} ]' -O json
  ```
* Result:

  ```bash
  [
    {
      "build": {
        "origin": "Japan",
        "year": "1972-01-01"
      },
      "efficiency": {
        "milesPerGallon": 27,
        "weightInLbs": 2100
      },
      "name": "toyota corolla 1600 (sw)",
      "performance": {
        "acceleration": 16.5,
        "cylinders": 4,
        "displacement": 97,
        "horsepower": 88
      }
    }
    ...
  ]
  ```

## Example of usage in Connector

Smartmodule can be used in a connector.

Download Connector from the Hub:

```
cdk hub download infinyon/http-source@0.4.3
```

Create a connector yaml file named `quotes.yaml`:

```
apiVersion: 0.1.0
meta:
  version: 0.4.3
  name: quotes-connector
  type: http-source
  topic: quotes

http:
  endpoint: https://demo-data.infinyon.com/api/quote
  interval: 10s

transforms:
  - uses: infinyon/jaq@0.1.0
    with:
      filter: ".quote"
```

Run the connector in the cloud

```bash
fluvio cloud connector create --config quotes.yaml
```

Or use `cdk` to deploy locally.


## For Developer

Compile and test the smartmodule using the `smdk` tool.

### Compile Smartmodule

Compile the smartmodule:

```bash
smdk build
```

### Test SmaartModule

Use [test-data/fruit-input.json](test-data/fruit-input.json) test dataset.

To get a simple fruit:

```bash
$ smdk test --file test-data/fruit-input.json --raw -e filter=.fruit
{"name":"apple","color":"green","price":1.2}
```

With pipeline:

```bash
$ smdk test --file test-data/creatures-input.json --raw -e filter=".[] | .name"
["Sammy","Bubbles","Splish","Splash"]
```