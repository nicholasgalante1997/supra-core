# @supra/pico

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

This is a super lightweight (~1.5KB) client side npm package for @picocss/pico.

## Table of Contents

- [@supra/pico](#suprapico)
  - [Table of Contents](#table-of-contents)
  - [Background](#background)
    - [What is @picocss/pico?](#what-is-picocsspico)
  - [Install](#install)
    - [With NPM](#with-npm)
    - [With Yarn](#with-yarn)
    - [With Pnpm](#with-pnpm)
    - [With Bun](#with-bun)
  - [Usage](#usage)
    - [Quickstart](#quickstart)
  - [Contributing](#contributing)
  - [License](#license)

## Background

### What is @picocss/pico?

This is from the `@picocss/pico` [docs](https://github.com/picocss/pico).

> A minimalist and lightweight starter kit that prioritizes semantic syntax, making every HTML element responsive and elegant by default.
>
> Write HTML, Add Pico CSS, and Voilà!

I highly advise you to read the [docs](https://picocss.com/).  

What the team behind Pico is doing is phenomenal. Whether or not you like the visual appeal of it, it is objectively a hyper performant, modern, and responsive approach to css, with a number of other out of the box benefits. If `tailwindcss` represents the bane of the growing complexity of the modern css landscape, `@picocss/pico` stands in refreshing and competent juxtapositon.

## Install

### With NPM

```sh
npm install @supra/pico
```

### With Yarn

```sh
yarn add @supra/pico
```

### With Pnpm

```sh
pnpm add @supra/pico
```

### With Bun

```sh
bun add @supra/pico

```

## Usage

### Quickstart

```ts
import pico from '@supra/pico';

const { init, render, unmount } = pico();

/**
 * Sets up @picocss/pico in the DOM
 * */
init();

/**
 * Renders @picocss/pico theme widget in the DOM
 * */
render();

/**
 * Removes @picocss/pico theme widget from the DOM
 * Removes @picocss/pico link from the DOM
 * */
unmount();
```

## Contributing

See [the contributing file](CONTRIBUTING.md)!

PRs accepted.

Small note: If editing the Readme, please conform to the [standard-readme](https://github.com/RichardLitt/standard-readme) specification.

## License

MIT ©
