#!/bin/bash

SESSION="uswds-axum-template"

SCRIPT_PATH="$(readlink -f "$0")"
SCRIPT_DIR="$(dirname "$SCRIPT_PATH")"
PROJECT_DIR="$(readlink -f "$SCRIPT_DIR/../")"

tmux new-session -d -s "$SESSION" -c "$PROJECT_DIR"

tmux rename-window -t "$SESSION:0" 'server'
tmux send-keys -t "$SESSION:0" 'npm run server' C-m # TODO: update to npm run server:watch when implemented

tmux new-window -t "$SESSION:1" -n 'ts' -c "$PROJECT_DIR"
tmux send-keys -t "$SESSION:1" 'npm run ts' C-m

tmux new-window -t "$SESSION:2" -n 'scss' -c "$PROJECT_DIR"
tmux send-keys -t "$SESSION:2" 'npm run scss' C-m

tmux new-window -t "$SESSION:3" -n 'uswds' -c "$PROJECT_DIR"
tmux send-keys -t "$SESSION:3" 'npm run uswds' C-m

tmux new-window -t "$SESSION:4" -n 'editor' -c "$PROJECT_DIR"
tmux send-keys -t "$SESSION:4" 'npm run eslint -- .' C-m

tmux attach -t "$SESSION"
