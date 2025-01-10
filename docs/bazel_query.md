# Bazel Query Guide 

## Package dependency: Which targets do my crate depends on?

```shell
bazel query --noimplicit_deps "deps(//alias:all_data_integration)" --output package | grep queng_
```

Here we set `--noimplicit_deps` to avoid the implicit dependencies of the `//alias:all_data_integration` target.
And we set the output to `package` to avoid the toolchains or other rules of the `//alias:all_data_integration` target in the output.
Lastly, we grep the `queng_` package name to only capture packages in the `queng_` namespace.

## Reverse dependency: Which targets depend on my crate?


```shell
bazel query --infer_universe_scope --order_output=auto "allrdeps(//alias:trait_data_integration)"
 ```

Here, the trait_data_integration is the name of the crate we want to find the reverse dependencies i.e.
find all other targets that import (and therefore likely implement) the trait_data_integration crate.
The remaining parameters are set to default values.


## Reference:

[Query guide](https://bazel.build/query/guide)

[The Bazel Query Reference](https://bazel.build/query/language#deps)

[Deep dive into Bazel queries](https://virtuslab.com/blog/backend/bazel-queries-with-examples/)