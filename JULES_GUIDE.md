# Jules Integration Guide

This document serves as a comprehensive guide for interacting with **Jules**, Google's autonomous AI coding agent. It outlines the tools available for developers to orchestrate, monitor, and automate sessions natively from the terminal and code, without relying exclusively on a web browser.

There are two primary ways to interact with Jules outside of the web UI:
1. **Jules Tools (CLI):** A command-line interface for managing sessions and interacting with Jules manually from your terminal.
2. **Jules SDK:** A Node.js library for programmatically orchestrating agents, streaming events, and building automated workflows.

---

## 1. Jules Tools (CLI)

Jules Tools is a lightweight command-line interface designed to keep you in your flow state by acting as a dashboard for your coding agent.

### Installation

To install the CLI globally on your local machine, run:

```bash
npm install -g @google/jules
```

*(Alternatively, you can run it on the fly using `npx @google/jules`.)*

### Authentication

Before using the tool, you must authenticate with your Google account:

```bash
jules login
```
This command opens a browser window to complete the authentication process.

To log out:
```bash
jules logout
```

### Core Usage and Commands

The CLI is built around commands and subcommands. You can access help at any time using the `-h` or `--help` flag.

- **`jules help`**: Display general help information.
- **`jules version`**: Shows the currently installed version.
- **`jules completion <shell>`**: Generates an autocompletion script for bash, zsh, etc.

#### Global Flags
- `--theme <string>`: Sets the terminal UI theme (e.g., `dark` or `light`).

#### The `remote` Command

The `remote` command is your primary interface for interacting with cloud-based Jules sessions.

**List connected resources:**
```bash
jules remote list --repo    # List all connected repositories
jules remote list --session # List all active and past sessions
```

**Start a new session:**
```bash
# Starts a new session in the specified repo
jules remote new --repo your-org/your-repo --session "Write unit tests for the auth module"
```
*(If run from inside a repository directory, Jules can infer the context, allowing you to omit `--repo`.)*

**Pull results:**
Once a session is finished, you can fetch the code changes:
```bash
jules remote pull --session <session_id>
```

### Interactive Dashboard (TUI)

For a rich, visual experience directly in your terminal, launch the built-in Terminal User Interface:

```bash
jules
```

This launches a dashboard where you can view sessions, review side-by-side diffs, and create new tasks step-by-step.

---

## 2. Jules SDK (`@google/jules-sdk`)

The Jules SDK enables you to run a fleet of coding agents in the cloud. You can orchestrate complex, long-running coding tasks to an ephemeral cloud environment integrated with a GitHub repository.

### Installation

Install the SDK in your Node.js project:

```bash
npm install @google/jules-sdk
```

Configure your environment variables:
```bash
export JULES_API_KEY="your-api-key"
```

### Core Features and Examples

#### 1. Interactive Sessions on a Repository

Use `jules.session()` to define tasks that run against a GitHub repository. You can observe the process, wait for plans, and approve them dynamically.

```typescript
import { jules } from '@google/jules-sdk';

const session = await jules.session({
  prompt: `Fix visibility issues in the examples/nextjs app.
  - Update the global styles to a dark theme.`,
  source: { github: 'davideast/dataprompt', baseBranch: 'main' },
  autoPr: true,
});

console.log(`Session created: ${session.id}`);

// Wait for the agent to generate a plan
await session.waitFor('awaitingPlanApproval');
await session.approve();

// Retrieve the outcome
const outcome = await session.result();
if (outcome.pullRequest) {
  console.log(`PR: ${outcome.pullRequest.url}`);
}
```

#### 2. Streaming Progress and Diffs

The `.stream()` method allows you to observe the agent's real-time progress via an `AsyncIterator`.

```typescript
import { jules } from '@google/jules-sdk';

const session = jules.session('<session-id>');

for await (const activity of session.stream()) {
  if (activity.type === 'progressUpdated') {
    for (const artifact of activity.artifacts) {
      if (artifact.type === 'changeSet') {
        const parsed = artifact.parsed();
        for (const file of parsed.files) {
          console.log(`${file.path}: +${file.additions} -${file.deletions}`);
        }
      }
    }
  }
}
```

#### 3. "Repoless" Cloud Functions

Sessions can act as pre-configured serverless functions running on a Jules VM (with Node.js, Python, Rust, etc., pre-installed). You can pass large context through prompts and let the agent generate a result entirely from scratch.

```typescript
import { jules } from '@google/jules-sdk';

const session = await jules.session({
  prompt: `Summarize the following user request and formulate an answer...`,
});

const result = await session.result();
const answer = result.generatedFiles().get('answer.md');
console.log(answer?.content);
```

#### 4. Orchestrating a Fleet

You can process multiple tasks in parallel using `jules.all()`.

```typescript
const todos = ['Fix login bug', 'Update README', 'Refactor tests'];

const sessions = await jules.all(todos, (task) => ({
  prompt: task,
  source: { github: 'your-org/your-repo', baseBranch: 'main' },
}));
```

### SDK Configuration Options

If you need to set configurations explicitly (e.g., handling multiple API keys or adjusting timeouts), use the `.with()` method:

```typescript
const customJules = jules.with({
  apiKey: 'other-api-key',
  pollingIntervalMs: 2000,
  timeout: 60000,
});
```

### Resources

- **Official Web Docs:** [jules.google/docs](https://jules.google/docs/)
- **SDK GitHub Repository:** [google-labs-code/jules-sdk](https://github.com/google-labs-code/jules-sdk)
