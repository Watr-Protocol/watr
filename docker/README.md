# Watr docker

Please note that this only works for mainnet.

1) Build Watr.

```shell
$ cargo build --profile production
```

2) Build the collator.

```shell
$ docker build --no-cache --target collator -t watr-collator -f docker/Dockerfile .
```

3) Build the runtime.

```shell
$ docker build --no-cache --target runtime -t watr-runtime -f docker/Dockerfile .
```

4) Build the node.

```shell
$ docker build --no-cache -t watr-node -f docker/Dockerfile .
```
