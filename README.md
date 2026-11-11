# Arbitrage Finder
A simple tool to find sports betting arbitrage opportunities.

The tool fetches the odds from [The Odds API](https://the-odds-api.com/) and compares the odds at different bookmakers to each other in order to determine whether there are profitable and risk-free bets available.

## Installation
Start out by cloning the repository:

    git clone https://github.com/daankoning/ArbitrageFinder.git

The project is managed with [uv](https://docs.astral.sh/uv/). Install the dependencies:

    uv sync

To also show a progress bar while matches are checked, install the optional `progress` extra (which pulls in [tqdm](https://github.com/tqdm/tqdm)):

    uv sync --extra progress

Next, ensure you have an API key from The Odds API and simply run:

    uv run main.py --key <YOUR_API_KEY>

## Usage
The tool offers the following optional command line arguments.

### API key
In order to set the API key to use either the `-k` or `--key` arguments. If neither of these are used the script will read the value of `$API_KEY`. `.env` files are also supported and automatically loaded.

### Region
Different parts of the world have different bookmakers. To reflect this the `-r` or `--region` arguments allow you to set the region in which to search. Accepts the values `"eu"`,`"us"`, `"uk"`, and `"au"`. 

### Unformatted
Using the `-u` or `--unformatted` flags will remove the pretty printing and simply dump the json which contains the arbs to the console directly. Use this if you intend to extend upon the script in some way, for regular usage the formatted print is significantly better.

### Cutoff
The `-c` or `--cutoff` allow you to set a minimum profit margin. Should be set as a percentage, so `-c 10` means a 10% profit margin. Defaults to 0 (i.e. any profitable arb will be displayed).

### Help
The `-h` or `--help` flags will show a short help message with documentation.

### Settlement analysis

`--budget 100 --commission 2` allocates 100 currency units across named outcomes,
with an illustrative 2% commission on winning profit. Each outcome reports its stake
and net settlement. Equal-priced outcomes remain separate. Whole cents are allocated
by largest remainder, preserving the available budget; this is not an optimal integer
stake solver. The worst settlement can be negative after rounding or commission even
when the quoted odds pass the theoretical cutoff.

Only complete head-to-head markets with finite decimal odds greater than one are
compared. Soccer markets must include a draw, and started matches are excluded by
default. The cutoff uses the theoretical return `1 / sum(1 / odds) - 1` before costs.
`--unformatted` emits valid JSON including the settlement analysis.

These calculations assume exhaustive, mutually exclusive outcomes and matching
bookmaker settlement rules. Limits, taxes, voids, minimum stakes and execution delays
are not modeled. No wagers are placed.

Run the offline regression tests with `uv run python -m unittest discover -s tests -v`.
