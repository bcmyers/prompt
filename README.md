# prompt

Small tool that creates a customized bash prompt with:

* Python virtualenv, i.e. `(name)`
* Standard info, i.e. `HH:MM username@hostname path`
* Git info, i.e. `branch@hash`
* Kubernetes info, i.e. `context@namespace`

## Requirements

[rust](https://www.rust-lang.org/tools/install)

## Installation

```bash
# clone the repository
git clone https://github.com/bcmyers/prompt.git

# cd into prompt directory
cd prompt

# install the prompt binary to somewhere on your system (example below installs to ~/bin)
cargo install --root ~ --path .
```

## Usage

Put the below (or something similar) in your `~/.bashrc`.

```bash
# prompt
prompt_command() {
	local exit="$?"
	if command -v prompt &>/dev/null; then
		if [[ $exit == "0" ]]; then
			PS1="$(prompt --color="bright blue")\n$ "
		else
			PS1="$(prompt --color="bright red")\n$ "
		fi
	else
		PS1="$exit \u@\H \w\n$ "
	fi
}

PROMPT_COMMAND=prompt_command
```
