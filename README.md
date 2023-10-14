# gtea

a small git utility in bash.

## usage

```
        [i]  -  init
    [clone]  -  clone
   [create]  -  create [repo]
       [wt]  -  worktree [name]
        [p]  -  push [branch]
        [s]  -  sync
       [cl]  -  changelog [N of commits|--since HASH]
       [ch]  -  cherry-pick
```

## installation

```shell
git clone --depth 1 https://github.com/crispybaccoon/gtea.git ~/gtea && cd ~/gtea
make clean all && make install
```

or using [hayashi](https://github.com/crispybaccoon/hayashi):

```
git clone --depth 1 https://github.com/crispybaccoon/gtea.git ~/gtea && cd ~/gtea
hayashi install .
```
