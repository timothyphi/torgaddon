# uswds-axum-template

A project template utilizing [Axum](https://github.com/tokio-rs/axum) as the backend application server, [Askama](https://github.com/askama-rs/askama) for HTML templating, SASS partials for the organizing stylesheets along with [USWDS](https://designsystem.digital.gov/) as a starter design system, and TypeScript with [Vite](https://vite.dev/) for the bundled, client-side, browser code.

## Development Requirements

These aren't actually hard requirements, just what's on my machine

- git v2.39.5
- rustc v1.86.0
- node v23.10.0
- npm v11.2.0

## Production Requirements

- TBD

## Development Setup

### Step 1. Install Rust (or update to at least the version listed above)

Check [Rust Docs](https://doc.rust-lang.org/book/ch01-01-installation.html#installing-rustup-on-linux-or-macos).

### Step 2. Build Axum backend

```shell
cd server && cargo build && cd ..
```

### Step 3. Install JavaScript dependencies

```shell
npm install
```

### Step 4. Initialize USWDS

```shell
./scripts/uswds_init.sh
```

### Step 5. Configure environment file

```shell
cp ./server/sample.env ./server/.env
```

Fill out environment file for your needs.

### Step 6. Build browser code

```shell
npm run build
```

### Step 7. Start Axum server

```shell
npm run server
```

## Development tools

### Option 1. Run dev tools individually

Watches `server` directory, triggers rebuild on change.

```shell
npm run server:watch
```

Watches `styles/main.scss` entry point, triggers rebuild on change in `styles` directory.

```shell
npm run scss
```

Watches `styles/vendors/uswds/styles.scss` entry point, triggers rebuild on change in `styles/vendors/uswds` directory.

```shell
npm run uswds
```

Watches `browser` directory, triggers rebuild on change.

```shell
npm run ts
```

### Option 2. Run dev tools concurrently using tmux

You must have `tmux` installed if you want to use this option.

```shell
./scripts/run_tmux_env.sh
```

Kill all `tmux` scripts by killing the session.

```shell
tmux kill-session -t uswds-axum-template
```

### Optional: Run accessibility check

```shell
npm run acheck -- http://localhost:8000
```

Runs the accessibility check on the URL provided and generates results.

Check [achecker npm listing](https://www.npmjs.com/package/accessibility-checker#Configuration) for more information.

## Deploying to production

It's recommended to build the server binary ([step 2](#step-2-build-axum-backend)) and browser assets ([step 3](#step-3-install-javascript-dependencies), [step 4](#step-4-initialize-uswds), and [step 6](#step-6-build-browser-code)) on a seperate machine.

1. Move those assets in `public` to the production machine

1. Configure the environment file ([step 5](#step-5-configure-environment-file))

1. Run the Axum server ([step 7](#step-7-start-axum-server))

## Supplemental Documentation

- [axum](https://docs.rs/axum/latest/axum/)
- [uswds](https://designsystem.digital.gov/)
- [askama](https://docs.rs/askama/latest/askama/)
- [sass](https://sass-lang.com/)
- [vite](https://vite.dev/)
