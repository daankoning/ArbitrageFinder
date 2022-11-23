# Arbitrage Finder
A simple tool to find sports betting arbitrage opportunities.

The tool fetches the odds from [The Odds API](https://the-odds-api.com/) and compares the odds at different bookmakers to each other in order to determine whether there are profitable and risk-free bets available.

## Installation
Start out by cloning the repository:

    git clone https://github.com/daankoning/ArbitrageFinder.git

Install the dependencies:

    pip install -r requirements.txt

Next, ensure you have an API key from The Odds API and simply run:

    python main.py --key <YOUR_API_KEY>

## Usage
The tool offers four optional command line arguments.

### API key
In order to set the API key to use either the `-k` or `--key` arguments. If neither of these are used the script will read the value of `$API_KEY`. `.env` files are also supported and automatically loaded.

### Region
Different parts of the world have different bookmakers. To reflect this the `-r` or `--region` arguments allow you to set the region in which to search. Accepts the values `"eu"`,`"us"`, `"uk"`, and `"au"`. 

### Unformatted
Using the `-u` or `--unformatted` will remove the pretty printing and simply dump the json which contains the arbs to the console directly. Use this if you intend to extend upon the script in some way, for regular usage the formatted print is significantly better.

### Help
The `-h` or `--help` flags will show a short help message with documentation.