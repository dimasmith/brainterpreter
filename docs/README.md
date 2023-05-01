# Brainterpreter documentation

The documentation here is based on [znai](https://github.com/testingisdocumenting/znai) tool.

## Local build and preview

Check [the prerequisites](https://testingisdocumenting.org/znai/znai-development/local-build/) to run `znai` locally.

Run the `znai` in live preview mode.

```shell
mvn znai:preview
```

It opens your documentation in a preview mode. 
The server port is `3333` by default.

## Deploying documentation to GitHub

Once you push the documentation, the `znai-pages-deploy` action picks it up and deploys as a project GitHub pages.
On successful build your documentation will be available on a [project pages](https://dimasmith.github.io/brainterpreter/).
