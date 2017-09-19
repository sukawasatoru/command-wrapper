Command Wrapper
===============

```sh
#!/bin/bash

java -jar $(dirname $0)/cfr_0_115.jar $@
```

:arrow_down:

```toml
path = "java"
default_args = ["-jar", "{exe_dir}/cfr_0_115.jar"]
```
