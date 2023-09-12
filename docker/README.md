# Watr docker

Please note that this only works for mainnet.

1) Build the collator.

```shell
$ docker build --no-cache --target collator -t watr-collator -f docker/water-parachain.Dockerfile .
```

2) Build the runtime.

```shell
$ docker build --no-cache --target runtime -t watr-runtime -f docker/water-parachain.Dockerfile .
```

3) Build the node.

```shell
$ docker build --no-cache -t watr-node -f docker/water-parachain.Dockerfile .
```
