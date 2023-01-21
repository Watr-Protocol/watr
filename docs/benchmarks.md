# Benchmarks
Weights are generated in a machine with the minimum specs required by a Watr collator: **4 vCPUs @ 3.1GHz | 16 GiB memory**

1. Connect via ssh to your benchmarking machine
    - `ssh <user>@<machine_ip>`
2. Make sure no one else is using the machine with `htop check`
3. If it is the first time you run the benchmarks, you will need to clone the repository
    - `git clone https://github.com/Watr-Protocol/watr.git`
4. Checkout to the branch to benchmark (usually `main`)
    - `git checkout main`
5. Compile `cargo build --release --features runtime-benchmarks`
6. From the root directory run `nohup ./scripts/benchmarks.sh &` (it will take quite a few hours)
7. To spot some possible issues, the `nohup.out` file can be checked from time to time:
    - `tail -f 100 nohup.out`
8. In your local machine, checkout to `main` when the benchmark is finished and you got the new weight files.
9. `scp` the weights from the benchmarking machine to your local machine for `devnet` and `mainnet`:
    - `/runtime/devnet/src/weights`
    - `/runtime/devnet/src/weights`

    Example:

    ```bash
    scp -r <user><machine_ip>:/home/<user>/watr/runtime/devnet/src/weights <absolute_path_to_watr_repo>/runtime/devnet/src
    scp -r <user><machine_ip>:/home/<user>/watr/runtime/mainnet/src/weights <absolute_path_to_watr_repo>/runtime/mainnet/src
    ```

10. Commit the changes in your local and create a PR
11. Review the PR and make sure all the new generated weights make sense (not too high/low compared to previous weights, 15% variance is acceptable)
