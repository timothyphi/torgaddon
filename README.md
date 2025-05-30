# uswds-axum-template

A project template utilizing [Axum](https://github.com/tokio-rs/axum) as the backend application server, [Askama](https://github.com/askama-rs/askama) for HTML templating, SASS partials for the organizing stylesheets along with [USWDS](https://designsystem.digital.gov/) as a starter design system, and TypeScript with [Vite](https://vite.dev/) for the bundled, client-side, browser code.

## Developer (System) Requirements

These aren't actually hard requirements, just what's on my machine

- git version 2.39.5
- rustc 1.86.0
- nvm version 0.37.2
- node set as version v23.10.0
- npm set as version v11.2.0

## Production Requirements

- TBD

## Initial Setup for Development

### Step 1. Install Rust (or update to at least the version listed above)

Check [Rust Docs](https://doc.rust-lang.org/book/ch01-01-installation.html#installing-rustup-on-linux-or-macos).

### Step 2. Install JavaScript dependencies

```shell
npm install
```

### Step 3. Initialize USWDS

```shell
./scripts/uswds_init.sh
```

### Step 4. Configure environment file

```shell
cp ./server/sample.env ./server/.env
```

Fill out environment file for your needs.

### Step 5. (Option 1) Build Browser Code for Deployment

```shell
npm run build   # One-time builds TS -> JS (bundle) and SCSS -> CSS (bundle)
```

### Step 5. (Option 2) Run tools in development

```shell
npm run server  # Watches `server` directory, triggers rebuild on change
npm run scss    # Watches `styles` directory, triggers rebuild on change
npm run ts      # Watches `browser` directory, triggers rebuild on change
```

### Step 5. (Option 3) Run tools in development

Run all three scripts with `tmux`.

```shell
./scripts/run_tmux_env.sh
```

Kill all `tmux` scripts by killing the session.

```shell
tmux kill-session -t uswds-axum-template
```

### Completely Optional: Run accessibility check

```shell
npm run acheck -- http://localhost:8000
```

Runs the accessibility check on the URL provided and generates results.

Check [achecker npm listing](https://www.npmjs.com/package/accessibility-checker#Configuration) for more information.
