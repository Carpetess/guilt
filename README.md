### What is this project?

A very barebones version of git that only has a few operations that git supports, all from the [Plumbing and Porcelain](https://git-scm.com/book/en/v2/Git-Internals-Plumbing-and-Porcelain) chapter of the Git Book:

- git init
- git cat-file
- git hash-object
- git update-index
- git write-tree

This basically gives you enough tools to version manage a repository, but it's very unpractical, since all the commands have to be written by hand.
I don't think this project will be continued to be developed in the future after I finish all the features, but who knows?

### Installation

Clone the repository and do one of the following:

    $ cargo install --path=.
    $ export PATH="$HOME/.cargo/bin:$PATH"

    This option installs the binary locally and adds the folder where all rust binaries 
    are downloaded into to your PATH, allowing you to call the CLI app directly.
    Note that this is temporary and the `export` command will need to be rerun for each new shell.
    See adding Cargo's bin to your path for a permanent solution.

    $ mkdir new_repo
    $ cd new_repo
    $ guilt init

or you can install the project to a lone folder and call it from there (though i recommend doing the step above and then uninstalling it with cargo uninstall if you want to)

    
