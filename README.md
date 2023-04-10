# ni-rs
`ni-rs` is rust implement of [ni](https://github.com/antfu/ni).

install it globally.
```
npm i -g ni-rs
```


`ni-rs` refers to [detect-package-manager](https://github.com/egoist/detect-package-manager)'s detect strategy.
## install
```
nir

# npm install
# yarn install
# pnpm install
```
```
ni -g eslint

# npm i -g eslint
# yarn global add eslint (Yarn 1)
# pnpm add -g eslint

# this uses default agent, regardless your current working directory
```
## run
```
nrr dev

# npm run dev
# yarn run dev
# pnpm run dev 
```

## execute
```
nxr vitest

# npx vitest
# yarn dlx vitest
# pnpm dlx vitest
```
Make sure your package manager support the above commands.